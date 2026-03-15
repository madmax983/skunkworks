use ::rand::prelude::*;
use ferrous_core::Platter;
use macroquad::prelude::*;
use rayon::prelude::*;
use crossbeam_channel::Sender;

use crate::audio::AudioCommand;
use crate::string::FerrousString;

pub const AGENT_COUNT: usize = 20_000;
pub const WORLD_SIZE: f32 = 1000.0;
pub const SPEED: f32 = 2.0;
pub const PHEROMONE_DECAY: f32 = 0.90;
pub const GRID_SCALE: f32 = 4.0;
pub const GRID_W: usize = 250;
pub const GRID_H: usize = 250;

#[derive(Clone, Copy)]
pub struct Locust {
    pub pos: Vec2,
    pub vel: Vec2,
    pub state: u8, // 0: Normal, 1: Dead (leaves trace)
}

pub struct World {
    pub agents: Vec<Locust>,
    pub pheromones: Vec<f32>,        // Grid of danger levels
    pub firewalls: Vec<(Vec2, f32)>, // (Center, Radius)
    pub target: Vec2,
    pub server_health: f32,
    pub max_health: f32,

    // Ferrous Strings specific
    pub platter: Platter,
    pub strings: Vec<FerrousString>,
    pub cmd_tx: Sender<AudioCommand>,
}

impl World {
    pub fn new(cmd_tx: Sender<AudioCommand>) -> Self {
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

        let mut strings = Vec::new();
        let string_count = 8;
        let string_spacing = 100.0;
        let base_freq = 110.0;
        for i in 0..string_count {
            let x = 100.0 + i as f32 * string_spacing;
            let pos = vec2(x, 100.0);
            let target_freq = base_freq * (2.0f32).powf(i as f32 / 12.0);
            strings.push(FerrousString::new(pos, 800.0, target_freq));
        }

        Self {
            agents,
            pheromones: vec![0.0; GRID_W * GRID_H],
            firewalls: Vec::new(),
            target: vec2(WORLD_SIZE / 2.0, WORLD_SIZE / 2.0),
            server_health: 10000.0,
            max_health: 10000.0,
            platter: Platter::new(GRID_W, GRID_H),
            strings,
            cmd_tx,
        }
    }

    pub fn add_firewall(&mut self, pos: Vec2, radius: f32) {
        self.firewalls.push((pos, radius));
    }

    pub fn clear_firewalls(&mut self) {
        self.firewalls.clear();
    }

    pub fn update(&mut self) {
        let dt = get_frame_time();

        // Decay Platter
        self.platter.decay(0.99);

        // Update Strings & Magnetize Platter
        for s in &mut self.strings {
            s.update_physics(dt, &self.platter, GRID_SCALE);
            if s.vibration.abs() > 0.1 {
                s.magnetize_platter(&mut self.platter, GRID_SCALE);
            }
        }

        // Decay Pheromones
        for p in &mut self.pheromones {
            *p *= PHEROMONE_DECAY;
            if *p < 0.01 {
                *p = 0.0;
            }
        }

        let target = self.target;
        let grid_w = GRID_W;
        let grid_h = GRID_H;
        let firewalls = &self.firewalls;
        let pheromones = &self.pheromones;
        let platter_magnetism = self.platter.magnetism();

        let mut server_damage = 0.0;

        // Process Agents
        let updates: Vec<(Locust, f32, Option<(usize, f32, f32)>)> = self
            .agents
            .par_iter()
            .map(|agent| {
                if agent.state == 1 {
                    // Dead agents just stay
                    return (*agent, 0.0, None);
                }

                let mut new_agent = *agent;
                let mut damage = 0.0;

                // 1. Calculate Desired Velocity (Towards Target)
                let to_target = target - new_agent.pos;
                let dist_to_target = to_target.length();

                if dist_to_target < 10.0 {
                    // Reached target
                    new_agent.state = 1;
                    damage = 1.0;
                    return (new_agent, damage, None);
                }

                let dir_to_target = to_target.normalize_or_zero();
                let mut desired_vel = dir_to_target * SPEED;

                // 2. Avoid Firewalls
                for (fw_pos, fw_radius) in firewalls {
                    let to_fw = *fw_pos - new_agent.pos;
                    let dist = to_fw.length();

                    if dist < *fw_radius + 5.0 {
                        // Very close, strong repulsion
                        let avoid = -to_fw.normalize_or_zero() * SPEED * 2.0;
                        desired_vel += avoid;

                        // High chance of death if hitting firewall directly
                        if dist < *fw_radius && ::rand::thread_rng().gen_bool(0.1) {
                            new_agent.state = 1;
                            return (new_agent, 0.0, None);
                        }
                    }
                }

                // 3. Avoid Pheromones (Danger paths) & Feel Magnetic Field
                let gx = (new_agent.pos.x / GRID_SCALE) as i32;
                let gy = (new_agent.pos.y / GRID_SCALE) as i32;

                if gx >= 0 && gy >= 0 && gx < grid_w as i32 && gy < grid_h as i32 {
                    let idx = (gy * grid_w as i32 + gx) as usize;
                    let danger = pheromones[idx];

                    if danger > 0.1 {
                        // Repel from danger
                        let mut rng = ::rand::thread_rng();
                        let random_dir = vec2(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize_or_zero();
                        desired_vel += random_dir * danger * SPEED;
                    }

                    // Feel Magnetic Field from Strings (read from platter)
                    let mag = platter_magnetism[idx] as f32;
                    // N/S poles attract or repel
                    if mag.abs() > 0.01 {
                        let pull = vec2(mag * SPEED * 0.5, 0.0);
                        desired_vel += pull;
                    }
                }

                // Smooth steering
                new_agent.vel = new_agent.vel.lerp(desired_vel, 0.1);
                new_agent.pos += new_agent.vel;

                // Check string intersection for plucking
                // Fast broadphase
                let mut pluck_info = None;
                if new_agent.vel.x.abs() > 0.5 {
                    // Approximating string x positions (100, 200, ... 800)
                    let string_idx = ((new_agent.pos.x - 100.0) / 100.0).round() as i32;
                    if string_idx >= 0 && string_idx < 8 {
                        let string_x = 100.0 + string_idx as f32 * 100.0;
                        // Did it cross the string this frame?
                        let prev_x = new_agent.pos.x - new_agent.vel.x;
                        let crossed = (prev_x < string_x && new_agent.pos.x >= string_x) || (prev_x > string_x && new_agent.pos.x <= string_x);

                        if crossed && new_agent.pos.y >= 100.0 && new_agent.pos.y <= 900.0 {
                            let strength = new_agent.vel.x.abs().clamp(5.0, 20.0);
                            let direction = new_agent.vel.x.signum();
                            pluck_info = Some((string_idx as usize, strength * direction, strength));
                        }
                    }
                }

                (new_agent, damage, pluck_info)
            })
            .collect();

        // Apply Updates
        let mut i = 0;
        let mut plucks_this_frame = Vec::new();

        for (new_agent, damage, pluck_info) in updates {
            if self.agents[i].state == 0 && new_agent.state == 1 {
                // Agent just died, leave pheromone
                let gx = (new_agent.pos.x / GRID_SCALE) as usize;
                let gy = (new_agent.pos.y / GRID_SCALE) as usize;
                if gx < GRID_W && gy < GRID_H {
                    let idx = gy * GRID_W + gx;
                    self.pheromones[idx] = (self.pheromones[idx] + 0.5).min(1.0);
                }
            }

            self.agents[i] = new_agent;
            server_damage += damage;

            if let Some((idx, pluck_val, strength)) = pluck_info {
                plucks_this_frame.push((idx, pluck_val, strength));
            }

            i += 1;
        }

        // Apply Plucks to strings
        let mut max_plucks_per_frame = 5;
        for (idx, pluck_val, strength) in plucks_this_frame {
            if max_plucks_per_frame > 0 && idx < self.strings.len() {
                self.strings[idx].pluck(pluck_val);
                let _ = self.cmd_tx.send(AudioCommand::Pluck {
                    frequency: self.strings[idx].frequency,
                    decay: self.strings[idx].decay,
                    amplitude: (strength / 20.0).clamp(0.1, 0.8),
                });
                max_plucks_per_frame -= 1;
            }
        }

        self.server_health = (self.server_health - server_damage).max(0.0);

        // Respawn dead agents
        let mut rng = ::rand::thread_rng();
        for a in &mut self.agents {
            if a.state == 1 && rng.gen_bool(0.05) {
                // Respawn
                let side = rng.gen_range(0..4);
                let (x, y) = match side {
                    0 => (rng.gen_range(0.0..WORLD_SIZE), 0.0),        // Top
                    1 => (rng.gen_range(0.0..WORLD_SIZE), WORLD_SIZE), // Bottom
                    2 => (0.0, rng.gen_range(0.0..WORLD_SIZE)),        // Left
                    _ => (WORLD_SIZE, rng.gen_range(0.0..WORLD_SIZE)), // Right
                };
                a.pos = vec2(x, y);
                a.vel = vec2(0.0, 0.0);
                a.state = 0;
            }
        }
    }

    pub fn render_to_buffer(&self, pixels: &mut [u8], width: usize, height: usize) {
        // Clear buffer
        for p in pixels.chunks_exact_mut(4) {
            p[0] = 0;
            p[1] = 0;
            p[2] = 0;
            p[3] = 255; // Alpha
        }

        // 1. Draw Platter (Heatmap)
        for y in 0..GRID_H {
            for x in 0..GRID_W {
                let mag = self.platter.get_magnetism(x, y); // 0.0 to 1.0

                let val = mag as f32;
                if val.abs() > 0.01 {
                    let mut r = 0;
                    let mut b = 0;
                    if val < 0.0 {
                        b = ((-val).clamp(0.0, 1.0) * 255.0) as u8;
                    } else {
                        r = (val.clamp(0.0, 1.0) * 255.0) as u8;
                    }

                    // Map grid to screen space
                    // Each grid cell is GRID_SCALE x GRID_SCALE
                    let px = (x as f32 * GRID_SCALE / WORLD_SIZE * width as f32) as usize;
                    let py = (y as f32 * GRID_SCALE / WORLD_SIZE * height as f32) as usize;
                    let cell_w = (GRID_SCALE / WORLD_SIZE * width as f32).ceil() as usize;
                    let cell_h = (GRID_SCALE / WORLD_SIZE * height as f32).ceil() as usize;

                    for dy in 0..cell_h {
                        for dx in 0..cell_w {
                            let cx = px + dx;
                            let cy = py + dy;
                            if cx < width && cy < height {
                                let idx = (cy * width + cx) * 4;
                                pixels[idx] = r.saturating_add(pixels[idx]); // R
                                pixels[idx + 2] = b.saturating_add(pixels[idx + 2]); // B
                            }
                        }
                    }
                }
            }
        }


        // 2. Draw Pheromones (Danger)
        for y in 0..GRID_H {
            for x in 0..GRID_W {
                let val = self.pheromones[y * GRID_W + x];
                if val > 0.05 {
                    let c = (val * 255.0) as u8;
                    let px = (x as f32 * GRID_SCALE / WORLD_SIZE * width as f32) as usize;
                    let py = (y as f32 * GRID_SCALE / WORLD_SIZE * height as f32) as usize;

                    if px < width && py < height {
                        let idx = (py * width + px) * 4;
                        pixels[idx] = c.saturating_add(pixels[idx]); // R
                        pixels[idx + 1] = (c / 2).saturating_add(pixels[idx + 1]); // G
                    }
                }
            }
        }

        // 3. Draw Agents
        for agent in &self.agents {
            if agent.state == 0 {
                let px = (agent.pos.x / WORLD_SIZE * width as f32) as usize;
                let py = (agent.pos.y / WORLD_SIZE * height as f32) as usize;

                if px < width && py < height {
                    let idx = (py * width + px) * 4;
                    // Draw agents as cyan
                    pixels[idx] = 0;     // R
                    pixels[idx + 1] = 255; // G
                    pixels[idx + 2] = 255; // B
                }
            }
        }
    }
}
