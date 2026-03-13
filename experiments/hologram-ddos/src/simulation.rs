use rand::prelude::*;
use rayon::prelude::*;

pub const AGENT_COUNT: usize = 10_000;
pub const WORLD_SIZE: f64 = 1000.0;
pub const SPEED: f64 = 2.0;
pub const PHEROMONE_DECAY: f64 = 0.90;
pub const GRID_SCALE: usize = 4; // 1000 / 4 = 250x250 grid

#[derive(Clone, Copy)]
pub struct Locust {
    pub pos: (f64, f64),
    pub vel: (f64, f64),
    pub state: u8, // 0: Normal, 1: Dead (leaves trace)
}

pub struct World {
    pub agents: Vec<Locust>,
    pub pheromones: Vec<f64>,        // Grid of danger levels
    pub firewalls: Vec<((f64, f64), f64)>, // (Center, Radius)
    pub grid_w: usize,
    pub grid_h: usize,
    pub target: (f64, f64),
    pub server_health: f64,
    pub max_health: f64,
}

impl World {
    pub fn new() -> Self {
        let mut rng = ::rand::thread_rng();
        let mut agents = Vec::with_capacity(AGENT_COUNT);

        // Spawn agents on the edges
        for _ in 0..AGENT_COUNT {
            let side = rng.gen_range(0..4);
            let (x, y) = match side {
                0 => (rng.gen_range(0.0..WORLD_SIZE), 0.0),        // Top
                1 => (rng.gen_range(0.0..WORLD_SIZE), WORLD_SIZE), // Bottom
                2 => (0.0, rng.gen_range(0.0..WORLD_SIZE)),        // Left
                _ => (WORLD_SIZE, rng.gen_range(0.0..WORLD_SIZE)), // Right
            };

            agents.push(Locust {
                pos: (x, y),
                vel: (0.0, 0.0),
                state: 0,
            });
        }

        let grid_w = (WORLD_SIZE as usize) / GRID_SCALE;
        let grid_h = (WORLD_SIZE as usize) / GRID_SCALE;

        World {
            agents,
            pheromones: vec![0.0; grid_w * grid_h],
            firewalls: Vec::new(),
            grid_w,
            grid_h,
            target: (WORLD_SIZE / 2.0, WORLD_SIZE / 2.0),
            server_health: 1000.0,
            max_health: 1000.0,
        }
    }

    pub fn update(&mut self) {
        let target = self.target;
        let firewalls = &self.firewalls;
        let grid_w = self.grid_w;
        let grid_h = self.grid_h;
        let scale = GRID_SCALE as f64;

        // We need a read-only view of pheromones for decision making
        let pheromones = &self.pheromones;

        // Parallel update of agents
        let updates: Vec<((f64, f64), (f64, f64), u8, Option<(usize, usize)>, f64)> = self
            .agents
            .par_iter()
            .map(|agent| {
                if agent.state == 1 {
                    // Dead agents don't move, but might respawn or stay dead
                    let mut rng = ::rand::thread_rng();
                    if rng.gen_bool(0.01) {
                        let side = rng.gen_range(0..4);
                        let (x, y) = match side {
                            0 => (rng.gen_range(0.0..WORLD_SIZE), 0.0),
                            1 => (rng.gen_range(0.0..WORLD_SIZE), WORLD_SIZE),
                            2 => (0.0, rng.gen_range(0.0..WORLD_SIZE)),
                            _ => (WORLD_SIZE, rng.gen_range(0.0..WORLD_SIZE)),
                        };
                        return ((x, y), (0.0, 0.0), 0, None, 0.0);
                    }
                    return (agent.pos, agent.vel, 1, None, 0.0);
                }

                // Seek Target
                let to_target_x = target.0 - agent.pos.0;
                let to_target_y = target.1 - agent.pos.1;
                let dist_target = (to_target_x * to_target_x + to_target_y * to_target_y).sqrt();

                // Hit Target?
                if dist_target < 15.0 {
                    // Respawn
                    let mut rng = ::rand::thread_rng();
                    let side = rng.gen_range(0..4);
                    let (x, y) = match side {
                        0 => (rng.gen_range(0.0..WORLD_SIZE), 0.0),
                        1 => (rng.gen_range(0.0..WORLD_SIZE), WORLD_SIZE),
                        2 => (0.0, rng.gen_range(0.0..WORLD_SIZE)),
                        _ => (WORLD_SIZE, rng.gen_range(0.0..WORLD_SIZE)),
                    };
                    return ((x, y), (0.0, 0.0), 0, None, 1.0);
                }

                let mut desire_x = 0.0;
                let mut desire_y = 0.0;
                if dist_target > 0.0 {
                    desire_x = (to_target_x / dist_target) * SPEED;
                    desire_y = (to_target_y / dist_target) * SPEED;
                }

                // Avoid Pheromones (Danger)
                // Check ahead
                let speed_len = (agent.vel.0 * agent.vel.0 + agent.vel.1 * agent.vel.1).sqrt();
                let dir_x = if speed_len > 0.0 { agent.vel.0 / speed_len } else { 0.0 };
                let dir_y = if speed_len > 0.0 { agent.vel.1 / speed_len } else { 0.0 };

                let look_ahead_x = agent.pos.0 + dir_x * 10.0;
                let look_ahead_y = agent.pos.1 + dir_y * 10.0;

                let gx = (look_ahead_x / scale).clamp(0.0, (grid_w - 1) as f64) as usize;
                let gy = (look_ahead_y / scale).clamp(0.0, (grid_h - 1) as f64) as usize;
                let idx = gy * grid_w + gx;

                if pheromones[idx] > 0.1 {
                    // Danger ahead! Steer away randomly or perpendicular
                    let mut rng = ::rand::thread_rng();
                    let angle = rng.gen_range(-std::f64::consts::PI..std::f64::consts::PI);
                    desire_x += angle.cos() * SPEED * 2.0;
                    desire_y += angle.sin() * SPEED * 2.0;
                }

                // Apply steering
                let mut steer_x = desire_x - agent.vel.0;
                let mut steer_y = desire_y - agent.vel.1;
                let steer_len = (steer_x * steer_x + steer_y * steer_y).sqrt();
                if steer_len > 0.5 {
                    steer_x = (steer_x / steer_len) * 0.5;
                    steer_y = (steer_y / steer_len) * 0.5;
                }

                let mut new_vel_x = agent.vel.0 + steer_x;
                let mut new_vel_y = agent.vel.1 + steer_y;
                let new_vel_len = (new_vel_x * new_vel_x + new_vel_y * new_vel_y).sqrt();
                if new_vel_len > SPEED {
                    new_vel_x = (new_vel_x / new_vel_len) * SPEED;
                    new_vel_y = (new_vel_y / new_vel_len) * SPEED;
                }

                let mut new_pos_x = agent.pos.0 + new_vel_x;
                let mut new_pos_y = agent.pos.1 + new_vel_y;

                // Wall collisions (Firewalls)
                let mut state = 0;
                let mut drop_pheromone = None;

                for (center, radius) in firewalls {
                    let dx = new_pos_x - center.0;
                    let dy = new_pos_y - center.1;
                    if (dx * dx + dy * dy).sqrt() < *radius {
                        state = 1; // Die
                        // Drop pheromone at current grid
                        let px = (new_pos_x / scale).clamp(0.0, (grid_w - 1) as f64) as usize;
                        let py = (new_pos_y / scale).clamp(0.0, (grid_h - 1) as f64) as usize;
                        drop_pheromone = Some((px, py));
                        break;
                    }
                }

                // Screen Bounds
                if new_pos_x < 0.0 { new_pos_x = 0.0; }
                if new_pos_x > WORLD_SIZE { new_pos_x = WORLD_SIZE; }
                if new_pos_y < 0.0 { new_pos_y = 0.0; }
                if new_pos_y > WORLD_SIZE { new_pos_y = WORLD_SIZE; }

                ((new_pos_x, new_pos_y), (new_vel_x, new_vel_y), state, drop_pheromone, 0.0)
            })
            .collect();

        // Apply updates
        let mut total_damage = 0.0;
        for (i, (pos, vel, state, pheromone, damage)) in updates.into_iter().enumerate() {
            self.agents[i].pos = pos;
            self.agents[i].vel = vel;
            self.agents[i].state = state;
            total_damage += damage;

            if let Some((px, py)) = pheromone {
                let idx = py * self.grid_w + px;
                self.pheromones[idx] = (self.pheromones[idx] + 2.0).min(10.0);
            }
        }

        self.server_health = (self.server_health - total_damage).max(0.0);

        // Diffusion (Box Blur)
        let w = self.grid_w;
        let h = self.grid_h;
        let prev_pheromones = self.pheromones.clone(); // Read from this

        self.pheromones
            .par_chunks_mut(w)
            .enumerate()
            .for_each(|(y, row)| {
                for (x, cell) in row.iter_mut().enumerate() {
                    if y == 0 || y == h - 1 || x == 0 || x == w - 1 {
                        continue; // Skip edges for simplicity
                    }

                    // Average of 3x3 kernel
                    let mut sum = 0.0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let idx =
                                ((y as isize + dy) as usize) * w + ((x as isize + dx) as usize);
                            sum += prev_pheromones[idx];
                        }
                    }
                    *cell = sum / 9.0;
                }
            });

        // Decay pheromones
        self.pheromones.par_iter_mut().for_each(|p| {
            *p *= PHEROMONE_DECAY;
            if *p < 0.01 {
                *p = 0.0;
            }
        });
    }

    pub fn add_firewall(&mut self, pos: (f64, f64), radius: f64) {
        self.firewalls.push((pos, radius));
    }

    pub fn clear_firewalls(&mut self) {
        self.firewalls.clear();
    }
}
