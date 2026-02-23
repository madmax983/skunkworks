use crate::fluid::FluidSim;
use macroquad::prelude::*;

pub struct Agent {
    pub id: usize,
    pub pos: Vec2,
    pub vel: Vec2,
    pub max_speed: f32,
    pub max_force: f32,
}

pub struct Lock {
    pub id: u8,
    pub pos: IVec2,
    pub contention_level: usize,
}

pub struct FlockManager {
    pub agents: Vec<Agent>,
    pub locks: Vec<Lock>,
}

impl FlockManager {
    pub fn new() -> Self {
        Self {
            agents: Vec::new(),
            locks: Vec::new(),
        }
    }

    pub fn add_lock(&mut self, id: u8, x: usize, y: usize) {
        self.locks.push(Lock {
            id,
            pos: IVec2::new(x as i32, y as i32),
            contention_level: 0,
        });
    }

    pub fn spawn_agent(&mut self, x: f32, y: f32) {
        let id = self.agents.len();
        let angle = rand::gen_range(0.0f32, std::f32::consts::TAU);
        self.agents.push(Agent {
            id,
            pos: vec2(x, y),
            vel: vec2(angle.cos(), angle.sin()) * 50.0,
            max_speed: rand::gen_range(80.0, 120.0),
            max_force: rand::gen_range(200.0, 400.0),
        });
    }

    pub fn update(&mut self, fluid: &mut FluidSim, dt: f32) {
        let width = fluid.width as f32 * 8.0;
        let height = fluid.height as f32 * 8.0;

        // Reset lock contention
        for lock in &mut self.locks {
            lock.contention_level = 0;
        }

        // We need a snapshot of positions/velocities for flocking to avoid order-dependence issues
        // behaving weirdly, but for performance in this simple sim, direct update is okay
        // providing we don't mutate while reading other agents.
        // But Rust won't let us iterate mutably and immutably at same time.
        // So we clone the agent data needed for flocking.
        let agent_data: Vec<(Vec2, Vec2)> = self.agents.iter().map(|a| (a.pos, a.vel)).collect();

        for (i, agent) in self.agents.iter_mut().enumerate() {
            let mut acc = vec2(0.0, 0.0);

            // 1. Flocking Behaviors
            let (sep, ali, coh) = compute_flocking(i, agent.pos, agent.vel, &agent_data);
            acc += sep * 1.5;
            acc += ali * 1.0;
            acc += coh * 1.0;

            // 2. Thermal Taxis (Attraction to Heat/Vents)
            // Instead of checking all vents, let's just check the fluid temperature gradient?
            // Or just attract to nearest vent if it's hot.
            // Let's check nearest vent.
            let mut closest_vent = None;
            let mut min_dist = f32::MAX;
            for lock in &self.locks {
                let lock_pos = vec2(lock.pos.x as f32 * 8.0 + 4.0, lock.pos.y as f32 * 8.0 + 4.0);
                let d = agent.pos.distance(lock_pos);
                if d < min_dist {
                    min_dist = d;
                    closest_vent = Some(lock_pos);
                }

                // If very close, increment contention
                if d < 20.0 {
                    // We need to update lock contention, but we can't mutate locks here easily
                    // because we are iterating agents.
                    // We'll do a separate pass or use the fact that we can't modify locks here.
                    // Let's just create heat directly here.
                    let cx = (lock_pos.x / 8.0) as usize;
                    let cy = (lock_pos.y / 8.0) as usize;
                    // Add heat based on proximity (Contention)
                    fluid.add_heat(cx, cy, 100.0 * dt);
                }
            }

            if let Some(target) = closest_vent {
                // Seek target
                let desired = (target - agent.pos).normalize_or_zero() * agent.max_speed;
                let steer = (desired - agent.vel).clamp_length_max(agent.max_force);
                acc += steer * 0.8; // Attraction weight
            }

            // 3. Hydrodynamic Drag / Lift
            // Heat rises. Agents in hot water should be pushed up.
            let cx = (agent.pos.x / 8.0).clamp(0.0, fluid.width as f32 - 1.0) as usize;
            let cy = (agent.pos.y / 8.0).clamp(0.0, fluid.height as f32 - 1.0) as usize;
            let temp = fluid.get_temp(cx, cy);

            // Buoyancy force
            if temp > 0.1 {
                acc += vec2(0.0, -1.0) * temp * 200.0;
            }
            // Drag (resistance)
            acc -= agent.vel * 0.5 * dt; // Damping

            // Apply Physics
            agent.vel += acc * dt;
            agent.vel = agent.vel.clamp_length_max(agent.max_speed);
            agent.pos += agent.vel * dt;

            // Boundaries (Wrap)
            if agent.pos.x < 0.0 {
                agent.pos.x += width;
            }
            if agent.pos.x > width {
                agent.pos.x -= width;
            }
            if agent.pos.y < 0.0 {
                agent.pos.y += height;
            }
            if agent.pos.y > height {
                agent.pos.y -= height;
            }
        }
    }
}

fn compute_flocking(
    my_idx: usize,
    my_pos: Vec2,
    my_vel: Vec2,
    others: &[(Vec2, Vec2)],
) -> (Vec2, Vec2, Vec2) {
    let mut sep = vec2(0.0, 0.0);
    let mut ali = vec2(0.0, 0.0);
    let mut coh = vec2(0.0, 0.0);
    let mut count = 0;

    let view_dist = 40.0;
    let sep_dist = 20.0;

    for (i, (pos, vel)) in others.iter().enumerate() {
        if i == my_idx {
            continue;
        }

        let d = my_pos.distance(*pos);
        if d > 0.0 && d < view_dist {
            // Alignment
            ali += *vel;
            // Cohesion
            coh += *pos;

            // Separation
            if d < sep_dist {
                let diff = (my_pos - *pos).normalize_or_zero() / d;
                sep += diff;
            }
            count += 1;
        }
    }

    if count > 0 {
        ali = (ali / count as f32).normalize_or_zero() * 100.0; // Max speed approx
        ali = (ali - my_vel).clamp_length_max(400.0); // Max force

        coh = (coh / count as f32 - my_pos).normalize_or_zero() * 100.0;
        coh = (coh - my_vel).clamp_length_max(400.0);

        sep = sep.normalize_or_zero() * 100.0;
        sep = (sep - my_vel).clamp_length_max(400.0);
    }

    (sep, ali, coh)
}
