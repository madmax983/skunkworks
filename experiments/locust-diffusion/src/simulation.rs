use ::rand::prelude::*;
use gray_scott::GrayScott;
use macroquad::prelude::*;
use rayon::prelude::*;

pub const AGENT_COUNT: usize = 100_000;
pub const WORLD_SIZE: f32 = 1000.0;
pub const SPEED: f32 = 2.0;
pub const GRID_SCALE: usize = 4; // 1000 / 4 = 250x250 grid

#[derive(Clone, Copy)]
pub struct Locust {
    pub pos: Vec2,
    pub vel: Vec2,
    pub state: u8, // 0: Normal, 1: Dead (leaves trace)
}

pub struct World {
    pub agents: Vec<Locust>,
    pub gray_scott: GrayScott,
    pub firewalls: Vec<(Vec2, f32)>, // (Center, Radius)
    pub grid_w: usize,
    pub grid_h: usize,
    pub target: Vec2,
    pub server_health: f32,
    pub max_health: f32,
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
                vel: vec2(0.0, 0.0), // Starts still, will seek
                state: 0,
            });
        }

        let grid_w = (WORLD_SIZE as usize) / GRID_SCALE;
        let grid_h = (WORLD_SIZE as usize) / GRID_SCALE;

        let mut gray_scott = GrayScott::new(grid_w, grid_h);

        // Target area emits high V (food)
        let cx = grid_w / 2;
        let cy = grid_h / 2;
        for dy in -5..=5isize {
            for dx in -5..=5isize {
                let nx = (cx as isize + dx) as usize;
                let ny = (cy as isize + dy) as usize;
                if nx < grid_w && ny < grid_h {
                    gray_scott.add_chemical(nx, ny, 1.0);
                }
            }
        }

        Self {
            agents,
            gray_scott,
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
        let grid_w = self.grid_w;
        let grid_h = self.grid_h;
        let scale = GRID_SCALE as f32;

        // Server constantly emits V (catalyst/food)
        let cx = grid_w / 2;
        let cy = grid_h / 2;
        self.gray_scott.add_chemical(cx, cy, 0.5);

        // Update Gray-Scott (Reaction-Diffusion)
        self.gray_scott.update(0.055, 0.062, 1.0);

        // We need a read-only view of the chemicals for decision making
        let v_chem = self.gray_scott.v();
        let u_chem = self.gray_scott.u();

        // Parallel update of agents
        let updates: Vec<(Vec2, Vec2, u8, Option<(usize, usize)>, f32)> = self
            .agents
            .par_iter()
            .map(|agent| {
                if agent.state == 1 {
                    // Dead agents don't move, but might respawn or stay dead
                    // Let's respawn them at edges if they are dead for too long?
                    // For now, just keep them dead as "blockage" markers, or respawn instantly at edge
                    let mut rng = ::rand::thread_rng();
                    if rng.gen_bool(0.01) {
                        let side = rng.gen_range(0..4);
                        let (x, y) = match side {
                            0 => (rng.gen_range(0.0..WORLD_SIZE), 0.0),
                            1 => (rng.gen_range(0.0..WORLD_SIZE), WORLD_SIZE),
                            2 => (0.0, rng.gen_range(0.0..WORLD_SIZE)),
                            _ => (WORLD_SIZE, rng.gen_range(0.0..WORLD_SIZE)),
                        };
                        return (vec2(x, y), vec2(0.0, 0.0), 0, None, 0.0);
                    }
                    return (agent.pos, agent.vel, 1, None, 0.0);
                }

                // Seek Target
                let to_target = target - agent.pos;
                let dist_target = to_target.length();

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
                    return (vec2(x, y), vec2(0.0, 0.0), 0, None, 1.0);
                }

                let mut desire = if dist_target > 0.0 {
                    to_target.normalize() * SPEED
                } else {
                    vec2(0.0, 0.0)
                };

                // Navigate via chemicals
                // Seek V, Avoid U
                let look_ahead = agent.pos + agent.vel.normalize_or_zero() * 10.0;
                let gx = (look_ahead.x / scale).clamp(0.0, (grid_w - 1) as f32) as usize;
                let gy = (look_ahead.y / scale).clamp(0.0, (grid_h - 1) as f32) as usize;
                let idx = gy * grid_w + gx;

                // Follow V gradient and avoid U gradient
                // Sample neighbors to find gradient
                let mut max_v = 0.0;
                let mut best_dx = 0.0;
                let mut best_dy = 0.0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 { continue; }
                        let nx = (gx as isize + dx).clamp(0, (grid_w - 1) as isize) as usize;
                        let ny = (gy as isize + dy).clamp(0, (grid_h - 1) as isize) as usize;
                        let n_idx = ny * grid_w + nx;

                        let score = v_chem[n_idx] - u_chem[n_idx];
                        if score > max_v {
                            max_v = score;
                            best_dx = dx as f32;
                            best_dy = dy as f32;
                        }
                    }
                }

                if max_v > 0.0 {
                    desire += vec2(best_dx, best_dy).normalize_or_zero() * SPEED * 0.5;
                }

                if u_chem[idx] > 0.6 {
                    // Danger ahead! Steer away randomly or perpendicular
                    let mut rng = ::rand::thread_rng();
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
                                   // Drop pheromone at current grid
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
                    // Wrap or clamp? Let's respawn if out of bounds (shouldn't happen often seeking center)
                    new_pos = new_pos.clamp(vec2(0.0, 0.0), vec2(WORLD_SIZE, WORLD_SIZE));
                }

                (new_pos, new_vel, state, drop_pheromone, 0.0)
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
                // Deposit U (inhibitor) where agents die
                for dy in -2..=2isize {
                    for dx in -2..=2isize {
                        let nx = (px as isize + dx) as usize;
                        let ny = (py as isize + dy) as usize;
                        if nx < self.grid_w && ny < self.grid_h {
                            // Instead of add_chemical which adds V, we want to add U.
                            // We don't have an `add_u` method in GrayScott, but we can set it via u_mut.
                            let idx = ny * self.grid_w + nx;
                            self.gray_scott.u_mut()[idx] = 1.0;
                            // And deplete V
                            self.gray_scott.v_mut()[idx] = 0.0;
                        }
                    }
                }
            }
        }

        self.server_health = (self.server_health - total_damage).max(0.0);
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

        // Draw Chemicals (Heatmap)
        // V is target (Green), U is walls/dead (Red)
        for y in 0..self.grid_h {
            for x in 0..self.grid_w {
                let idx_gs = y * self.grid_w + x;
                let v = self.gray_scott.v()[idx_gs];
                let u = self.gray_scott.u()[idx_gs];

                if v > 0.1 || u > 0.5 {
                    // Map grid cell to pixels
                    let screen_x = (x as f32 * GRID_SCALE as f32 * scale_x) as usize;
                    let screen_y = (y as f32 * GRID_SCALE as f32 * scale_y) as usize;

                    let block_size = (GRID_SCALE as f32 * scale_x) as usize;

                    let v_intensity = (v * 255.0).min(255.0) as u8;
                    // U normally sits at 1.0, so let's only draw where U is high AND V is low, to show dead zones
                    let u_intensity = if u > 0.8 && v < 0.2 { ((u - 0.8) * 5.0 * 255.0).min(200.0) as u8 } else { 0 };

                    if v_intensity > 0 || u_intensity > 0 {
                        for dy in 0..block_size {
                            for dx in 0..block_size {
                                let sx = screen_x + dx;
                                let sy = screen_y + dy;
                                if sx < width && sy < height {
                                    let idx = (sy * width + sx) * 4;

                                    // Red for U (walls)
                                    buffer[idx] = buffer[idx].saturating_add(u_intensity);
                                    // Green for V (target food)
                                    buffer[idx + 1] = buffer[idx + 1].saturating_add(v_intensity);
                                }
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
                    // Dead / blocked (Bright Red / White)
                    buffer[idx] = 255;
                    buffer[idx + 1] = 150;
                    buffer[idx + 2] = 150;
                } else {
                    // Alive (Cyan / Electric Blue)
                    // Additive blending
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_world_init() {
        let world = World::new();
        assert_eq!(world.agents.len(), AGENT_COUNT);
    }

    #[test]
    fn test_server_damage() {
        let mut world = World::new();
        // Move one agent to target
        world.agents[0].pos = world.target;

        let initial_health = world.server_health;
        world.update();

        assert!(world.server_health < initial_health);
    }
}
