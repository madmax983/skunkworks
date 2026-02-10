use macroquad::prelude::*;

pub const G: f32 = 50.0;
pub const SOFTENING: f32 = 10.0;

#[derive(Clone, Debug)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub char: char,
    pub color: Color,
}

impl Body {
    pub fn new(pos: Vec2, vel: Vec2, mass: f32, char: char, color: Color) -> Self {
        Self {
            pos,
            vel,
            mass,
            char,
            color,
        }
    }
}

pub struct Universe {
    pub bodies: Vec<Body>,
    pub black_hole_pos: Vec2,
    pub black_hole_mass: f32,
}

impl Universe {
    pub fn new() -> Self {
        Self {
            bodies: Vec::new(),
            black_hole_pos: Vec2::ZERO,
            black_hole_mass: 10000.0,
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

        let mut acc = vec![Vec2::ZERO; n];

        for i in 0..n {
            // Attraction to Black Hole
            let r = self.black_hole_pos - self.bodies[i].pos;
            let dist_sq = r.length_squared() + SOFTENING * SOFTENING;
            // F = G * M * m / r^2
            // a = F / m = G * M / r^2
            let a_bh = r.normalize_or_zero() * (G * self.black_hole_mass / dist_sq);
            acc[i] += a_bh;

            // Body-Body interactions (N-body)
            // Limit N-body interactions if too many bodies to maintain performance
            // For now, let's just do full N^2 up to a limit, or optimize
            // If N < 500, N^2 is fine (~250k iterations per tick)
            for j in 0..n {
                if i == j {
                    continue;
                }
                let r = self.bodies[j].pos - self.bodies[i].pos;
                let dist_sq = r.length_squared() + SOFTENING * SOFTENING;
                let a_body = r.normalize_or_zero() * (G * self.bodies[j].mass / dist_sq);
                acc[i] += a_body;
            }
        }

        // Symplectic Euler Integration
        for i in 0..n {
            self.bodies[i].vel += acc[i] * dt;
            let vel = self.bodies[i].vel;
            self.bodies[i].pos += vel * dt;

            // Drag/Friction to prevent exploding energy from integration errors or close encounters
            self.bodies[i].vel *= 0.999;
        }

        // Remove bodies that fall into the event horizon (optional)
        // For now, just let them slingshot
    }
}
