use ::rand::prelude::*;
use macroquad::prelude::*;
use rayon::prelude::*;

pub const AGENT_COUNT: usize = 20_000;
pub const WORLD_SIZE: f32 = 1000.0;
pub const SPEED: f32 = 2.0;
pub const PHEROMONE_DECAY: f32 = 0.95;
pub const GRID_SCALE: usize = 4; // 1000 / 4 = 250x250 grid

#[derive(Clone, Copy)]
pub struct Locust {
    pub pos: Vec2,
    pub vel: Vec2,
    pub state: u8, // 0: Normal, 1: Reached Node (leaves trace)
}

pub struct World {
    pub agents: Vec<Locust>,
    pub pheromones: Vec<f32>,
    pub commit_nodes: Vec<(Vec2, f32)>, // (Pos, Radius/Size)
    pub current_target: Vec2,
    pub grid_w: usize,
    pub grid_h: usize,
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
                pos: vec2(x, y),
                vel: vec2(0.0, 0.0),
                state: 0,
            });
        }

        let grid_w = (WORLD_SIZE as usize) / GRID_SCALE;
        let grid_h = (WORLD_SIZE as usize) / GRID_SCALE;

        Self {
            agents,
            pheromones: vec![0.0; grid_w * grid_h],
            commit_nodes: Vec::new(),
            current_target: vec2(WORLD_SIZE / 2.0, WORLD_SIZE / 2.0),
            grid_w,
            grid_h,
        }
    }

    pub fn set_target(&mut self, pos: Vec2) {
        self.current_target = pos;
        self.commit_nodes.push((pos, 20.0));
        if self.commit_nodes.len() > 10 {
            self.commit_nodes.remove(0); // Keep last 10
        }
    }

    pub fn update(&mut self) {
        let target = self.current_target;
        let grid_w = self.grid_w;
        let grid_h = self.grid_h;
        let scale = GRID_SCALE as f32;

        let pheromones = &self.pheromones;

        let updates: Vec<(Vec2, Vec2, u8, Option<(usize, usize)>)> = self
            .agents
            .par_iter()
            .map(|agent| {
                if agent.state == 1 {
                    let mut rng = ::rand::thread_rng();
                    if rng.gen_bool(0.02) {
                        let side = rng.gen_range(0..4);
                        let (x, y) = match side {
                            0 => (rng.gen_range(0.0..WORLD_SIZE), 0.0),
                            1 => (rng.gen_range(0.0..WORLD_SIZE), WORLD_SIZE),
                            2 => (0.0, rng.gen_range(0.0..WORLD_SIZE)),
                            _ => (WORLD_SIZE, rng.gen_range(0.0..WORLD_SIZE)),
                        };
                        return (vec2(x, y), vec2(0.0, 0.0), 0, None);
                    }
                    return (agent.pos, agent.vel, 1, None);
                }

                let to_target = target - agent.pos;
                let dist_target = to_target.length();

                if dist_target < 10.0 {
                    // Reached commit node
                    let px = (agent.pos.x / scale).clamp(0.0, (grid_w - 1) as f32) as usize;
                    let py = (agent.pos.y / scale).clamp(0.0, (grid_h - 1) as f32) as usize;
                    return (agent.pos, vec2(0.0, 0.0), 1, Some((px, py)));
                }

                let mut desire = if dist_target > 0.0 {
                    to_target.normalize() * SPEED
                } else {
                    vec2(0.0, 0.0)
                };

                // Follow Pheromones (Attraction to established paths)
                let look_ahead = agent.pos + agent.vel.normalize_or_zero() * 10.0;
                let gx = (look_ahead.x / scale).clamp(0.0, (grid_w - 1) as f32) as usize;
                let gy = (look_ahead.y / scale).clamp(0.0, (grid_h - 1) as f32) as usize;
                let idx = gy * grid_w + gx;

                if pheromones[idx] > 0.1 {
                    desire += agent.vel.normalize_or_zero() * SPEED * 1.5; // Speed up along path
                }

                // Add some noise for organic swarming
                let mut rng = ::rand::thread_rng();
                let angle_noise: f32 = rng.gen_range(-0.5..0.5);
                let noise = vec2(angle_noise.cos(), angle_noise.sin()) * SPEED * 0.5;
                desire += noise;

                let steer = (desire - agent.vel).clamp_length_max(0.2);
                let new_vel = (agent.vel + steer).clamp_length_max(SPEED);
                let mut new_pos = agent.pos + new_vel;

                // Screen Bounds
                if new_pos.x < 0.0 || new_pos.x > WORLD_SIZE || new_pos.y < 0.0 || new_pos.y > WORLD_SIZE {
                    new_pos = new_pos.clamp(vec2(0.0, 0.0), vec2(WORLD_SIZE, WORLD_SIZE));
                }

                let px = (new_pos.x / scale).clamp(0.0, (grid_w - 1) as f32) as usize;
                let py = (new_pos.y / scale).clamp(0.0, (grid_h - 1) as f32) as usize;
                let drop_pheromone = if rng.gen_bool(0.1) { Some((px, py)) } else { None };

                (new_pos, new_vel, 0, drop_pheromone)
            })
            .collect();

        for (i, (pos, vel, state, pheromone)) in updates.into_iter().enumerate() {
            self.agents[i].pos = pos;
            self.agents[i].vel = vel;
            self.agents[i].state = state;

            if let Some((px, py)) = pheromone {
                let idx = py * self.grid_w + px;
                self.pheromones[idx] = (self.pheromones[idx] + 0.5).min(5.0);
            }
        }

        let w = self.grid_w;
        let h = self.grid_h;
        let prev_pheromones = self.pheromones.clone();

        self.pheromones
            .par_chunks_mut(w)
            .enumerate()
            .for_each(|(y, row)| {
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
                    *cell = sum / 9.0;
                }
            });

        self.pheromones.par_iter_mut().for_each(|p| {
            *p *= PHEROMONE_DECAY;
            if *p < 0.01 {
                *p = 0.0;
            }
        });
    }

    pub fn render_to_buffer(&self, buffer: &mut [u8], width: usize, height: usize) {
        // Clear buffer
        buffer.par_chunks_exact_mut(4).for_each(|pixel| {
            pixel[0] = 5;
            pixel[1] = 5;
            pixel[2] = 10;
            pixel[3] = 255;
        });

        let scale_x = width as f32 / WORLD_SIZE;
        let scale_y = height as f32 / WORLD_SIZE;

        // Draw Pheromones (Trails)
        for y in 0..self.grid_h {
            for x in 0..self.grid_w {
                let val = self.pheromones[y * self.grid_w + x];
                if val > 0.1 {
                    let screen_x = (x as f32 * GRID_SCALE as f32 * scale_x) as usize;
                    let screen_y = (y as f32 * GRID_SCALE as f32 * scale_y) as usize;
                    let block_size = (GRID_SCALE as f32 * scale_x) as usize;
                    let intensity = (val * 40.0).min(200.0) as u8;

                    for dy in 0..block_size {
                        for dx in 0..block_size {
                            let sx = screen_x + dx;
                            let sy = screen_y + dy;
                            if sx < width && sy < height {
                                let idx = (sy * width + sx) * 4;
                                buffer[idx] = buffer[idx].saturating_add(intensity / 2);
                                buffer[idx + 1] = buffer[idx + 1].saturating_add(intensity);
                                buffer[idx + 2] = buffer[idx + 2].saturating_add(intensity);
                            }
                        }
                    }
                }
            }
        }

        // Draw Nodes
        for (pos, radius) in &self.commit_nodes {
            let cx = (pos.x * scale_x) as isize;
            let cy = (pos.y * scale_y) as isize;
            let r = (radius * scale_x) as isize;

            for y in (cy - r)..=(cy + r) {
                for x in (cx - r)..=(cx + r) {
                    if x >= 0 && x < width as isize && y >= 0 && y < height as isize {
                        if (x - cx) * (x - cx) + (y - cy) * (y - cy) <= r * r {
                            let idx = ((y as usize) * width + (x as usize)) * 4;
                            buffer[idx] = 255;
                            buffer[idx + 1] = 200;
                            buffer[idx + 2] = 50;
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
                    buffer[idx] = 255;
                    buffer[idx + 1] = 255;
                    buffer[idx + 2] = 100;
                } else {
                    buffer[idx] = buffer[idx].saturating_add(100);
                    buffer[idx + 1] = buffer[idx + 1].saturating_add(150);
                    buffer[idx + 2] = buffer[idx + 2].saturating_add(255);
                }
            }
        }
    }
}
