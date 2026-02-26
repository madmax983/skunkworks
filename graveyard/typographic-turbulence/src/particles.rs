use crate::lbm::{FluidSim, HEIGHT, WIDTH};
use macroquad::color::hsl_to_rgb;
use macroquad::prelude::*;

pub struct TextParticle {
    pub position: Vec2,
    pub char: char,
    pub color: Color,
    pub lifetime: f32,
    pub max_lifetime: f32,
    pub velocity: Vec2,
}

pub struct ParticleSystem {
    pub particles: Vec<TextParticle>,
    pub max_particles: usize,
}

impl ParticleSystem {
    pub fn new(max_particles: usize) -> Self {
        Self {
            particles: Vec::with_capacity(max_particles),
            max_particles,
        }
    }

    pub fn spawn(&mut self, x: f32, y: f32, char: char, color: Color) {
        if self.particles.len() < self.max_particles {
            self.particles.push(TextParticle {
                position: vec2(x, y),
                char,
                color,
                lifetime: 0.0,
                max_lifetime: 5.0 + rand::gen_range(0.0, 5.0),
                velocity: vec2(0.0, 0.0),
            });
        }
    }

    pub fn update(&mut self, sim: &FluidSim, dt: f32) {
        // Respawn logic: Maintain particle count
        let needed = self.max_particles.saturating_sub(self.particles.len());
        let spawn_count = needed.min(50); // Limit spawn per frame

        for _ in 0..spawn_count {
            let y = rand::gen_range(1.0, (HEIGHT - 1) as f32);
            let char = match rand::gen_range(0, 6) {
                0 => '.',
                1 => ',',
                2 => '`',
                3 => '\'',
                4 => '"',
                _ => '~',
            };
            self.spawn(1.0, y, char, WHITE);
        }

        // Update existing particles
        let mut i = 0;
        while i < self.particles.len() {
            let dead = {
                let p = &mut self.particles[i];

                // Advect using RK2
                let (ux1, uy1) = sim.get_velocity(p.position.x, p.position.y);
                let k1 = vec2(ux1, uy1);
                let time_scale = dt * 60.0;

                let mid_pos = p.position + k1 * 0.5 * time_scale;
                let (ux2, uy2) = sim.get_velocity(mid_pos.x, mid_pos.y);
                let k2 = vec2(ux2, uy2);

                p.velocity = k2;
                p.position += k2 * time_scale;
                p.lifetime += dt;

                // Color based on curl and speed
                let curl = sim.get_curl(p.position.x, p.position.y);
                let speed = p.velocity.length();

                let hue = (0.5 + curl * 4.0).clamp(0.0, 1.0);
                let lightness = (0.3 + speed * 1.0).clamp(0.2, 0.9);

                p.color = hsl_to_rgb(hue, 1.0, lightness);

                // Check kill conditions
                p.lifetime > p.max_lifetime
                    || p.position.x > (WIDTH - 1) as f32
                    || p.position.y > (HEIGHT - 1) as f32
                    || p.position.x < 0.0
                    || p.position.y < 0.0
            };

            if dead {
                self.particles.swap_remove(i);
            } else {
                i += 1;
            }
        }
    }

    pub fn particles(&self) -> &Vec<TextParticle> {
        &self.particles
    }

    pub fn count(&self) -> usize {
        self.particles.len()
    }
}
