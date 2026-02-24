//! # Physics PBD
//!
//! A simple Position Based Dynamics (PBD) physics engine for 2D/3D applications.
//!
//! This crate provides a `PbdSystem` struct that manages particles and constraints.
//! It is designed to be easy to use with `macroquad`, utilizing `glam` types (via `macroquad::prelude::Vec3`).
//!
//! ## Key Concepts
//!
//! - **Particles**: Point masses with position, velocity, and inverse mass.
//! - **Constraints**: Rules that limit the movement of particles (e.g., distance, pinning).
//! - **Solver**: An iterative solver that resolves constraints to simulate physical behavior.
//!
//! ## Example
//!
//! ```
//! use physics_pbd::{PbdSystem, Constraint};
//! use macroquad::prelude::Vec3;
//!
//! let mut system = PbdSystem::new();
//!
//! // Add two particles
//! let p1 = system.add_particle(Vec3::new(0.0, 10.0, 0.0), 1.0);
//! let p2 = system.add_particle(Vec3::new(1.0, 10.0, 0.0), 1.0);
//!
//! // Add a distance constraint
//! system.add_distance_constraint(p1, p2, 0.5);
//!
//! // Simulate
//! system.step(0.016, 5);
//! ```

use macroquad::prelude::*;

/// A point mass in the physics simulation.
#[derive(Debug, Clone, Copy)]
pub struct Particle {
    /// Current position of the particle.
    pub pos: Vec3,
    /// Previous position of the particle (used for Verlet integration).
    pub prev_pos: Vec3,
    /// Inverse mass of the particle (1.0 / mass). 0.0 means infinite mass (static).
    pub inv_mass: f32,
    /// Velocity of the particle.
    pub vel: Vec3,
}

/// A constraint that limits the movement of particles.
#[derive(Debug, Clone, Copy)]
pub enum Constraint {
    /// Constrains two particles to be at a fixed distance from each other.
    Distance {
        /// Index of the first particle.
        p1: usize,
        /// Index of the second particle.
        p2: usize,
        /// The target distance between the particles.
        rest_length: f32,
        /// The stiffness of the constraint (0.0 to 1.0).
        stiffness: f32,
    },
    /// An actuator that changes the distance between two particles based on a factor.
    ///
    /// Useful for simulating muscles, pistons, or motorized hinges.
    Actuator {
        /// Index of the first particle.
        p1: usize,
        /// Index of the second particle.
        p2: usize,
        /// The minimum length of the actuator.
        min_len: f32,
        /// The maximum length of the actuator.
        max_len: f32,
        /// The current extension factor (0.0 = min_len, 1.0 = max_len).
        factor: f32,
        /// The stiffness of the constraint.
        stiffness: f32,
    },
    /// Pins a particle to a specific position in world space.
    ///
    /// Useful for anchoring objects or implementing mouse dragging.
    Pin {
        /// Index of the particle to pin.
        p: usize,
        /// The position to pin the particle to.
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
        assert!(
            mass >= 0.0 && mass.is_finite(),
            "Mass must be non-negative and finite"
        );
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

    /// Adds an actuator constraint between two particles.
    ///
    /// # Example
    /// ```
    /// use physics_pbd::PbdSystem;
    /// use macroquad::prelude::Vec3;
    ///
    /// let mut system = PbdSystem::new();
    /// let p1 = system.add_particle(Vec3::ZERO, 1.0);
    /// let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);
    /// system.add_actuator_constraint(p1, p2, 0.5, 1.5, 1.0);
    /// ```
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

    /// Pins a particle to a specific position.
    ///
    /// # Example
    /// ```
    /// use physics_pbd::PbdSystem;
    /// use macroquad::prelude::Vec3;
    ///
    /// let mut system = PbdSystem::new();
    /// let p = system.add_particle(Vec3::ZERO, 1.0);
    /// system.add_pin_constraint(p, Vec3::new(5.0, 5.0, 5.0));
    /// ```
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
    /// # Panics
    /// Panics if any constraint references a particle index that does not exist.
    ///
    /// # Example
    /// ```
    /// use physics_pbd::PbdSystem;
    ///
    /// let mut system = PbdSystem::new();
    /// system.step(0.016, 10);
    /// ```
    pub fn step(&mut self, dt: f32, iterations: usize) {
        if dt <= f32::EPSILON {
            return;
        }

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
                        if !factor.is_finite() {
                            continue;
                        }
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

    /// Solves a distance constraint between two particles.
    ///
    /// This function is marked `#[inline]` because it is called in a tight loop (iterations * constraints),
    /// and function call overhead can be significant.
    ///
    /// The particle data is accessed using block-scoped re-borrowing to encourage the compiler
    /// to perform a single bounds check per particle, rather than one per field access.
    #[inline]
    fn solve_distance(
        particles: &mut [Particle],
        p1: usize,
        p2: usize,
        target_len: f32,
        stiffness: f32,
    ) {
        if p1 >= particles.len() || p2 >= particles.len() {
            return;
        }

        if !target_len.is_finite() || !stiffness.is_finite() {
            return;
        }

        // Optimization: Access particle data once to minimize bounds checks.
        let (pos1, w1) = {
            let p = &particles[p1];
            (p.pos, p.inv_mass)
        };
        let (pos2, w2) = {
            let p = &particles[p2];
            (p.pos, p.inv_mass)
        };

        if (w1 + w2).abs() < f32::EPSILON || !(w1 + w2).is_finite() {
            return;
        }

        let delta = pos1 - pos2;
        let len = delta.length();
        if !len.is_finite() || len < f32::EPSILON {
            return;
        } // Avoid division by zero and numeric instability

        // Clamp stiffness to ensure stability (0.0 to 1.0)
        let stiffness = stiffness.clamp(0.0, 1.0);

        let diff = (len - target_len) / len;
        // Optimization: Pre-calculate scalar term to reduce vector multiplications
        let correction = delta * (diff * stiffness / (w1 + w2));

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

        // Should not panic anymore
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

    #[test]
    fn test_stiffness_explosion() {
        let mut system = PbdSystem::new();
        let p1 = system.add_particle(Vec3::ZERO, 1.0);
        let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);

        // Add a constraint with MAX stiffness
        system.constraints.push(Constraint::Distance {
            p1,
            p2,
            rest_length: 0.5, // Pull them closer
            stiffness: f32::MAX,
        });

        // Step simulation multiple times
        for _ in 0..10 {
            system.step(0.1, 1);
        }

        let pos1 = system.particles[p1].pos;

        // It should remain finite due to clamping
        assert!(pos1.is_finite());
    }

    #[test]
    fn test_actuator_constraint() {
        let mut system = PbdSystem::new();
        let p1 = system.add_particle(Vec3::new(0.0, 0.0, 0.0), 1.0);
        let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);

        // Add actuator expanding from 1.0 to 2.0
        // Currently at 1.0. Factor 1.0 means target = 2.0.
        system.constraints.push(Constraint::Actuator {
            p1,
            p2,
            min_len: 1.0,
            max_len: 2.0,
            factor: 1.0,
            stiffness: 1.0,
        });

        // Step
        system.step(0.1, 10);

        let dist = system.particles[p1].pos.distance(system.particles[p2].pos);
        // Should expand towards 2.0
        assert!(dist > 1.0);
        assert!((dist - 2.0).abs() < 0.1);

        // Test contraction
        system.constraints[0] = Constraint::Actuator {
            p1,
            p2,
            min_len: 1.0,
            max_len: 2.0,
            factor: 0.0, // Target 1.0
            stiffness: 1.0,
        };

        system.step(0.1, 10);
        let dist = system.particles[p1].pos.distance(system.particles[p2].pos);
        // Should contract towards 1.0
        assert!((dist - 1.0).abs() < 0.1);
    }

    #[test]
    fn test_step_zero_dt() {
        let mut system = PbdSystem::new();
        let p1 = system.add_particle(Vec3::ZERO, 1.0);
        system.particles[p1].vel = Vec3::new(1.0, 0.0, 0.0);

        // Should simply return and not update anything or panic
        system.step(0.0, 1);

        let vel = system.particles[p1].vel;
        assert!(!vel.is_nan(), "Velocity should not be NaN");
        assert_eq!(vel, Vec3::new(1.0, 0.0, 0.0)); // Velocity remains unchanged
    }

    #[test]
    fn test_step_negative_dt() {
        let mut system = PbdSystem::new();
        let p1 = system.add_particle(Vec3::ZERO, 1.0);

        // Should return early
        system.step(-0.1, 1);

        let pos = system.particles[p1].pos;
        assert_eq!(pos, Vec3::ZERO);
    }

    #[test]
    fn test_singularity_behavior() {
        let mut system = PbdSystem::new();
        // Two particles at exactly the same position
        let p1 = system.add_particle(Vec3::ZERO, 1.0);
        let p2 = system.add_particle(Vec3::ZERO, 1.0);

        // Constraint trying to push them apart to distance 1.0
        system.constraints.push(Constraint::Distance {
            p1,
            p2,
            rest_length: 1.0,
            stiffness: 1.0,
        });

        system.step(0.1, 10);

        // Due to len < EPSILON check, they should NOT move
        let dist = system.particles[p1].pos.distance(system.particles[p2].pos);
        assert_eq!(dist, 0.0);
    }

    #[test]
    fn test_nan_propagation() {
        let mut system = PbdSystem::new();
        let p1 = system.add_particle(Vec3::ZERO, 1.0);
        let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);

        // Inject NaN into p1
        system.particles[p1].pos = Vec3::NAN;

        // p1 connected to p2
        system.add_distance_constraint(p1, p2, 1.0);

        system.step(0.1, 10);

        // p2 should NOT be infected if we guard against it.
        // Currently this assertion will FAIL if the bug exists.
        assert!(
            system.particles[p2].pos.is_finite(),
            "NaN propagated to p2!"
        );
    }

    #[test]
    fn test_zombie_constraints() {
        let mut system = PbdSystem::new();
        let p1 = system.add_particle(Vec3::ZERO, 1.0);
        let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);

        system.add_distance_constraint(p1, p2, 1.0);

        // Remove the particles (hacky: standard Vec::pop)
        // Note: this invalidates indices p1(0) and p2(1).
        system.particles.pop(); // Removes p2
        system.particles.pop(); // Removes p1

        // Step should not panic because solve_distance and Pin constraints
        // internally check bounds before accessing particles.
        system.step(0.1, 1);
    }

    #[test]
    fn test_actuator_out_of_bounds() {
        let mut system = PbdSystem::new();
        let p1 = system.add_particle(Vec3::ZERO, 1.0);
        let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);

        // Factor 2.0 -> target = 1.0 + (2.0-1.0)*2.0 = 3.0
        system.constraints.push(Constraint::Actuator {
            p1,
            p2,
            min_len: 1.0,
            max_len: 2.0,
            factor: 2.0,
            stiffness: 1.0,
        });

        system.step(0.1, 10);

        let dist = system.particles[p1].pos.distance(system.particles[p2].pos);
        // Should expand towards 3.0
        assert!((dist - 3.0).abs() < 0.1);
    }

    #[test]
    #[should_panic(expected = "Mass must be non-negative and finite")]
    fn test_negative_mass_panic() {
        let mut system = PbdSystem::new();
        system.add_particle(Vec3::ZERO, -1.0);
    }

    #[test]
    fn test_nan_mass_robustness() {
        let mut system = PbdSystem::new();
        let p1 = system.add_particle(Vec3::ZERO, 1.0); // Normal mass to start
        let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);

        // Manually inject NaN into inv_mass to bypass add_particle check
        // and verify solver robustness
        system.particles[p1].inv_mass = f32::NAN;

        system.add_distance_constraint(p1, p2, 1.0);
        system.step(0.1, 10);

        // p2 should remain finite
        assert!(system.particles[p2].pos.is_finite());
    }

    #[test]
    fn test_actuator_nan_factor_robustness() {
        let mut system = PbdSystem::new();
        let p1 = system.add_particle(Vec3::ZERO, 1.0);
        let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);

        system.add_actuator_constraint(p1, p2, 1.0, 2.0, 1.0);

        // Inject NaN factor
        if let Constraint::Actuator { factor, .. } = &mut system.constraints[0] {
            *factor = f32::NAN;
        }

        system.step(0.1, 10);

        // Should be safe
        assert!(system.particles[p1].pos.is_finite());
        assert!(system.particles[p2].pos.is_finite());
    }
}
