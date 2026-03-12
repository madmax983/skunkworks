use macroquad::prelude::*;
use rayon::prelude::*;
use std::f32::consts::PI;
use ::rand::Rng;

pub const WORLD_SIZE: f32 = 1000.0;
pub const GRID_SCALE: usize = 5;
pub const AGENT_COUNT: usize = 3000;
pub const SPEED: f32 = 2.0;
pub const PHEROMONE_DECAY: f32 = 0.95; // Decay rate for trails

#[derive(Clone, Copy)]
pub struct Agent {
    pub pos: Vec2,
    pub angle: f32,
    pub state: u8, // 0 = alive, 1 = dead/blocked
}

impl Agent {
    pub fn new(pos: Vec2, angle: f32) -> Self {
        Self { pos, angle, state: 0 }
    }

    pub fn sense(&self, world: &World, angle_offset: f32, sensor_dist: f32) -> f32 {
        let sensor_angle = self.angle + angle_offset;
        let sensor_pos = self.pos + vec2(sensor_angle.cos(), sensor_angle.sin()) * sensor_dist;

        let scale = GRID_SCALE as f32;
        let sx = (sensor_pos.x / scale).clamp(0.0, (world.grid_w - 1) as f32) as usize;
        let sy = (sensor_pos.y / scale).clamp(0.0, (world.grid_h - 1) as f32) as usize;

        let trail_strength = world.pheromones[sy * world.grid_w + sx];

        // Gradient towards target (server)
        let to_target = world.target - sensor_pos;
        let dist = to_target.length().max(1.0);

        // Simple attraction gradient (1 / distance)
        let gradient_strength = 500.0 / dist;

        trail_strength + gradient_strength
    }
}

pub struct World {
    pub agents: Vec<Agent>,
    pub pheromones: Vec<f32>,
    pub firewalls: Vec<(Vec2, f32)>,
    pub target: Vec2,
    pub server_health: f32,
    pub max_health: f32,
    pub grid_w: usize,
    pub grid_h: usize,
}

impl World {
    pub fn new() -> Self {
        let mut rng = ::rand::thread_rng();
        let mut agents = Vec::with_capacity(AGENT_COUNT);

        // Spawn on edges
        for _ in 0..AGENT_COUNT {
            let side = rng.gen_range(0..4);
            let (x, y) = match side {
                0 => (rng.gen_range(0.0..WORLD_SIZE), 0.0),
                1 => (rng.gen_range(0.0..WORLD_SIZE), WORLD_SIZE),
                2 => (0.0, rng.gen_range(0.0..WORLD_SIZE)),
                _ => (WORLD_SIZE, rng.gen_range(0.0..WORLD_SIZE)),
            };

            // Point generally towards center
            let pos = vec2(x, y);
            let center = vec2(WORLD_SIZE / 2.0, WORLD_SIZE / 2.0);
            let dir = center - pos;
            let angle = dir.y.atan2(dir.x) + rng.gen_range(-PI/4.0..PI/4.0);

            agents.push(Agent::new(pos, angle));
        }

        let grid_w = (WORLD_SIZE / GRID_SCALE as f32).ceil() as usize;
        let grid_h = (WORLD_SIZE / GRID_SCALE as f32).ceil() as usize;

        Self {
            agents,
            pheromones: vec![0.0; grid_w * grid_h],
            firewalls: Vec::new(),
            grid_w,
            grid_h,
            target: vec2(WORLD_SIZE / 2.0, WORLD_SIZE / 2.0),
            server_health: 1000.0,
            max_health: 1000.0,
        }
    }

    pub fn update(&mut self) {
        let target = self.target;
        let firewalls = &self.firewalls;
        let scale = GRID_SCALE as f32;
        let w = self.grid_w;
        let h = self.grid_h;

        let updates: Vec<(Vec2, f32, u8, Option<(usize, usize)>, f32)> = self.agents.par_iter().map(|agent| {
            if agent.state == 1 {
                // Dead: maybe respawn
                let mut rng = ::rand::thread_rng();
                if rng.gen_bool(0.02) {
                    let side = rng.gen_range(0..4);
                    let (x, y) = match side {
                        0 => (rng.gen_range(0.0..WORLD_SIZE), 0.0),
                        1 => (rng.gen_range(0.0..WORLD_SIZE), WORLD_SIZE),
                        2 => (0.0, rng.gen_range(0.0..WORLD_SIZE)),
                        _ => (WORLD_SIZE, rng.gen_range(0.0..WORLD_SIZE)),
                    };
                    let pos = vec2(x, y);
                    let center = vec2(WORLD_SIZE / 2.0, WORLD_SIZE / 2.0);
                    let dir = center - pos;
                    let angle = dir.y.atan2(dir.x) + rng.gen_range(-PI/4.0..PI/4.0);
                    return (pos, angle, 0, None, 0.0);
                }
                return (agent.pos, agent.angle, 1, None, 0.0);
            }

            // Hit target?
            let dist_target = (target - agent.pos).length();
            if dist_target < 20.0 {
                let mut rng = ::rand::thread_rng();
                let side = rng.gen_range(0..4);
                let (x, y) = match side {
                    0 => (rng.gen_range(0.0..WORLD_SIZE), 0.0),
                    1 => (rng.gen_range(0.0..WORLD_SIZE), WORLD_SIZE),
                    2 => (0.0, rng.gen_range(0.0..WORLD_SIZE)),
                    _ => (WORLD_SIZE, rng.gen_range(0.0..WORLD_SIZE)),
                };
                let pos = vec2(x, y);
                let center = vec2(WORLD_SIZE / 2.0, WORLD_SIZE / 2.0);
                let dir = center - pos;
                let angle = dir.y.atan2(dir.x) + rng.gen_range(-PI/4.0..PI/4.0);
                return (pos, angle, 0, None, 1.0); // 1.0 damage
            }

            // Sensing
            let sensor_angle = PI / 4.0;
            let sensor_dist = 15.0;
            let turn_angle = PI / 8.0;

            let left = agent.sense(self, -sensor_angle, sensor_dist);
            let center_sense = agent.sense(self, 0.0, sensor_dist);
            let right = agent.sense(self, sensor_angle, sensor_dist);

            let mut rng = ::rand::thread_rng();
            let mut next_angle = agent.angle;

            if center_sense > left && center_sense > right {
                // Keep going
            } else if center_sense < left && center_sense < right {
                // Random turn
                if rng.gen_bool(0.5) {
                    next_angle += turn_angle;
                } else {
                    next_angle -= turn_angle;
                }
            } else if left > right {
                next_angle -= turn_angle;
            } else if right > left {
                next_angle += turn_angle;
            }

            // Add a little random wander
            next_angle += rng.gen_range(-0.1..0.1);

            let new_pos = agent.pos + vec2(next_angle.cos(), next_angle.sin()) * SPEED;
            let mut state = 0;
            let mut drop_pheromone = None;

            // Firewall collisions
            for (fw_center, fw_radius) in firewalls {
                if new_pos.distance(*fw_center) < *fw_radius {
                    state = 1;
                    break;
                }
            }

            let px = (new_pos.x / scale).clamp(0.0, (w - 1) as f32) as usize;
            let py = (new_pos.y / scale).clamp(0.0, (h - 1) as f32) as usize;

            if state == 0 {
                // Not dead, drop pheromone
                drop_pheromone = Some((px, py));
            } else {
                // Dead, maybe drop "negative" pheromone?
                // Let's just drop nothing so the trail decays and others find new paths
            }

            (new_pos, next_angle, state, drop_pheromone, 0.0)
        }).collect();

        let mut total_damage = 0.0;
        for (i, (pos, angle, state, pheromone, damage)) in updates.into_iter().enumerate() {
            self.agents[i].pos = pos;
            self.agents[i].angle = angle;
            self.agents[i].state = state;
            total_damage += damage;

            if let Some((px, py)) = pheromone {
                let idx = py * w + px;
                self.pheromones[idx] = (self.pheromones[idx] + 2.0).min(50.0);
            }
        }

        self.server_health = (self.server_health - total_damage).max(0.0);

        // Diffuse & Decay
        let prev_pheromones = self.pheromones.clone();

        self.pheromones.par_chunks_mut(w).enumerate().for_each(|(y, row)| {
            for (x, cell) in row.iter_mut().enumerate() {
                if y == 0 || y == h - 1 || x == 0 || x == w - 1 {
                    continue;
                }

                let mut sum = 0.0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let idx = ((y as isize + dy) as usize) * w + ((x as isize + dx) as usize);
                        sum += prev_pheromones[idx];
                    }
                }
                *cell = (sum / 9.0) * PHEROMONE_DECAY;
            }
        });
    }

    pub fn render_to_buffer(&self, buffer: &mut [u8], width: usize, height: usize) {
        // Dark background
        buffer.par_chunks_exact_mut(4).for_each(|pixel| {
            pixel[0] = 5;
            pixel[1] = 5;
            pixel[2] = 10;
            pixel[3] = 255;
        });

        let scale_x = width as f32 / WORLD_SIZE;
        let scale_y = height as f32 / WORLD_SIZE;

        // Draw Pheromones (Mycelium glow)
        for y in 0..self.grid_h {
            for x in 0..self.grid_w {
                let val = self.pheromones[y * self.grid_w + x];
                if val > 0.5 {
                    let screen_x = (x as f32 * GRID_SCALE as f32 * scale_x) as usize;
                    let screen_y = (y as f32 * GRID_SCALE as f32 * scale_y) as usize;

                    let block_size = (GRID_SCALE as f32 * scale_x).max(1.0) as usize;
                    let intensity = (val * 10.0).min(255.0) as u8;

                    for dy in 0..block_size {
                        for dx in 0..block_size {
                            let sx = screen_x + dx;
                            let sy = screen_y + dy;
                            if sx < width && sy < height {
                                let idx = (sy * width + sx) * 4;
                                // Fungal Glow (Cyan/Greenish)
                                buffer[idx] = buffer[idx].saturating_add(intensity / 4);
                                buffer[idx + 1] = buffer[idx + 1].saturating_add(intensity);
                                buffer[idx + 2] = buffer[idx + 2].saturating_add(intensity);
                            }
                        }
                    }
                }
            }
        }

        // Draw Agents
        for agent in &self.agents {
            let px = (agent.pos.x * scale_x) as isize;
            let py = (agent.pos.y * scale_y) as isize;

            if px >= 0 && px < width as isize && py >= 0 && py < height as isize {
                let idx = ((py as usize) * width + (px as usize)) * 4;

                if agent.state == 1 {
                    // Dead (Red)
                    buffer[idx] = 255;
                    buffer[idx + 1] = 50;
                    buffer[idx + 2] = 50;
                } else {
                    // Alive (Bright Cyan)
                    buffer[idx] = buffer[idx].saturating_add(100);
                    buffer[idx + 1] = buffer[idx + 1].saturating_add(255);
                    buffer[idx + 2] = buffer[idx + 2].saturating_add(255);
                }
            }
        }
    }

    pub fn add_firewall(&mut self, pos: Vec2, radius: f32) {
        self.firewalls.push((pos, radius));
    }

    pub fn clear_firewalls(&mut self) {
        self.firewalls.clear();
    }
}
