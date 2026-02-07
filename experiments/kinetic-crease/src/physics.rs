use nalgebra::Vector3;

#[derive(Clone, Debug)]
pub struct Particle {
    pub pos: Vector3<f32>,
    pub prev_pos: Vector3<f32>,
    pub inv_mass: f32,
}

impl Particle {
    pub fn new(x: f32, y: f32, z: f32, mass: f32) -> Self {
        let pos = Vector3::new(x, y, z);
        let inv_mass = if mass > 0.0 { 1.0 / mass } else { 0.0 };
        Self {
            pos,
            prev_pos: pos,
            inv_mass,
        }
    }
}

#[derive(Clone, Copy, Debug)]
pub enum Constraint {
    Distance {
        p1: usize,
        p2: usize,
        rest_len: f32,
        stiffness: f32,
    },
    Hinge {
        p1: usize, // Wing vertex 1
        p2: usize, // Wing vertex 2
        target_len: f32,
        stiffness: f32,
        // We store the original configuration to recalculate target_len
        // but for now, we just update target_len directly
    },
}

pub struct Solver {
    pub particles: Vec<Particle>,
    pub constraints: Vec<Constraint>,
    pub gravity: Vector3<f32>,
}

impl Solver {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            constraints: Vec::new(),
            gravity: Vector3::new(0.0, -9.81, 0.0),
        }
    }

    pub fn add_particle(&mut self, p: Particle) -> usize {
        let id = self.particles.len();
        self.particles.push(p);
        id
    }

    pub fn add_distance_constraint(&mut self, p1: usize, p2: usize, stiffness: f32) {
        let dist = (self.particles[p1].pos - self.particles[p2].pos).magnitude();
        self.constraints.push(Constraint::Distance {
            p1,
            p2,
            rest_len: dist,
            stiffness,
        });
    }

    pub fn add_hinge_constraint(&mut self, p1: usize, p2: usize, stiffness: f32) {
        let dist = (self.particles[p1].pos - self.particles[p2].pos).magnitude();
        self.constraints.push(Constraint::Hinge {
            p1,
            p2,
            target_len: dist,
            stiffness,
        });
    }


    pub fn step(&mut self, dt: f32, substeps: usize) {
        let dt_sub = dt / substeps as f32;

        for _ in 0..substeps {
            // Predict Positions (Verlet)
            for p in &mut self.particles {
                if p.inv_mass == 0.0 {
                    continue;
                }
                // Verlet: pos += (pos - prev_pos) + a * dt^2
                let vel = p.pos - p.prev_pos;
                p.prev_pos = p.pos;
                // Add gravity and damping
                let damping = 0.99;
                p.pos += vel * damping + self.gravity * dt_sub * dt_sub;
            }

            // Resolve Constraints
            self.solve_constraints_iter(5);
        }
    }

    fn solve_constraints_iter(&mut self, iterations: usize) {
        let particles = &mut self.particles;
        let constraints = &self.constraints;

        for _ in 0..iterations {
            for c in constraints {
                match c {
                    Constraint::Distance {
                        p1,
                        p2,
                        rest_len,
                        stiffness,
                    } => {
                        solve_distance(particles, *p1, *p2, *rest_len, *stiffness);
                    }
                    Constraint::Hinge {
                        p1,
                        p2,
                        target_len,
                        stiffness,
                    } => {
                        solve_distance(particles, *p1, *p2, *target_len, *stiffness);
                    }
                }
            }
        }
    }
}

fn solve_distance(particles: &mut [Particle], p1_idx: usize, p2_idx: usize, target_len: f32, stiffness: f32) {
    let p1 = particles[p1_idx].pos;
    let p2 = particles[p2_idx].pos;
    let w1 = particles[p1_idx].inv_mass;
    let w2 = particles[p2_idx].inv_mass;

    if w1 + w2 == 0.0 {
        return;
    }

    let delta = p1 - p2;
    let dist = delta.magnitude();

    if dist < 1e-6 {
        return;
    }

    let diff = (dist - target_len) / dist;
    let correction = delta * diff * stiffness / (w1 + w2);

    if w1 > 0.0 {
        particles[p1_idx].pos -= correction * w1;
    }
    if w2 > 0.0 {
        particles[p2_idx].pos += correction * w2;
    }
}
