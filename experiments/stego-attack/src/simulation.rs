use crate::config::AttackConfig;
use ::rand::Rng;
use image::RgbaImage;
use macroquad::prelude::*;
use rayon::prelude::*;

#[derive(Clone, Copy)]
pub struct Agent {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: [u8; 4],
    pub active: bool,
}

pub struct World {
    pub agents: Vec<Agent>,
    pub background: Vec<u8>, // RGBA8 buffer of the image state
    pub width: usize,
    pub height: usize,
    pub target: Vec2,
    pub speed: f32,
    pub dissolve_rate: f32,
}

impl World {
    pub fn new(image: &RgbaImage, config: &AttackConfig) -> Self {
        let width = image.width() as usize;
        let height = image.height() as usize;
        let mut agents = Vec::new();
        let mut background = image.as_raw().clone();

        let mut rng = ::rand::thread_rng();

        // Spawn agents from non-transparent pixels
        // Stride to limit agent count (e.g., every 5th pixel)
        // Adjust stride based on image size to aim for ~10k-50k agents
        let pixel_count = width * height;
        let stride = if pixel_count > 50_000 {
            pixel_count / 20_000
        } else {
            1
        };

        for (i, pixel) in image.pixels().enumerate() {
            if i % stride != 0 {
                continue;
            }

            let [r, g, b, a] = pixel.0;
            if a > 10 {
                let x = (i % width) as f32;
                let y = (i / width) as f32;

                // Random initial velocity
                let angle = rng.gen_range(0.0..std::f32::consts::TAU);
                let vel = vec2(angle.cos(), angle.sin()) * 0.5;

                agents.push(Agent {
                    pos: vec2(x, y),
                    vel,
                    color: [r, g, b, a],
                    active: true,
                });

                // Dissolve source pixel?
                // If we dissolve immediately, the image disappears on load.
                // Let's keep the background as is, and dissolve it over time or when agent leaves?
                // For now, keep background intact.
            }
        }

        println!("Spawned {} agents", agents.len());

        World {
            agents,
            background,
            width,
            height,
            target: vec2(
                config.target_x * width as f32,
                config.target_y * height as f32,
            ),
            speed: config.agent_speed,
            dissolve_rate: config.dissolve_rate,
        }
    }

    pub fn update(&mut self) {
        let target = self.target;
        let speed = self.speed;
        let width = self.width as f32;
        let height = self.height as f32;
        let dissolve = self.dissolve_rate;

        // Update agents
        self.agents.par_iter_mut().for_each(|agent| {
            if !agent.active {
                return;
            }

            // Seek Target
            let to_target = target - agent.pos;
            let dist = to_target.length();

            let desire = if dist > 0.0 {
                to_target.normalize() * speed
            } else {
                vec2(0.0, 0.0)
            };

            // Steering
            let steer = (desire - agent.vel).clamp_length_max(0.1);
            agent.vel = (agent.vel + steer).clamp_length_max(speed);
            agent.pos += agent.vel;

            // Simple noise/wander
            let mut rng = ::rand::thread_rng();
            agent.vel += vec2(rng.gen_range(-0.1..0.1), rng.gen_range(-0.1..0.1));

            // Bounds
            agent.pos.x = agent.pos.x.clamp(0.0, width);
            agent.pos.y = agent.pos.y.clamp(0.0, height);
        });

        // Dissolve background (fade out alpha randomly)
        // This simulates the image breaking apart
        if dissolve > 0.0 {
            let chunk_size = self.width * 4;
            self.background.par_chunks_mut(chunk_size).for_each(|row| {
                let mut rng = ::rand::thread_rng();
                for i in (0..row.len()).step_by(4) {
                    if row[i + 3] > 0 && rng.gen_bool(dissolve as f64) {
                        row[i + 3] = row[i + 3].saturating_sub(10);
                    }
                }
            });
        }
    }

    pub fn render_to_buffer(&self, buffer: &mut [u8]) {
        // First copy background
        // Ideally we would blend, but simple copy is fast
        // buffer.copy_from_slice(&self.background);
        // Use parallel copy if large
        buffer
            .par_iter_mut()
            .zip(self.background.par_iter())
            .for_each(|(dst, src)| {
                *dst = *src;
            });

        // Draw agents
        // This is sequential or needs mutex if parallel, let's do sequential for simplicity
        // Or parallel with atomic operations? No, additive blending needs care.
        // Sequential is fine for ~20k agents usually.

        for agent in &self.agents {
            if !agent.active {
                continue;
            }

            let px = agent.pos.x as usize;
            let py = agent.pos.y as usize;

            if px < self.width && py < self.height {
                let idx = (py * self.width + px) * 4;

                // Simple alpha blending or overwrite?
                // Let's overwrite with agent color, but respect agent alpha?
                // Agents are solid pixels of the original image

                let [r, g, b, a] = agent.color;

                buffer[idx] = r;
                buffer[idx + 1] = g;
                buffer[idx + 2] = b;
                buffer[idx + 3] = a;
            }
        }
    }
}
