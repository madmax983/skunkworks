use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct Particle {
    pub pos: Vec3,
    pub prev_pos: Vec3,
    pub inv_mass: f32,
    pub pinned: bool,
}

impl Particle {
    pub fn new(pos: Vec3, mass: f32) -> Self {
        Self {
            pos,
            prev_pos: pos,
            inv_mass: if mass > 0.0 { 1.0 / mass } else { 0.0 },
            pinned: mass <= 0.0,
        }
    }
}

#[derive(Clone, Copy)]
pub struct DistanceConstraint {
    pub p1: usize,
    pub p2: usize,
    pub rest_length: f32,
    pub compliance: f32,
}

pub struct Solver {
    pub particles: Vec<Particle>,
    pub distance_constraints: Vec<DistanceConstraint>,
    pub sub_steps: usize,
    pub gravity: Vec3,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            distance_constraints: Vec::new(),
            sub_steps: 10,
            gravity: Vec3::ZERO,
        }
    }

    pub fn add_particle(&mut self, pos: Vec3, mass: f32) -> usize {
        let idx = self.particles.len();
        self.particles.push(Particle::new(pos, mass));
        idx
    }

    pub fn add_distance_constraint(&mut self, p1: usize, p2: usize, compliance: f32) -> usize {
        let idx = self.distance_constraints.len();
        let p1_pos = self.particles[p1].pos;
        let p2_pos = self.particles[p2].pos;
        let rest_length = p1_pos.distance(p2_pos);
        self.distance_constraints.push(DistanceConstraint {
            p1,
            p2,
            rest_length,
            compliance,
        });
        idx
    }

    pub fn set_rest_length(&mut self, idx: usize, length: f32) {
        if let Some(c) = self.distance_constraints.get_mut(idx) {
            c.rest_length = length;
        }
    }

    pub fn update(&mut self, dt: f32) {
        let dt_sub = dt / self.sub_steps as f32;

        for _ in 0..self.sub_steps {
            // Predict
            for p in &mut self.particles {
                if p.pinned {
                    continue;
                }
                let vel = p.pos - p.prev_pos;
                // Damping
                let vel = vel * 0.99;

                p.prev_pos = p.pos;
                p.pos += vel + self.gravity * dt_sub * dt_sub;
            }

            // Solve Constraints
            self.solve_distance_constraints(dt_sub);
        }
    }

    fn solve_distance_constraints(&mut self, dt: f32) {
        for c in &self.distance_constraints {
            let p1 = self.particles[c.p1];
            let p2 = self.particles[c.p2];

            let w1 = p1.inv_mass;
            let w2 = p2.inv_mass;
            let w = w1 + w2;
            if w == 0.0 {
                continue;
            }

            let delta = p1.pos - p2.pos;
            let len = delta.length();
            if len == 0.0 {
                continue;
            }

            let n = delta / len;
            let c_val = len - c.rest_length;

            // XPBD: compliance alpha = compliance / dt^2
            let alpha = c.compliance / (dt * dt);

            let lambda = -c_val / (w + alpha);

            let correction = n * lambda;

            if !p1.pinned {
                self.particles[c.p1].pos += correction * w1;
            }
            if !p2.pinned {
                self.particles[c.p2].pos -= correction * w2;
            }
        }
    }
}
