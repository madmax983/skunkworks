use ::rand::prelude::*;
use macroquad::prelude::*;
use rayon::prelude::*;

pub const AGENT_COUNT: usize = 100_000;
pub const WORLD_SIZE: f32 = 1000.0;
pub const SPEED: f32 = 2.0;
pub const PHEROMONE_DECAY: f32 = 0.90;
pub const GRID_SCALE: usize = 4; // 1000 / 4 = 250x250 grid

#[derive(Clone, Copy)]
pub struct Target {
    pub pos: Vec2,
    pub attractiveness: f32, // 0.0 to 1.0
    pub health: f32,
    pub max_health: f32,
}

#[derive(Clone, Copy)]
pub struct Locust {
    pub pos: Vec2,
    pub vel: Vec2,
    pub state: u8, // 0: Normal, 1: Dead (leaves trace)
    pub current_target: Option<usize>,
}

pub struct World {
    pub agents: Vec<Locust>,
    pub pheromones: Vec<f32>,        // Grid of danger levels
    pub firewalls: Vec<(Vec2, f32)>, // (Center, Radius)
    pub grid_w: usize,
    pub grid_h: usize,
    pub targets: Vec<Target>,
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
                current_target: None,
            });
        }

        let grid_w = (WORLD_SIZE / GRID_SCALE as f32) as usize;
        let grid_h = (WORLD_SIZE / GRID_SCALE as f32) as usize;

        Self {
            agents,
            pheromones: vec![0.0; grid_w * grid_h],
            firewalls: Vec::new(),
            grid_w,
            grid_h,
            targets: Vec::new(),
        }
    }

    pub fn update(&mut self) {
        let firewalls = &self.firewalls;
        let grid_w = self.grid_w;
        let grid_h = self.grid_h;
        let scale = GRID_SCALE as f32;

        let pheromones = &self.pheromones;
        let targets = &self.targets;

        // Parallel update of agents
        let updates: Vec<(Vec2, Vec2, u8, Option<(usize, usize)>, Option<usize>)> = self
            .agents
            .par_iter()
            .map(|agent| {
                if agent.state == 1 {
                    let mut rng = ::rand::thread_rng();
                    if rng.gen_bool(0.01) {
                        let side = rng.gen_range(0..4);
                        let (x, y) = match side {
                            0 => (rng.gen_range(0.0..WORLD_SIZE), 0.0),
                            1 => (rng.gen_range(0.0..WORLD_SIZE), WORLD_SIZE),
                            2 => (0.0, rng.gen_range(0.0..WORLD_SIZE)),
                            _ => (WORLD_SIZE, rng.gen_range(0.0..WORLD_SIZE)),
                        };
                        return (vec2(x, y), vec2(0.0, 0.0), 0, None, None);
                    }
                    return (agent.pos, agent.vel, 1, None, None);
                }

                if targets.is_empty() {
                    return (agent.pos, agent.vel, agent.state, None, None);
                }

                // Choose a target if not already chasing one, or occasionally re-evaluate
                let mut current_target_idx = agent.current_target;
                let mut rng = ::rand::thread_rng();

                if current_target_idx.is_none() || rng.gen_bool(0.05) {
                    // Pick a random target, weighted by attractiveness (approximated randomly for performance)
                    // Alternatively, just pick a random alive target
                    let alive_targets: Vec<usize> = targets
                        .iter()
                        .enumerate()
                        .filter(|(_, t)| t.health > 0.0)
                        .map(|(i, _)| i)
                        .collect();

                    if !alive_targets.is_empty() {
                        let rand_idx = alive_targets[rng.gen_range(0..alive_targets.len())];
                        current_target_idx = Some(rand_idx);
                    } else {
                        current_target_idx = None;
                    }
                }

                let mut damage_target = None;
                let mut desire = vec2(0.0, 0.0);

                if let Some(target_idx) = current_target_idx {
                    if let Some(target) = targets.get(target_idx) {
                        if target.health > 0.0 {
                            let to_target = target.pos - agent.pos;
                            let dist_target = to_target.length();

                            if dist_target < 10.0 {
                                // Hit Target!
                                damage_target = Some(target_idx);

                                // Respawn
                                let side = rng.gen_range(0..4);
                                let (x, y) = match side {
                                    0 => (rng.gen_range(0.0..WORLD_SIZE), 0.0),
                                    1 => (rng.gen_range(0.0..WORLD_SIZE), WORLD_SIZE),
                                    2 => (0.0, rng.gen_range(0.0..WORLD_SIZE)),
                                    _ => (WORLD_SIZE, rng.gen_range(0.0..WORLD_SIZE)),
                                };
                                return (vec2(x, y), vec2(0.0, 0.0), 0, None, damage_target);
                            }

                            if dist_target > 0.0 {
                                desire = to_target.normalize() * SPEED;
                            }
                        }
                    }
                }

                // Avoid Pheromones (Danger)
                let look_ahead = agent.pos + agent.vel.normalize_or_zero() * 10.0;
                let gx = (look_ahead.x / scale).clamp(0.0, (grid_w - 1) as f32) as usize;
                let gy = (look_ahead.y / scale).clamp(0.0, (grid_h - 1) as f32) as usize;
                let idx = gy * grid_w + gx;

                if pheromones[idx] > 0.1 {
                    let angle = rng.gen_range(-std::f32::consts::PI..std::f32::consts::PI);
                    let avoid = vec2(angle.cos(), angle.sin()) * SPEED * 2.0;
                    desire += avoid;
                }

                // Apply steering
                let steer = (desire - agent.vel).clamp_length_max(0.5);
                let new_vel = (agent.vel + steer).clamp_length_max(SPEED);
                let mut new_pos = agent.pos + new_vel;

                // Wall collisions (Firewalls)
                let mut state = 0;
                let mut drop_pheromone = None;

                for (center, radius) in firewalls {
                    if new_pos.distance(*center) < *radius {
                        state = 1; // Die
                        let px = (new_pos.x / scale).clamp(0.0, (grid_w - 1) as f32) as usize;
                        let py = (new_pos.y / scale).clamp(0.0, (grid_h - 1) as f32) as usize;
                        drop_pheromone = Some((px, py));
                        break;
                    }
                }

                // Screen Bounds
                if new_pos.x < 0.0
                    || new_pos.x > WORLD_SIZE
                    || new_pos.y < 0.0
                    || new_pos.y > WORLD_SIZE
                {
                    new_pos = new_pos.clamp(vec2(0.0, 0.0), vec2(WORLD_SIZE, WORLD_SIZE));
                }

                (new_pos, new_vel, state, drop_pheromone, damage_target)
            })
            .collect();

        // Apply updates
        for (i, (pos, vel, state, pheromone, damage_target)) in updates.into_iter().enumerate() {
            self.agents[i].pos = pos;
            self.agents[i].vel = vel;
            self.agents[i].state = state;

            if let Some((px, py)) = pheromone {
                let idx = py * self.grid_w + px;
                self.pheromones[idx] = (self.pheromones[idx] + 2.0).min(10.0);
            }

            if let Some(t_idx) = damage_target {
                if let Some(target) = self.targets.get_mut(t_idx) {
                    target.health = (target.health - 1.0).max(0.0);
                }
            }
        }

        // Diffusion (Box Blur)
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

    pub fn render_to_buffer(&self, buffer: &mut [u8], width: usize, height: usize) {
        // Clear buffer (Dark Blue-ish)
        buffer.par_chunks_exact_mut(4).for_each(|pixel| {
            pixel[0] = 5;
            pixel[1] = 5;
            pixel[2] = 15;
            pixel[3] = 255;
        });

        let scale_x = width as f32 / WORLD_SIZE;
        let scale_y = height as f32 / WORLD_SIZE;

        // Draw Pheromones (Heatmap)
        for y in 0..self.grid_h {
            for x in 0..self.grid_w {
                let val = self.pheromones[y * self.grid_w + x];
                if val > 0.1 {
                    let screen_x = (x as f32 * GRID_SCALE as f32 * scale_x) as usize;
                    let screen_y = (y as f32 * GRID_SCALE as f32 * scale_y) as usize;

                    let block_size = (GRID_SCALE as f32 * scale_x) as usize;
                    let intensity = (val * 25.0).min(200.0) as u8;

                    for dy in 0..block_size {
                        for dx in 0..block_size {
                            let sx = screen_x + dx;
                            let sy = screen_y + dy;
                            if sx < width && sy < height {
                                let idx = (sy * width + sx) * 4;
                                buffer[idx] = buffer[idx].saturating_add(intensity);
                                buffer[idx + 1] = buffer[idx + 1].saturating_add(intensity / 4);
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
                    // Dead (Bright Red / White)
                    buffer[idx] = 255;
                    buffer[idx + 1] = 150;
                    buffer[idx + 2] = 150;
                } else {
                    // Alive
                    buffer[idx] = buffer[idx].saturating_add(50);
                    buffer[idx + 1] = buffer[idx + 1].saturating_add(200);
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
