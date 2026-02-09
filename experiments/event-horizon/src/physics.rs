use macroquad::prelude::Vec2;

#[derive(Clone, Copy, Debug)]
pub struct Body {
    pub pos: Vec2,
    pub vel: Vec2,
    pub mass: f32,
    pub radius: f32,
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

    pub fn step(&mut self, dt: f32) {
        let n = self.bodies.len();
        let mut acc = vec![Vec2::ZERO; n];

        // G = 1.0
        let g = 1.0;

        for i in 0..n {
            for j in 0..n {
                if i == j {
                    continue;
                }
                let diff = self.bodies[j].pos - self.bodies[i].pos;
                let dist_sq = diff.length_squared();
                if dist_sq < 0.1 {
                    continue;
                } // Softening
                let dist = dist_sq.sqrt();

                let force_mag = g * self.bodies[j].mass / dist_sq;
                acc[i] += diff / dist * force_mag;
            }
        }

        for i in 0..n {
            self.bodies[i].vel += acc[i] * dt;
            let vel = self.bodies[i].vel;
            self.bodies[i].pos += vel * dt;
        }
    }
}
