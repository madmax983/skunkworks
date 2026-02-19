use macroquad::prelude::*;

#[derive(Debug, Clone, Copy)]
pub struct Particle {
    pub pos: Vec3,
    pub prev_pos: Vec3,
    pub inv_mass: f32,
    pub vel: Vec3,
}

#[derive(Debug, Clone, Copy)]
pub enum Constraint {
    Distance {
        p1: usize,
        p2: usize,
        rest_length: f32,
        stiffness: f32,
    },
    // Actuator controls the distance between two points (usually wing tips of a hinge)
    // factor: 0.0 = min_len, 1.0 = max_len
    Actuator {
        p1: usize,
        p2: usize,
        min_len: f32,
        max_len: f32,
        factor: f32,
        stiffness: f32,
    },
    // Pins a particle to a specific position (e.g. for dragging or anchoring)
    Pin {
        p: usize,
        pos: Vec3,
    },
}

#[derive(Clone)]
pub struct PbdSystem {
    pub particles: Vec<Particle>,
    pub constraints: Vec<Constraint>,
}

impl Default for PbdSystem {
    fn default() -> Self {
        Self::new()
    }
}

impl PbdSystem {
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            constraints: Vec::new(),
        }
    }

    /// Adds a particle to the system.
    ///
    /// # Arguments
    /// * `pos` - Initial position.
    /// * `mass` - Mass of the particle. If 0.0, the particle is static (infinite mass).
    ///
    /// # Returns
    /// The index of the added particle.
    ///
    /// # Example
    /// ```
    /// use physics_pbd::PbdSystem;
    /// use macroquad::prelude::Vec3;
    ///
    /// let mut system = PbdSystem::new();
    /// let idx = system.add_particle(Vec3::new(0.0, 10.0, 0.0), 1.0);
    /// assert_eq!(idx, 0);
    /// ```
    pub fn add_particle(&mut self, pos: Vec3, mass: f32) -> usize {
        let idx = self.particles.len();
        self.particles.push(Particle {
            pos,
            prev_pos: pos,
            inv_mass: if mass == 0.0 { 0.0 } else { 1.0 / mass },
            vel: Vec3::ZERO,
        });
        idx
    }

    /// Adds a distance constraint between two particles.
    ///
    /// The rest length is automatically calculated based on the current distance between the particles.
    ///
    /// # Example
    /// ```
    /// use physics_pbd::PbdSystem;
    /// use macroquad::prelude::Vec3;
    ///
    /// let mut system = PbdSystem::new();
    /// let p1 = system.add_particle(Vec3::ZERO, 1.0);
    /// let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);
    /// system.add_distance_constraint(p1, p2, 0.5);
    /// ```
    pub fn add_distance_constraint(&mut self, p1: usize, p2: usize, stiff: f32) {
        let dist = self.particles[p1].pos.distance(self.particles[p2].pos);
        self.constraints.push(Constraint::Distance {
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
        self.constraints.push(Constraint::Actuator {
            p1,
            p2,
            min_len,
            max_len,
            factor: 1.0, // Start fully extended
            stiffness: stiff,
        });
    }

    pub fn add_pin_constraint(&mut self, p: usize, pos: Vec3) {
        self.constraints.push(Constraint::Pin { p, pos });
    }

    /// Advances the simulation by `dt` seconds, applying integration and resolving constraints.
    ///
    /// This method uses a Position Based Dynamics (PBD) approach.
    /// Optimization note: The constraint solver loop iterates directly over constraints and uses a
    /// split-borrow of particles to avoid repeated array indexing and `self` borrowing overhead,
    /// significantly improving performance on large systems.
    ///
    /// # Example
    /// ```
    /// use physics_pbd::PbdSystem;
    ///
    /// let mut system = PbdSystem::new();
    /// system.step(0.016, 10);
    /// ```
    pub fn step(&mut self, dt: f32, iterations: usize) {
        // Integrate
        for p in &mut self.particles {
            if p.inv_mass == 0.0 {
                continue;
            }
            // p.vel += Vec3::ZERO * dt; // No gravity for space simulation
            p.prev_pos = p.pos;
            p.pos += p.vel * dt;
        }

        // Constraints
        let particles = &mut self.particles;
        let constraints = &self.constraints;

        for _ in 0..iterations {
            for constraint in constraints {
                match constraint {
                    Constraint::Distance {
                        p1,
                        p2,
                        rest_length,
                        stiffness,
                    } => {
                        Self::solve_distance(particles, *p1, *p2, *rest_length, *stiffness);
                    }
                    Constraint::Actuator {
                        p1,
                        p2,
                        min_len,
                        max_len,
                        factor,
                        stiffness,
                    } => {
                        let target_len = min_len + (max_len - min_len) * factor;
                        Self::solve_distance(particles, *p1, *p2, target_len, *stiffness);
                    }
                    Constraint::Pin { p, pos } => {
                        // Hard constraint: set position directly
                        // But we should respect inv_mass = 0 if it's static?
                        // Pin usually overrides dynamics.
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
            p.vel *= 0.95;
        }
    }

    fn solve_distance(
        particles: &mut [Particle],
        p1: usize,
        p2: usize,
        target_len: f32,
        stiffness: f32,
    ) {
        let pos1 = particles[p1].pos;
        let pos2 = particles[p2].pos;
        let w1 = particles[p1].inv_mass;
        let w2 = particles[p2].inv_mass;
        if w1 + w2 == 0.0 {
            return;
        }

        let delta = pos1 - pos2;
        let len = delta.length();
        if len == 0.0 {
            return;
        } // Avoid division by zero

        let diff = (len - target_len) / len;
        let correction = delta * diff * stiffness / (w1 + w2);

        if w1 > 0.0 {
            particles[p1].pos -= correction * w1;
        }
        if w2 > 0.0 {
            particles[p2].pos += correction * w2;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn bench_pbd_step() {
        let mut system = PbdSystem::new();
        let count = 2000;
        let start_pos = Vec3::new(0.0, 0.0, 0.0);

        // Add chain of particles
        let mut prev = system.add_particle(start_pos, 0.0); // Fixed anchor
        for i in 1..count {
            let pos = start_pos + Vec3::new(i as f32, 0.0, 0.0);
            let p = system.add_particle(pos, 1.0);
            system.add_distance_constraint(prev, p, 1.0);
            prev = p;
        }

        let start = std::time::Instant::now();
        for _ in 0..1000 {
            system.step(0.016, 10);
        }
        let elapsed = start.elapsed();
        println!("Time taken: {:?}", elapsed);
    }

    #[test]
    fn test_add_particle() {
        let mut system = PbdSystem::new();
        let pos = Vec3::new(10.0, 5.0, 0.0);
        let idx = system.add_particle(pos, 10.0);

        assert_eq!(idx, 0);
        assert_eq!(system.particles.len(), 1);
        assert_eq!(system.particles[0].pos, pos);
        assert!((system.particles[0].inv_mass - 0.1).abs() < f32::EPSILON);
    }

    #[test]
    fn test_integration() {
        let mut system = PbdSystem::new();
        let start_pos = Vec3::new(0.0, 0.0, 0.0);
        let idx = system.add_particle(start_pos, 1.0);

        // Set velocity manually
        system.particles[idx].vel = Vec3::new(1.0, 0.0, 0.0);

        // Step simulation
        let dt = 1.0;
        system.step(dt, 1);

        // New position should be approx (1.0, 0.0, 0.0)
        // Note: Logic is p.pos += p.vel * dt
        let expected = Vec3::new(1.0, 0.0, 0.0);
        let actual = system.particles[idx].pos;

        assert!((actual.x - expected.x).abs() < 1e-5);
    }

    #[test]
    fn test_distance_constraint() {
        let mut system = PbdSystem::new();

        // Two particles at distance 2.0
        let p1 = system.add_particle(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let p2 = system.add_particle(Vec3::new(2.0, 0.0, 0.0), 1.0);

        // Constrain them to distance 1.0
        system.constraints.push(Constraint::Distance {
            p1,
            p2,
            rest_length: 1.0,
            stiffness: 1.0,
        });

        // Step
        system.step(0.1, 10);

        let dist = system.particles[p1].pos.distance(system.particles[p2].pos);
        // They should have moved closer to 1.0
        assert!(dist < 2.0);
        assert!((dist - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_pin_constraint() {
        let mut system = PbdSystem::new();
        let pos = Vec3::new(5.0, 5.0, 0.0);
        let p1 = system.add_particle(pos, 1.0);

        // Pin it to (0,0,0)
        system.add_pin_constraint(p1, Vec3::ZERO);

        system.step(0.1, 5);

        // Should be at (0,0,0)
        assert_eq!(system.particles[p1].pos, Vec3::ZERO);
    }

    #[test]
    #[should_panic]
    fn test_invalid_indices_panic() {
        let mut system = PbdSystem::new();
        let p1 = system.add_particle(Vec3::ZERO, 1.0);

        // Add constraint with invalid index
        system.constraints.push(Constraint::Distance {
            p1,
            p2: 999,
            rest_length: 1.0,
            stiffness: 1.0,
        });

        system.step(0.1, 1);
    }

    #[test]
    fn test_zero_mass() {
        let mut system = PbdSystem::new();
        let p1 = system.add_particle(Vec3::ZERO, 0.0); // Infinite mass

        system.particles[p1].vel = Vec3::new(100.0, 0.0, 0.0);

        system.step(1.0, 1);

        // Should not move
        assert_eq!(system.particles[p1].pos, Vec3::ZERO);
    }
}
