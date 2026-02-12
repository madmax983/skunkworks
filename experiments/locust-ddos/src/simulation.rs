use macroquad::prelude::*;
use rayon::prelude::*;
use ::rand::prelude::*;

pub const AGENT_COUNT: usize = 100_000;
pub const WORLD_SIZE: f32 = 1000.0;
pub const SPEED: f32 = 2.0;
pub const PHEROMONE_DECAY: f32 = 0.90;
pub const GRID_SCALE: usize = 4; // 1000 / 4 = 250x250 grid

#[derive(Clone, Copy)]
pub struct Locust {
    pub pos: Vec2,
    pub vel: Vec2,
    pub state: u8, // 0: Normal, 1: Dead (leaves trace)
}

pub struct World {
    pub agents: Vec<Locust>,
    pub pheromones: Vec<f32>, // Grid of danger levels
    pub firewalls: Vec<(Vec2, f32)>, // (Center, Radius)
    pub width: usize,
    pub height: usize,
    pub grid_w: usize,
    pub grid_h: usize,
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
                0 => (rng.gen_range(0.0..WORLD_SIZE), 0.0), // Top
                1 => (rng.gen_range(0.0..WORLD_SIZE), WORLD_SIZE), // Bottom
                2 => (0.0, rng.gen_range(0.0..WORLD_SIZE)), // Left
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

        World {
            agents,
            pheromones: vec![0.0; grid_w * grid_h],
            firewalls: Vec::new(),
            width: WORLD_SIZE as usize,
            height: WORLD_SIZE as usize,
            grid_w,
            grid_h,
            target: vec2(WORLD_SIZE / 2.0, WORLD_SIZE / 2.0),
        }
    }

    pub fn update(&mut self) {
        let target = self.target;
        let firewalls = &self.firewalls;
        let grid_w = self.grid_w;
        let grid_h = self.grid_h;
        let scale = GRID_SCALE as f32;

        // We need a read-only view of pheromones for decision making
        let pheromones = &self.pheromones;

        // Parallel update of agents
        let updates: Vec<(Vec2, Vec2, u8, Option<(usize, usize)>)> = self.agents.par_iter().map(|agent| {
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
                    return (vec2(x, y), vec2(0.0, 0.0), 0, None);
                }
                return (agent.pos, agent.vel, 1, None);
            }

            // Seek Target
            let to_target = target - agent.pos;
            let dist_target = to_target.length();
            let mut desire = if dist_target > 0.0 {
                to_target.normalize() * SPEED
            } else {
                vec2(0.0, 0.0)
            };

            // Avoid Pheromones (Danger)
            // Check ahead
            let look_ahead = agent.pos + agent.vel.normalize_or_zero() * 10.0;
            let gx = (look_ahead.x / scale).clamp(0.0, (grid_w - 1) as f32) as usize;
            let gy = (look_ahead.y / scale).clamp(0.0, (grid_h - 1) as f32) as usize;
            let idx = gy * grid_w + gx;

            if pheromones[idx] > 0.1 {
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
             if new_pos.x < 0.0 || new_pos.x > WORLD_SIZE || new_pos.y < 0.0 || new_pos.y > WORLD_SIZE {
                 // Wrap or clamp? Let's respawn if out of bounds (shouldn't happen often seeking center)
                 new_pos = new_pos.clamp(vec2(0.0, 0.0), vec2(WORLD_SIZE, WORLD_SIZE));
             }

            (new_pos, new_vel, state, drop_pheromone)
        }).collect();

        // Apply updates
        for (i, (pos, vel, state, pheromone)) in updates.into_iter().enumerate() {
            self.agents[i].pos = pos;
            self.agents[i].vel = vel;
            self.agents[i].state = state;

            if let Some((px, py)) = pheromone {
                let idx = py * self.grid_w + px;
                self.pheromones[idx] = (self.pheromones[idx] + 2.0).min(10.0);
            }
        }

        // Decay pheromones
        // Simple decay
        // Also maybe diffuse? Diffusion is expensive on CPU for 250x250 every frame?
        // 62500 cells. It's fine.

        // Parallel diffusion (approximated or just simple decay for speed first)
        // Let's just do decay first. 100k agents is the bottleneck.
        self.pheromones.par_iter_mut().for_each(|p| {
            *p *= PHEROMONE_DECAY;
            if *p < 0.01 { *p = 0.0; }
        });
    }

    pub fn render_to_buffer(&self, buffer: &mut [u8], width: usize, height: usize) {
        // Clear buffer (Black)
        // Using parellel iterator for clearing is overkill but consistent
        buffer.par_chunks_exact_mut(4).for_each(|pixel| {
            pixel[0] = 10; // Dark background
            pixel[1] = 10;
            pixel[2] = 20;
            pixel[3] = 255;
        });

        let scale_x = width as f32 / WORLD_SIZE;
        let scale_y = height as f32 / WORLD_SIZE;

        // Draw Pheromones (Heatmap)
        // Iterate over grid and draw rects? No, too slow.
        // We iterate pixels and sample grid?
        // Or just map grid directly if resolution matches.
        // Let's skip drawing pheromones for now, or draw them lightly.

        // Draw Agents
        // Serial because random access to buffer is hard to parallelize without conflicts
        // But we can use atomics or just accept race conditions (glitch art!)
        // Since we want deterministic-ish visualization, serial is safer.
        // 100k points is fast on CPU.

        for agent in &self.agents {
            let px = (agent.pos.x * scale_x) as isize;
            let py = (agent.pos.y * scale_y) as isize;

            if px >= 0 && px < width as isize && py >= 0 && py < height as isize {
                let idx = ((py as usize) * width + (px as usize)) * 4;

                if agent.state == 1 {
                    // Dead / blocked (Red)
                    buffer[idx] = 255;
                    buffer[idx+1] = 50;
                    buffer[idx+2] = 50;
                } else {
                    // Alive (Green/Cyan)
                    // Additive blending manually
                    let c = buffer[idx+1].saturating_add(100);
                    buffer[idx+1] = c;
                    buffer[idx+2] = c;
                }
            }
        }

        // Draw Pheromones (Overlay)
         for y in 0..self.grid_h {
            for x in 0..self.grid_w {
                let val = self.pheromones[y * self.grid_w + x];
                if val > 0.1 {
                     // Map grid cell to pixels
                     let screen_x = (x as f32 * GRID_SCALE as f32 * scale_x) as usize;
                     let screen_y = (y as f32 * GRID_SCALE as f32 * scale_y) as usize;

                     // Draw a generic 2x2 or 4x4 block
                     let block_size = (GRID_SCALE as f32 * scale_x) as usize;

                     for dy in 0..block_size {
                         for dx in 0..block_size {
                             let sx = screen_x + dx;
                             let sy = screen_y + dy;
                             if sx < width && sy < height {
                                 let idx = (sy * width + sx) * 4;
                                 let r = buffer[idx].saturating_add((val * 20.0) as u8);
                                 buffer[idx] = r;
                             }
                         }
                     }
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
}
