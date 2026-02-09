use macroquad::prelude::Vec2;
use std::collections::VecDeque;

#[derive(Debug, Clone)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub radius: f32,
    pub name: String,
    pub history: VecDeque<Vec2>,
    pub parent_id: Option<usize>,
}

#[allow(non_snake_case)]
pub struct Universe {
    pub bodies: Vec<Body>,
    pub G: f32,
}

impl Default for Universe {
    fn default() -> Self {
        Self::new()
    }
}

impl Universe {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            G: 100.0,
        }
    }

    pub fn add_body(&mut self, body: Body) {
        self.bodies.push(body);
    }

    pub fn step(&mut self, dt: f32) {
        let n = self.bodies.len();
        if n == 0 {
            return;
        }

        let mut forces = vec![Vec2::ZERO; n];

        // 1. Calculate Forces (Gravity)
        for i in 0..n {
            for j in (i + 1)..n {
                let r = self.bodies[j].pos - self.bodies[i].pos;
                let dist_sq = r.length_squared();
                let dist = dist_sq.sqrt();

                if dist < 0.1 {
                    continue; // Softening / Collision avoidance
                }

                let f_mag = (self.G * self.bodies[i].mass * self.bodies[j].mass) / dist_sq;
                let f_dir = r / dist;

                let f = f_dir * f_mag;

                forces[i] += f;
                forces[j] -= f;
            }
        }

        // 2. Symplectic Euler Integration
        for (i, body) in self.bodies.iter_mut().enumerate() {
            // F = ma => a = F/m
            let acc = forces[i] / body.mass;

            // v = v + a * dt
            body.vel += acc * dt;

            // x = x + v * dt
            body.pos += body.vel * dt;

            // Update history
            body.history.push_back(body.pos);
            if body.history.len() > 1000 {
                body.history.pop_front();
            }
        }
    }
}
