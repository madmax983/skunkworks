use crate::fluid::FluidSim;
use macroquad::prelude::*;

pub struct Agent {
    pub id: usize,
    pub pos: Vec2,
    pub vel: Vec2,
    pub max_speed: f32,
    pub max_force: f32,
}

pub struct FlockManager {
    pub agents: Vec<Agent>,
}

impl FlockManager {
    pub fn new() -> Self {
        Self { agents: Vec::new() }
    }

    pub fn spawn_agent(&mut self, x: f32, y: f32) {
        let id = self.agents.len();
        let angle = rand::gen_range(0.0f32, std::f32::consts::TAU);
        self.agents.push(Agent {
            id,
            pos: vec2(x, y),
            vel: vec2(angle.cos(), angle.sin()) * 50.0,
            max_speed: rand::gen_range(60.0, 100.0),
            max_force: rand::gen_range(100.0, 200.0),
        });
    }

    pub fn update(&mut self, fluid: &mut FluidSim, dt: f32) {
        let width = fluid.width as f32 * 8.0;
        let height = fluid.height as f32 * 8.0;

        let agent_positions: Vec<Vec2> = self.agents.iter().map(|a| a.pos).collect();
        let agent_velocities: Vec<Vec2> = self.agents.iter().map(|a| a.vel).collect();

        for (i, agent) in self.agents.iter_mut().enumerate() {
            let mut acc = vec2(0.0, 0.0);

            // 1. Flocking
            let (sep, ali, coh) =
                compute_flocking(i, agent.pos, agent.vel, &agent_positions, &agent_velocities);
            acc += sep * 2.0;
            acc += ali * 1.0;
            acc += coh * 1.0;

            // 2. Chemical Interaction
            let cx = (agent.pos.x / 8.0).clamp(0.0, fluid.width as f32 - 1.0) as usize;
            let cy = (agent.pos.y / 8.0).clamp(0.0, fluid.height as f32 - 1.0) as usize;

            // Emit Chemical B (Activator)
            fluid.add_chem_b(cx, cy, 5.0 * dt); // Stronger emission

            // Sense Gradient of B
            let angle = agent.vel.y.atan2(agent.vel.x);
            let sensor_l_pos = agent.pos + vec2((angle + 0.5).cos(), (angle + 0.5).sin()) * 15.0;
            let sensor_r_pos = agent.pos + vec2((angle - 0.5).cos(), (angle - 0.5).sin()) * 15.0;

            let val_l = get_chem_at(fluid, sensor_l_pos);
            let val_r = get_chem_at(fluid, sensor_r_pos);

            let turn_strength = (val_l - val_r) * 20.0;
            let left_perp = vec2(-agent.vel.y, agent.vel.x).normalize_or_zero();
            acc += left_perp * turn_strength * agent.max_force;

            // 3. Physics
            agent.vel += acc * dt;
            if agent.vel.length() > agent.max_speed {
                agent.vel = agent.vel.normalize() * agent.max_speed;
            }
            agent.pos += agent.vel * dt;

            // Wrap
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

fn get_chem_at(fluid: &FluidSim, pos: Vec2) -> f32 {
    let cx = (pos.x / 8.0).clamp(0.0, fluid.width as f32 - 1.0) as usize;
    let cy = (pos.y / 8.0).clamp(0.0, fluid.height as f32 - 1.0) as usize;
    fluid.get_chem_b(cx, cy)
}

fn compute_flocking(
    my_idx: usize,
    my_pos: Vec2,
    my_vel: Vec2,
    positions: &[Vec2],
    velocities: &[Vec2],
) -> (Vec2, Vec2, Vec2) {
    let mut sep = vec2(0.0, 0.0);
    let mut ali = vec2(0.0, 0.0);
    let mut coh = vec2(0.0, 0.0);
    let mut count = 0;

    let view_dist = 40.0;
    let sep_dist = 15.0;

    for i in 0..positions.len() {
        if i == my_idx {
            continue;
        }

        let pos = positions[i];
        let vel = velocities[i];
        let d = my_pos.distance(pos);

        if d > 0.0 && d < view_dist {
            ali += vel;
            coh += pos;
            if d < sep_dist {
                let diff = (my_pos - pos).normalize_or_zero() / d;
                sep += diff;
            }
            count += 1;
        }
    }

    if count > 0 {
        ali = (ali / count as f32).normalize_or_zero() * 100.0;
        ali = (ali - my_vel).clamp_length_max(200.0);

        coh = (coh / count as f32 - my_pos).normalize_or_zero() * 100.0;
        coh = (coh - my_vel).clamp_length_max(200.0);

        sep = sep.normalize_or_zero() * 100.0;
        sep = (sep - my_vel).clamp_length_max(300.0);
    }

    (sep, ali, coh)
}
