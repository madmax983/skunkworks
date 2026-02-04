use macroquad::prelude::*;

pub const G: f32 = 100.0; // Gravity constant (adjusted for 3D scale)

#[derive(Clone, Copy)]
pub struct Body {
    pub pos: Vec3,
    pub vel: Vec3,
    pub mass: f32,
    pub radius: f32,
    pub color: Color,
    pub base_color: Color,
    pub trail: [Vec3; 50], // Fixed size trail ring buffer
    pub trail_idx: usize,
}

impl Body {
    pub fn new(pos: Vec3, mass: f32, radius: f32, color: Color) -> Self {
        Self {
            pos,
            vel: Vec3::ZERO,
            mass,
            radius,
            color,
            base_color: color,
            trail: [pos; 50],
            trail_idx: 0,
        }
    }

    pub fn with_velocity(mut self, vel: Vec3) -> Self {
        self.vel = vel;
        self
    }
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
        let mut accelerations = vec![Vec3::ZERO; n];

        for i in 0..n {
            for j in 0..n {
                if i == j { continue; }

                let diff = self.bodies[j].pos - self.bodies[i].pos;
                let dist_sq = diff.length_squared();

                // Softening to avoid singularity
                let softened_dist_sq = dist_sq + 10.0;
                let dist = softened_dist_sq.sqrt();

                let f = (G * self.bodies[j].mass) / softened_dist_sq;
                let dir = diff / dist;

                accelerations[i] += dir * f;
            }
        }

        for (i, body) in self.bodies.iter_mut().enumerate() {
            body.vel += accelerations[i] * dt;
            body.pos += body.vel * dt;

            // Update trail every few frames or just every frame?
            // Every frame is smooth.
            body.trail[body.trail_idx] = body.pos;
            body.trail_idx = (body.trail_idx + 1) % body.trail.len();
        }
    }
}
