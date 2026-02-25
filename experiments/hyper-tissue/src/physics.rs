use hyper_system::math::Vec4;

#[derive(Debug, Clone, Copy)]
pub struct Particle4D {
    pub pos: Vec4,
    pub prev_pos: Vec4,
    pub inv_mass: f32,
    pub vel: Vec4,
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
}

impl PbdSystem4D {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            constraints: Vec::new(),
        }
    }

    pub fn add_particle(&mut self, pos: Vec4, mass: f32) -> usize {
        let idx = self.particles.len();
        self.particles.push(Particle4D {
            pos,
            prev_pos: pos,
            inv_mass: if mass == 0.0 { 0.0 } else { 1.0 / mass },
            vel: Vec4::zero(),
        });
        idx
    }

    pub fn add_distance_constraint(&mut self, p1: usize, p2: usize, stiff: f32) {
        let dist = self.particles[p1].pos.distance_squared(self.particles[p2].pos).sqrt();
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
            factor: 1.0,
            stiffness: stiff,
        });
    }

    pub fn add_pin_constraint(&mut self, p: usize, pos: Vec4) {
        self.constraints.push(Constraint4D::Pin { p, pos });
    }

    pub fn step(&mut self, dt: f32, iterations: usize) {
        if dt <= f32::EPSILON {
            return;
        }

        // Integrate
        for p in &mut self.particles {
            if p.inv_mass == 0.0 {
                continue;
            }
            p.prev_pos = p.pos;
            p.pos = p.pos + p.vel.scale(dt);
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
            // Damping
            p.vel = p.vel.scale(0.98);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_integration() {
        let mut system = PbdSystem4D::new();
        let p = system.add_particle(Vec4::zero(), 1.0);
        system.particles[p].vel = Vec4::new(1.0, 0.0, 0.0, 0.0);

        system.step(1.0, 1);

        // Pos should be approx (1.0, 0.0, 0.0, 0.0) * 0.98 damping?
        // Wait, integration happens first: pos = pos + vel * dt
        // Then constraints. Then velocity update: vel = (pos - prev) / dt * 0.98

        let pos = system.particles[p].pos;
        assert!((pos.x - 1.0).abs() < 1e-6);
    }

    #[test]
    fn test_distance_constraint() {
        let mut system = PbdSystem4D::new();
        let p1 = system.add_particle(Vec4::zero(), 1.0);
        let p2 = system.add_particle(Vec4::new(2.0, 0.0, 0.0, 0.0), 1.0);

        // Target length 1.0
        system.constraints.push(Constraint4D::Distance {
            p1, p2, rest_length: 1.0, stiffness: 1.0
        });

        system.step(0.1, 10);

        let dist = system.particles[p1].pos.distance_squared(system.particles[p2].pos).sqrt();
        assert!((dist - 1.0).abs() < 0.1);
    }
}
