use crate::fluid::FluidSim;
use macroquad::prelude::*;

pub struct TextParticle {
    pub position: Vec2,
    pub char: char,
    pub color: Color,
    pub lifetime: f32,
    pub max_lifetime: f32,
}

pub struct ParticleSystem {
    pub particles: Vec<TextParticle>,
}

impl ParticleSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
        }
    }

    pub fn spawn(&mut self, x: f32, y: f32, char: char, color: Color) {
        self.particles.push(TextParticle {
            position: vec2(x, y),
            char,
            color,
            lifetime: 0.0,
            max_lifetime: 10.0 + rand::gen_range(0.0, 5.0),
        });
    }

    pub fn update(&mut self, sim: &FluidSim, dt: f32) {
        // Remove dead particles
        self.particles.retain(|p| p.lifetime < p.max_lifetime);

        for p in &mut self.particles {
            // Advect using RK2 (Runge-Kutta 2nd order) for better stability
            // k1 = v(x)
            // k2 = v(x + k1 * 0.5 * dt)
            // x_new = x + k2 * dt

            let (ux1, uy1) = sim.get_velocity(p.position.x, p.position.y);
            let k1 = vec2(ux1, uy1);

            // Assume 60 FPS target for dt scaling
            let time_scale = dt * 60.0;

            let mid_pos = p.position + k1 * 0.5 * time_scale;
            let (ux2, uy2) = sim.get_velocity(mid_pos.x, mid_pos.y);
            let k2 = vec2(ux2, uy2);

            p.position += k2 * time_scale;
            p.lifetime += dt;
        }
    }

    pub fn particles(&self) -> &Vec<TextParticle> {
        &self.particles
    }

    pub fn count(&self) -> usize {
        self.particles.len()
    }
}
