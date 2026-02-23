use crate::fluid::FluidSim;
use crate::grid::{CellType, Grid};
use macroquad::prelude::*;

pub struct Particle {
    pub pos: Vec2,
    pub vel: Vec2,
    pub age: f32,
    pub lifetime: f32,
}

pub struct ParticleSystem {
    pub particles: Vec<Particle>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    pub fn spawn(&mut self, pos: Vec2, count: usize) {
        for _ in 0..count {
            let angle = rand::gen_range(0.0f32, std::f32::consts::PI * 2.0);
            let speed = rand::gen_range(5.0f32, 20.0f32);
            self.particles.push(Particle {
                pos,
                vel: vec2(angle.cos() * speed, angle.sin() * speed - 10.0), // Initial upward bias
                age: 0.0,
                lifetime: rand::gen_range(5.0f32, 10.0f32),
            });
        }
    }

    pub fn update(&mut self, grid: &mut Grid, fluid: &FluidSim, dt: f32) {
        // We need to iterate and remove dead particles.
        // Retain based approach is easiest for removal.
        // But we need to mutate grid, so we can't use retain easily if we need grid access inside closure.
        // We'll iterate manually.

        let mut dead_indices = Vec::new();

        for (i, p) in self.particles.iter_mut().enumerate() {
            p.age += dt;
            if p.age >= p.lifetime {
                dead_indices.push(i);
                continue;
            }

            // Physics
            // Buoyancy: Heat rises. We can use fluid temp to drive velocity.
            let grid_x = (p.pos.x / 8.0) as usize;
            let grid_y = (p.pos.y / 8.0) as usize;

            let temp = fluid.get_temp(grid_x, grid_y);

            // Advection (simplified)
            // Move up faster if hot
            p.vel.y -= (temp * 10.0 + 5.0) * dt;

            // Drag
            p.vel *= 0.98;

            let prev_pos = p.pos;
            p.pos += p.vel * dt;

            // Boundary
            if p.pos.x < 0.0 || p.pos.x > grid.width as f32 * 8.0 || p.pos.y < 0.0 {
                dead_indices.push(i);
                continue;
            }

            // Deposition Check
            let gx = (p.pos.x / 8.0) as usize;
            let gy = (p.pos.y / 8.0) as usize;

            let cell_type = grid.get(gx, gy).map(|c| c.cell_type);

            if let Some(ct) = cell_type {
                if ct == CellType::Rock
                    || ct == CellType::Chimney
                    || matches!(ct, CellType::Vent(_))
                {
                    // Collision!
                    // Deposit at previous position if valid
                    let pgx = (prev_pos.x / 8.0) as usize;
                    let pgy = (prev_pos.y / 8.0) as usize;

                    let prev_cell_type = grid.get(pgx, pgy).map(|c| c.cell_type);

                    if let Some(pct) = prev_cell_type {
                        if pct == CellType::Water {
                            grid.set_type(pgx, pgy, CellType::Chimney);
                            dead_indices.push(i); // Particle becomes rock
                        } else {
                            // If previous was also solid (weird tunneling?), just die
                            dead_indices.push(i);
                        }
                    }
                }
            }
        }

        // Remove dead particles (reverse order to keep indices valid)
        for idx in dead_indices.into_iter().rev() {
            self.particles.swap_remove(idx);
        }
    }

    pub fn draw(&self) {
        for p in &self.particles {
            draw_rectangle(p.pos.x, p.pos.y, 2.0, 2.0, Color::new(0.2, 0.2, 0.2, 0.8));
        }
    }
}
