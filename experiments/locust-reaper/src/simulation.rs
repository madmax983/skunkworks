use ::rand::prelude::*;
use macroquad::prelude::*;
use rayon::prelude::*;

pub const AGENT_COUNT: usize = 20_000;
pub const WORLD_SIZE: f32 = 1000.0;
pub const SPEED: f32 = 2.0;
pub const PHEROMONE_DECAY: f32 = 0.90;
pub const GRID_SCALE: usize = 4;

#[derive(Clone, Copy)]
pub struct Locust {
    pub pos: Vec2,
    pub vel: Vec2,
    pub state: u8, // 0: Normal, 1: Dead (leaves trace)
}

#[derive(Clone, Copy)]
pub struct GarbageNode {
    pub pos: Vec2,
    pub radius: f32,
    pub health: f32, // Health of the GC node, decreases as it's being 'reaped'
}

pub struct World {
    pub agents: Vec<Locust>,
    pub pheromones: Vec<f32>,
    pub garbage: Vec<GarbageNode>,
    pub grid_w: usize,
    pub target: Vec2,
}

impl World {
    pub fn new() -> Self {
        let mut rng = ::rand::thread_rng();
        let mut agents = Vec::with_capacity(AGENT_COUNT);

        // Spawn agents on the edges
        for _ in 0..AGENT_COUNT {
            let side = rng.gen_range(0..4);
            let (x, y) = match side {
                0 => (rng.gen_range(0.0..WORLD_SIZE), 0.0),
                1 => (rng.gen_range(0.0..WORLD_SIZE), WORLD_SIZE),
                2 => (0.0, rng.gen_range(0.0..WORLD_SIZE)),
                _ => (WORLD_SIZE, rng.gen_range(0.0..WORLD_SIZE)),
            };

            agents.push(Locust {
                pos: vec2(x, y),
                vel: vec2(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize_or_zero(),
                state: 0,
            });
        }

        let grid_w = (WORLD_SIZE / GRID_SCALE as f32) as usize;
        let grid_h = (WORLD_SIZE / GRID_SCALE as f32) as usize;

        Self {
            agents,
            pheromones: vec![0.0; grid_w * grid_h],
            garbage: vec![],
            grid_w,
            target: vec2(WORLD_SIZE / 2.0, WORLD_SIZE / 2.0),
        }
    }

    pub fn add_garbage(&mut self, pos: Vec2, radius: f32) {
        self.garbage.push(GarbageNode { pos, radius, health: 100.0 });
    }

    pub fn clear_garbage(&mut self) {
        self.garbage.clear();
    }

    fn grid_idx(&self, pos: Vec2) -> Option<usize> {
        if pos.x < 0.0 || pos.x >= WORLD_SIZE || pos.y < 0.0 || pos.y >= WORLD_SIZE {
            return None;
        }
        let gx = (pos.x / GRID_SCALE as f32) as usize;
        let gy = (pos.y / GRID_SCALE as f32) as usize;
        Some(gy * self.grid_w + gx)
    }

    pub fn update(&mut self) {
        // Decay pheromones
        for p in self.pheromones.iter_mut() {
            *p *= PHEROMONE_DECAY;
        }

        let target = self.target;
        let pheromones = &self.pheromones;
        let grid_w = self.grid_w;
        let garbage = &self.garbage;

        // Update agents
        let updates: Vec<(usize, Locust)> = self.agents.par_iter().enumerate().filter_map(|(i, agent)| {
            if agent.state == 1 {
                return None;
            }

            let mut vel = agent.vel;

            // Find the closest garbage node
            let mut closest_garbage: Option<&GarbageNode> = None;
            let mut min_dist = f32::MAX;
            for node in garbage.iter() {
                let dist = agent.pos.distance(node.pos);
                if dist < min_dist {
                    min_dist = dist;
                    closest_garbage = Some(node);
                }
            }

            // Goal Seeking
            let current_target = match closest_garbage {
                Some(node) => node.pos,
                None => target,
            };

            let dir_to_target = (current_target - agent.pos).normalize_or_zero();
            vel = (vel * 0.95 + dir_to_target * 0.05).normalize_or_zero();

            // Pheromone Avoidance
            let mut avoid = Vec2::ZERO;
            let check_dist = 10.0;
            let offsets = [
                vec2(check_dist, 0.0),
                vec2(-check_dist, 0.0),
                vec2(0.0, check_dist),
                vec2(0.0, -check_dist),
                vec2(check_dist, check_dist),
                vec2(-check_dist, -check_dist),
            ];

            for offset in offsets {
                let p = agent.pos + offset;
                if let Some(idx) = grid_idx_static(p, grid_w) {
                    let level = pheromones[idx];
                    if level > 0.1 {
                        avoid += (agent.pos - p).normalize_or_zero() * level;
                    }
                }
            }

            if avoid.length_squared() > 0.0 {
                vel = (vel + avoid.normalize_or_zero() * 0.5).normalize_or_zero();
            }

            // Move
            let pos = agent.pos + vel * SPEED;
            let mut state = agent.state;

            // Bounds check
            if pos.x < 0.0 || pos.x >= WORLD_SIZE || pos.y < 0.0 || pos.y >= WORLD_SIZE {
                state = 1;
            } else if let Some(node) = closest_garbage {
                if pos.distance(node.pos) < node.radius {
                    state = 1; // Dies but "eats" it
                }
            }

            Some((i, Locust { pos, vel, state }))
        }).collect();

        // Apply updates
        let mut respawns = 0;
        let mut rng = ::rand::thread_rng();

        for (i, new_agent) in updates {
            self.agents[i] = new_agent;
            if new_agent.state == 1 {
                // Deposit pheromone
                if let Some(idx) = self.grid_idx(new_agent.pos) {
                    self.pheromones[idx] = (self.pheromones[idx] + 1.0).min(5.0);
                }

                // Damage garbage node if hit
                for node in self.garbage.iter_mut() {
                    if new_agent.pos.distance(node.pos) < node.radius {
                        node.health -= 1.0;
                    }
                }

                respawns += 1;
            }
        }

        self.garbage.retain(|n| n.health > 0.0);

        // Respawn dead agents
        for _ in 0..respawns {
            let side = rng.gen_range(0..4);
            let (x, y) = match side {
                0 => (rng.gen_range(0.0..WORLD_SIZE), 0.0),
                1 => (rng.gen_range(0.0..WORLD_SIZE), WORLD_SIZE),
                2 => (0.0, rng.gen_range(0.0..WORLD_SIZE)),
                _ => (WORLD_SIZE, rng.gen_range(0.0..WORLD_SIZE)),
            };
            if let Some(agent) = self.agents.iter_mut().find(|a| a.state == 1) {
                agent.pos = vec2(x, y);
                agent.vel = vec2(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize_or_zero();
                agent.state = 0;
            }
        }
    }

    pub fn draw(&self) {
        let sw = screen_width();
        let sh = screen_height();
        let scale_x = sw / WORLD_SIZE;
        let scale_y = sh / WORLD_SIZE;

        // Draw pheromones (optional, skipping for performance/visual clarity)
        // Draw garbage nodes (gray for dead matter)
        for node in &self.garbage {
            draw_circle(
                node.pos.x * scale_x,
                node.pos.y * scale_y,
                node.radius * scale_x,
                Color::new(0.4, 0.4, 0.4, node.health / 100.0),
            );
        }

        // Draw Agents
        for agent in &self.agents {
            if agent.state == 0 {
                draw_circle(
                    agent.pos.x * scale_x,
                    agent.pos.y * scale_y,
                    1.5, // pixel radius
                    GREEN,
                );
            }
        }

        // Draw Server Root
        draw_circle(
            self.target.x * scale_x,
            self.target.y * scale_y,
            30.0 * scale_x,
            BLUE,
        );
    }
}

fn grid_idx_static(pos: Vec2, grid_w: usize) -> Option<usize> {
    if pos.x < 0.0 || pos.x >= WORLD_SIZE || pos.y < 0.0 || pos.y >= WORLD_SIZE {
        return None;
    }
    let gx = (pos.x / GRID_SCALE as f32) as usize;
    let gy = (pos.y / GRID_SCALE as f32) as usize;
    Some(gy * grid_w + gx)
}
