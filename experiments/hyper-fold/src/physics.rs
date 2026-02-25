use hyper_system::math::Vec4;

#[derive(Debug, Clone, Copy)]
pub struct Particle4D {
    pub pos: Vec4,
    pub prev_pos: Vec4,
    pub inv_mass: f32,
    pub vel: Vec4,
    pub magnetic_polarity: Vec4,
}

#[derive(Debug, Clone, Copy)]
pub enum Constraint4D {
    Distance {
        p1: usize,
        p2: usize,
        rest_length: f32,
        stiffness: f32,
    },
    Actuator {
        p1: usize,
        p2: usize,
        min_len: f32,
        max_len: f32,
        factor: f32,
        stiffness: f32,
    },
    Pin {
        p: usize,
        pos: Vec4,
    },
}

pub struct PbdSystem4D {
    pub particles: Vec<Particle4D>,
    pub constraints: Vec<Constraint4D>,
    pub grid_size: (usize, usize),
}

impl PbdSystem4D {
    pub fn new(grid_size: (usize, usize)) -> Self {
        Self {
            particles: Vec::new(),
            constraints: Vec::new(),
            grid_size,
        }
    }

    pub fn add_particle(&mut self, pos: Vec4, mass: f32) -> usize {
        let idx = self.particles.len();
        self.particles.push(Particle4D {
            pos,
            prev_pos: pos,
            inv_mass: if mass == 0.0 { 0.0 } else { 1.0 / mass },
            vel: Vec4::zero(),
            magnetic_polarity: Vec4::zero(),
        });
        idx
    }

    pub fn add_distance_constraint(&mut self, p1: usize, p2: usize, stiff: f32) {
        if p1 >= self.particles.len() || p2 >= self.particles.len() {
            return;
        }
        let dist = self.particles[p1]
            .pos
            .distance_squared(self.particles[p2].pos)
            .sqrt();
        self.constraints.push(Constraint4D::Distance {
            p1,
            p2,
            rest_length: dist,
            stiffness: stiff,
        });
    }

    pub fn add_actuator_constraint(
        &mut self,
        p1: usize,
        p2: usize,
        min_len: f32,
        max_len: f32,
        stiff: f32,
    ) {
        self.constraints.push(Constraint4D::Actuator {
            p1,
            p2,
            min_len,
            max_len,
            factor: 0.5,
            stiffness: stiff,
        });
    }

    pub fn add_pin_constraint(&mut self, p: usize, pos: Vec4) {
        self.constraints.push(Constraint4D::Pin { p, pos });
    }

    pub fn apply_magnetic_forces(&mut self, dt: f32) {
        let n = self.particles.len();
        // Naive O(N^2) magnetic force calculation
        // Force formula: F_ij = k * (P_i . P_j) * (Pos_j - Pos_i).normalized() / dist^2
        // Wait, parallel spins attract? Yes, ferromagnetic domains align.
        // Actually, let's say "Hyper-Magnetism": Aligned vectors attract, Opposite repel.
        // Force magnitude depends on alignment (dot product).

        let strength = 5.0; // Calibration needed

        // Use a temporary buffer to accumulate forces to avoid double-mutable borrow issues if we did it in place
        // But since we modify velocity, we can just iterate.
        // Actually, we need to read all positions/polarities to compute forces on one particle.
        // So we can't mutate velocities while reading.
        // We'll compute forces first.

        let mut forces = vec![Vec4::zero(); n];

        for i in 0..n {
            for j in (i + 1)..n {
                let p_i = &self.particles[i];
                let p_j = &self.particles[j];

                let diff = p_j.pos - p_i.pos;
                let dist_sq = diff.length_squared();

                if dist_sq < 0.01 { continue; } // Avoid singularity

                let dist = dist_sq.sqrt();
                let dir = diff / dist; // Direction from i to j

                // Alignment factor: -1.0 (Opposite) to 1.0 (Parallel)
                // We want Parallel to Attract (Force towards j for i)
                // Opposite to Repel (Force away from j for i)
                let p_i_mag = p_i.magnetic_polarity;
                let p_j_mag = p_j.magnetic_polarity;
                let alignment = p_i_mag.x * p_j_mag.x + p_i_mag.y * p_j_mag.y + p_i_mag.z * p_j_mag.z + p_i_mag.w * p_j_mag.w;

                // Force magnitude scales with alignment and inverse square distance
                let force_mag = strength * alignment / dist_sq;
                let force = dir.scale(force_mag);

                forces[i] = forces[i] + force;
                forces[j] = forces[j] - force; // Newton's 3rd law
            }
        }

        // Apply forces to velocity
        for (i, force) in forces.iter().enumerate() {
            let p = &mut self.particles[i];
            if p.inv_mass > 0.0 {
                p.vel = p.vel + force.scale(dt * p.inv_mass);
            }
        }
    }

    pub fn step(&mut self, dt: f32, iterations: usize) {
        if dt <= f32::EPSILON {
            return;
        }

        // Apply Forces first (Magnetism)
        self.apply_magnetic_forces(dt);

        // Integrate
        for p in &mut self.particles {
            if p.inv_mass == 0.0 {
                continue;
            }
            // Damping
            p.vel = p.vel.scale(0.99);

            p.prev_pos = p.pos;
            p.pos = p.pos + p.vel.scale(dt);

            // Floor constraint (Y > -5.0)? Or maybe let it float.
            // Let's add a simple bounds check to keep it visible
            if p.pos.y < -10.0 { p.pos.y = -10.0; }
        }

        // Constraints
        let particles = &mut self.particles;
        let constraints = &self.constraints;

        for _ in 0..iterations {
            for constraint in constraints {
                match constraint {
                    Constraint4D::Distance {
                        p1,
                        p2,
                        rest_length,
                        stiffness,
                    } => {
                        Self::solve_distance(particles, *p1, *p2, *rest_length, *stiffness);
                    }
                    Constraint4D::Actuator {
                        p1,
                        p2,
                        min_len,
                        max_len,
                        factor,
                        stiffness,
                    } => {
                        let target = min_len + (max_len - min_len) * factor;
                        Self::solve_distance(particles, *p1, *p2, target, *stiffness);
                    }
                    Constraint4D::Pin { p, pos } => {
                        if let Some(particle) = particles.get_mut(*p) {
                            particle.pos = *pos;
                        }
                    }
                }
            }
        }

        // Update Velocity
        for p in &mut self.particles {
            if p.inv_mass == 0.0 {
                continue;
            }
            p.vel = (p.pos - p.prev_pos) / dt;
        }
    }

    fn solve_distance(
        particles: &mut [Particle4D],
        p1: usize,
        p2: usize,
        target_len: f32,
        stiffness: f32,
    ) {
        if p1 >= particles.len() || p2 >= particles.len() {
            return;
        }

        let (pos1, w1) = {
            let p = &particles[p1];
            (p.pos, p.inv_mass)
        };
        let (pos2, w2) = {
            let p = &particles[p2];
            (p.pos, p.inv_mass)
        };

        if (w1 + w2).abs() < f32::EPSILON {
            return;
        }

        let delta = pos1 - pos2;
        let len = delta.length();

        if len < f32::EPSILON {
            return;
        }

        let diff = (len - target_len) / len;
        let correction = delta.scale(diff * stiffness / (w1 + w2));

        if w1 > 0.0 {
            particles[p1].pos = particles[p1].pos - correction.scale(w1);
        }
        if w2 > 0.0 {
            particles[p2].pos = particles[p2].pos + correction.scale(w2);
        }
    }
}
