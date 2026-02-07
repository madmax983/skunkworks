use chimera_lang::vm::ChimeraVM;
use glam::Vec2;
use ratatui::style::Color;
use std::collections::VecDeque;

pub const G: f32 = 1.0;

pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub color: Color,
    pub trail: VecDeque<Vec2>,
    pub vm: Option<ChimeraVM>,
    pub name: String,
}

pub struct Universe {
    pub bodies: Vec<Body>,
}

impl Universe {
    pub fn new() -> Self {
        Self { bodies: Vec::new() }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    fn calculate_forces(&self) -> Vec<Vec2> {
        let n = self.bodies.len();
        let mut forces = vec![Vec2::ZERO; n];
        for i in 0..n {
            for j in (i + 1)..n {
                let r_vec = self.bodies[j].pos - self.bodies[i].pos;
                let r_sq = r_vec.length_squared();
                if r_sq > 1e-4 {
                    let r_mag = r_sq.sqrt();
                    let f_mag = G * self.bodies[i].mass * self.bodies[j].mass / r_sq;
                    let f_vec = r_vec / r_mag * f_mag;
                    forces[i] += f_vec;
                    forces[j] -= f_vec;
                }
            }
        }
        forces
    }

    pub fn step(&mut self, dt: f32) {
        let n = self.bodies.len();

        // 1. Calculate initial forces
        let forces = self.calculate_forces();

        // 2. First half-kick (velocity) and drift (position)
        for i in 0..n {
            let acc = forces[i] / self.bodies[i].mass;
            self.bodies[i].vel += acc * 0.5 * dt;
            let vel = self.bodies[i].vel;
            self.bodies[i].pos += vel * dt;

            // Update trail
            if self.bodies[i].trail.len() >= 200 {
                self.bodies[i].trail.pop_front();
            }
            let pos = self.bodies[i].pos;
            self.bodies[i].trail.push_back(pos);
        }

        // 3. Calculate final forces (at new positions)
        let new_forces = self.calculate_forces();

        // 4. Second half-kick (velocity)
        for i in 0..n {
            let acc = new_forces[i] / self.bodies[i].mass;
            self.bodies[i].vel += acc * 0.5 * dt;
        }
    }

    pub fn total_energy(&self) -> f32 {
        let mut kinetic = 0.0;
        let mut potential = 0.0;

        for (i, body) in self.bodies.iter().enumerate() {
            kinetic += 0.5 * body.mass * body.vel.length_squared();

            for (j, other) in self.bodies.iter().enumerate() {
                if i >= j {
                    continue;
                }
                let r = body.pos.distance(other.pos);
                if r > 1e-6 {
                    potential -= G * body.mass * other.mass / r;
                }
            }
        }
        kinetic + potential
    }
}
