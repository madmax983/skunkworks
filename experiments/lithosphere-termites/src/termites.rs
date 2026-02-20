use crate::terrain::Terrain;
use rand::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Termite {
    pub x: f32,
    pub y: f32,
    pub vx: f32,
    pub vy: f32,
    pub carrying: f32,
}

impl Termite {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            vx: 0.0,
            vy: 0.0,
            carrying: 0.0,
        }
    }
}

pub struct TermiteColony {
    pub agents: Vec<Termite>,
}

impl TermiteColony {
    pub fn new(count: usize, width: f32, height: f32) -> Self {
        let mut rng = rand::thread_rng();
        let mut agents = Vec::with_capacity(count);
        for _ in 0..count {
            agents.push(Termite::new(
                rng.gen_range(0.0..width),
                rng.gen_range(0.0..height),
            ));
        }
        Self { agents }
    }

    pub fn update(&mut self, terrain: &mut Terrain) {
        let width = terrain.width as f32;
        let height = terrain.height as f32;
        let mut rng = rand::thread_rng();

        for agent in &mut self.agents {
            let ix = agent.x as usize;
            let iy = agent.y as usize;

            // 1. Sense Environment
            let (gx, gy) = terrain.get_gradient(ix, iy);

            // 2. Decision Logic
            let max_carry = 0.5;

            if agent.carrying > 0.0 {
                // Carrying: Move Uphill (against gradient) to build mounds
                // Gradient points UPHILL (h1 - h0).
                // Wait, if gradient is positive uphill, we follow it?
                // Let's re-verify gradient direction in terrain.rs.
                // gx = (h10 - h00)...
                // If h10 > h00, gx > 0. So gradient points towards higher neighbor.
                // So to go uphill, follow gradient.

                agent.vx += gx * 0.5;
                agent.vy += gy * 0.5;

                // Random wander
                agent.vx += rng.gen_range(-0.2..0.2);
                agent.vy += rng.gen_range(-0.2..0.2);

                // Deposit Logic
                let h = terrain.get_height(ix, iy);
                let slope = (gx*gx + gy*gy).sqrt();

                let should_deposit = if slope < 0.1 && h > 1.0 {
                    rng.gen_bool(0.1) // Top of hill
                } else if slope > 1.0 {
                    rng.gen_bool(0.01) // Extend slope
                } else {
                    rng.gen_bool(0.001) // Random drop
                };

                if should_deposit {
                    let amount = agent.carrying;
                    terrain.deposit_sediment(ix, iy, amount);
                    agent.carrying = 0.0;
                }

            } else {
                // Empty: Move Downhill (against gradient)
                agent.vx -= gx * 0.5;
                agent.vy -= gy * 0.5;

                // Random wander
                agent.vx += rng.gen_range(-0.2..0.2);
                agent.vy += rng.gen_range(-0.2..0.2);

                // Pick Logic
                let sed = terrain.sediment[terrain.get_index(ix, iy)];
                if sed > 0.1 {
                    // Pick up
                    let amount = terrain.take_sediment(ix, iy, max_carry);
                    agent.carrying = amount;
                }
            }

            // 3. Physics Update
            let speed = (agent.vx*agent.vx + agent.vy*agent.vy).sqrt();
            if speed > 1.0 {
                agent.vx /= speed;
                agent.vy /= speed;
            }

            agent.x += agent.vx;
            agent.y += agent.vy;

            // Friction
            agent.vx *= 0.9;
            agent.vy *= 0.9;

            // Bounds
            if agent.x < 0.0 { agent.x = 0.0; agent.vx *= -1.0; }
            if agent.x >= width { agent.x = width - 0.01; agent.vx *= -1.0; }
            if agent.y < 0.0 { agent.y = 0.0; agent.vy *= -1.0; }
            if agent.y >= height { agent.y = height - 0.01; agent.vy *= -1.0; }
        }
    }
}
