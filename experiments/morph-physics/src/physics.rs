use crate::phonology::{Phoneme};
use macroquad::prelude::Vec2;

pub struct Particle {
    pub phoneme: Phoneme,
    pub pos: Vec2,
    pub vel: Vec2,
    pub force: Vec2,
    pub mass: f32,
}

pub struct World {
    pub particles: Vec<Particle>,
    pub springs: Vec<(usize, usize, f32)>, // (idx1, idx2, rest_length)
    pub friction: f32,
    pub repulsion_strength: f32,
    pub spring_k: f32,
}

impl World {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            springs: Vec::new(),
            friction: 0.90,
            repulsion_strength: 2000.0,
            spring_k: 2.0,
        }
    }

    pub fn add_particle(&mut self, p: Phoneme, pos: Vec2) {
        self.particles.push(Particle {
            phoneme: p,
            pos,
            vel: Vec2::ZERO,
            force: Vec2::ZERO,
            mass: 1.0,
        });
    }

    pub fn add_spring(&mut self, idx1: usize, idx2: usize, length: f32) {
        self.springs.push((idx1, idx2, length));
    }

    pub fn update(&mut self, dt: f32) {
        // Reset forces
        for p in &mut self.particles {
            p.force = Vec2::ZERO;
        }

        // Apply Spring Forces
        for &(i, j, len) in &self.springs {
            // Check bounds just in case
            if i >= self.particles.len() || j >= self.particles.len() { continue; }

            let p1_pos = self.particles[i].pos;
            let p2_pos = self.particles[j].pos;
            let delta = p2_pos - p1_pos;
            let dist = delta.length();
            if dist > 0.001 {
                let dir = delta / dist;
                let displacement = dist - len;
                let force = dir * (displacement * self.spring_k);
                self.particles[i].force += force;
                self.particles[j].force -= force;
            }
        }

        // Apply Repulsion (Coulomb-like)
        let count = self.particles.len();
        for i in 0..count {
            for j in (i + 1)..count {
                let delta = self.particles[j].pos - self.particles[i].pos;
                let dist_sq = delta.length_squared();
                if dist_sq < 2500.0 && dist_sq > 0.001 { // Radius 50 interaction
                    let dist = dist_sq.sqrt();
                    let dir = delta / dist;
                    let force = -dir * (self.repulsion_strength / dist);
                    self.particles[i].force += force;
                    self.particles[j].force -= force;
                }
            }
        }

        // Integration (Euler)
        for p in &mut self.particles {
            let acc = p.force / p.mass;
            p.vel += acc * dt;
            p.vel *= self.friction;
            p.pos += p.vel * dt;
        }
    }
}
