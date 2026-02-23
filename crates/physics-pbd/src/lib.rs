//! # Physics PBD 🍎
//!
//! > *"Motion is the language of the universe, and constraints are its grammar."*
//!
//! A lightweight, high-performance Position Based Dynamics (PBD) physics engine designed for
//! creative coding, games, and simulations.
//!
//! Unlike impulse-based engines (like Box2D), PBD solves constraints by directly modifying
//! particle positions. This makes it unconditionally stable, easy to implement, and perfect
//! for simulating soft bodies, cloth, ropes, and biological structures.
//!
//! ## Key Concepts
//!
//! - **Particles**: Point masses that obey Newton's laws but are ultimately ruled by geometry.
//! - **Constraints**: Geometric rules (like "keep these two points 5 units apart") that are
//!   resolved iteratively.
//! - **The Solver**: A loop that nudges particles to satisfy constraints, effectively
//!   projecting them onto a valid manifold.
//!
//! ## The Hero's Journey
//!
//! Create a swinging pendulum in just a few lines of code:
//!
//! ```
//! use physics_pbd::{PbdSystem, Constraint};
//! use macroquad::prelude::Vec3;
//!
//! fn main() {
//!     let mut system = PbdSystem::new();
//!
//!     // 1. Create the Anchor (Static)
//!     // Mass 0.0 means infinite mass - it won't move!
//!     let anchor = system.add_particle(Vec3::new(0.0, 10.0, 0.0), 0.0);
//!
//!     // 2. Create the Bob (Dynamic)
//!     let bob = system.add_particle(Vec3::new(5.0, 10.0, 0.0), 1.0);
//!
//!     // 3. Connect them with a rigid rod (Distance Constraint)
//!     // Stiffness 1.0 means it's a hard constraint (steel rod), not a spring.
//!     system.add_distance_constraint(anchor, bob, 1.0);
//!
//!     // 4. Simulation Loop
//!     let dt = 0.016;
//!
//!     // Apply Gravity manually (this engine is opinion-free about external forces)
//!     system.particles[bob].vel += Vec3::new(0.0, -9.81, 0.0) * dt;
//!
//!     // Step the physics world by 16ms, with 10 solver iterations for stability.
//!     system.step(dt, 10);
//!
//!     // The bob falls and swings!
//!     let pos = system.particles[bob].pos;
//!     assert!(pos.y < 10.0);
//! }
//! ```
//!
//! ## Performance
//!
//! PBD is fast. This implementation uses a **split-borrow** optimization in the solver loop,
//! allowing it to iterate over constraints without repeatedly indexing into the particle array
//! with bounds checks.
//!
//! - **O(N)** Integration step.
//! - **O(C * I)** Constraint solving, where `C` is constraints and `I` is iterations.
//!
//! For best performance, keep `iterations` low (2-5) for soft bodies (jelly, cloth) and
//! higher (10-20) for rigid structures.
//!
//! ## Caveats
//!
//! - **NaN Propagation**: If a particle position becomes `NaN` (e.g., from external logic),
//!   constraint solvers usually ignore it to prevent infection, but it's best to filter inputs.
//! - **Singularities**: Constraints between particles at the *exact* same position (distance 0)
//!   are ignored to avoid division by zero. Give them a tiny offset if they must start together.
//! - **Public Fields**: The `Constraint` enum fields are public for direct access, but be
//!   careful! Setting `stiffness` outside `[0.0, 1.0]` or `rest_length` < 0 can explode the universe.

use macroquad::prelude::*;

/// A point mass in the physics simulation.
///
/// Particles are the fundamental building blocks of the PBD system. They have position,
/// velocity, and mass properties that determine how they interact with constraints and forces.
#[derive(Debug, Clone, Copy)]
pub struct Particle {
    /// Current position of the particle.
    pub pos: Vec3,
    /// Previous position of the particle (used for Verlet integration).
    ///
    /// The PBD solver implicitly calculates velocity as `(pos - prev_pos) / dt`.
    pub prev_pos: Vec3,
    /// Inverse mass of the particle (1.0 / mass).
    ///
    /// - `0.0`: Infinite mass (static/kinematic object).
    /// - `> 0.0`: Dynamic object.
    pub inv_mass: f32,
    /// Velocity of the particle.
    ///
    /// While PBD is position-based, explicit velocity is tracked for damping and external forces.
    pub vel: Vec3,
}

impl Particle {
    /// Returns true if the particle is static (infinite mass).
    ///
    /// # Example
    /// ```
    /// use physics_pbd::Particle;
    /// use macroquad::prelude::Vec3;
    ///
    /// let p = Particle {
    ///     pos: Vec3::ZERO,
    ///     prev_pos: Vec3::ZERO,
    ///     inv_mass: 0.0,
    ///     vel: Vec3::ZERO,
    /// };
    /// assert!(p.is_static());
    /// ```
    pub fn is_static(&self) -> bool {
        self.inv_mass == 0.0
    }
}

/// A geometric rule that limits or influences the movement of particles.
#[derive(Debug, Clone, Copy)]
pub enum Constraint {
    /// Constrains two particles to be at a fixed distance from each other.
    ///
    /// Think of this as a bone, rod, or spring connecting two points.
    Distance {
        /// Index of the first particle.
        p1: usize,
        /// Index of the second particle.
        p2: usize,
        /// The target distance between the particles.
        /// **Warning:** Must be non-negative.
        rest_length: f32,
        /// The stiffness of the constraint (0.0 to 1.0).
        /// - `1.0`: Rigid constraint (immediate correction).
        /// - `0.1`: Elastic/springy behavior.
        stiffness: f32,
    },
    /// An actuator that changes the distance between two particles based on a factor.
    ///
    /// Useful for simulating muscles, pistons, or motorized hinges.
    /// The target length is calculated as `min_len + (max_len - min_len) * factor`.
    Actuator {
        /// Index of the first particle.
        p1: usize,
        /// Index of the second particle.
        p2: usize,
        /// The length when `factor` is 0.0.
        min_len: f32,
        /// The length when `factor` is 1.0.
        max_len: f32,
        /// The current extension factor.
        /// - `0.0`: Retracted (`min_len`).
        /// - `1.0`: Extended (`max_len`).
        /// - Values outside `[0.0, 1.0]` allow over-extension/compression.
        factor: f32,
        /// The stiffness of the constraint (0.0 to 1.0).
        stiffness: f32,
    },
    /// Pins a particle to a specific position in world space.
    ///
    /// Useful for anchoring objects (like a flag on a pole) or implementing mouse dragging.
    Pin {
        /// Index of the particle to pin.
        p: usize,
        /// The absolute world position to pin the particle to.
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
    /// Creates a new, empty physics system.
    ///
    /// # Example
    /// ```
    /// use physics_pbd::PbdSystem;
    /// let system = PbdSystem::new();
    /// ```
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            constraints: Vec::new(),
        }
    }

    /// Adds a particle to the system.
    ///
    /// # Arguments
    /// * `pos` - Initial position of the particle.
    /// * `mass` - Mass of the particle. If `0.0`, the particle is **static** (infinite mass) and will not move unless manually updated.
    ///
    /// # Returns
    /// The index of the added particle. Store this index to reference the particle later (e.g., for constraints).
    ///
    /// # Example
    /// ```
    /// use physics_pbd::PbdSystem;
    /// use macroquad::prelude::Vec3;
    ///
    /// let mut system = PbdSystem::new();
    ///
    /// // A static anchor point
    /// let anchor = system.add_particle(Vec3::new(0.0, 10.0, 0.0), 0.0);
    ///
    /// // A dynamic particle with mass 1.0
    /// let ball = system.add_particle(Vec3::new(1.0, 10.0, 0.0), 1.0);
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

    /// Adds a distance constraint (a rigid rod or spring) between two particles.
    ///
    /// The constraint's `rest_length` is automatically set to the *current* distance between the particles.
    ///
    /// # Arguments
    /// * `p1`, `p2` - Indices of the particles to connect.
    /// * `stiff` - Stiffness of the constraint, from `0.0` (loose) to `1.0` (rigid).
    ///
    /// # Example
    /// ```
    /// use physics_pbd::PbdSystem;
    /// use macroquad::prelude::Vec3;
    ///
    /// let mut system = PbdSystem::new();
    /// let p1 = system.add_particle(Vec3::ZERO, 1.0);
    /// let p2 = system.add_particle(Vec3::new(2.0, 0.0, 0.0), 1.0);
    ///
    /// // Connect them. Rest length will be 2.0.
    /// system.add_distance_constraint(p1, p2, 1.0);
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

    /// Adds an actuator constraint (a muscle or piston) between two particles.
    ///
    /// An actuator behaves like a distance constraint, but its target length can be modulated
    /// dynamically by changing the `factor` field of the constraint.
    ///
    /// # Arguments
    /// * `min_len` - The length when `factor` is 0.0.
    /// * `max_len` - The length when `factor` is 1.0.
    /// * `stiff` - Stiffness of the constraint.
    ///
    /// # Example
    /// ```
    /// use physics_pbd::PbdSystem;
    /// use macroquad::prelude::Vec3;
    ///
    /// let mut system = PbdSystem::new();
    /// let p1 = system.add_particle(Vec3::ZERO, 1.0);
    /// let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);
    ///
    /// // Create a piston that can extend from 1.0 to 2.0
    /// system.add_actuator_constraint(p1, p2, 1.0, 2.0, 1.0);
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

    /// Pins a particle to a specific position in world space.
    ///
    /// This is a "hard" constraint that overrides other forces. Useful for mouse dragging or anchors.
    ///
    /// # Example
    /// ```
    /// use physics_pbd::PbdSystem;
    /// use macroquad::prelude::Vec3;
    ///
    /// let mut system = PbdSystem::new();
    /// let p = system.add_particle(Vec3::ZERO, 1.0);
    ///
    /// // Pin the particle to (5, 5, 5)
    /// system.add_pin_constraint(p, Vec3::new(5.0, 5.0, 5.0));
    /// ```
    pub fn add_pin_constraint(&mut self, p: usize, pos: Vec3) {
        self.constraints.push(Constraint::Pin { p, pos });
    }

    /// Advances the simulation by `dt` seconds.
    ///
    /// This performs:
    /// 1. **Integration**: Updates positions based on velocity.
    /// 2. **Constraint Solving**: Iteratively corrects positions to satisfy constraints.
    /// 3. **Velocity Update**: Updates velocities based on position changes.
    ///
    /// # Arguments
    /// * `dt` - Delta time in seconds. If `<= 0` or `NaN`, the step is skipped.
    /// * `iterations` - Number of solver iterations.
    ///     - **1-5**: Fast, bouncy (good for cloth/soft bodies).
    ///     - **10-20**: Stable, rigid (good for structures).
    ///
    /// # Panics
    /// This method does **not** panic if constraints reference invalid particle indices; it simply ignores them.
    ///
    /// # Example
    /// ```
    /// use physics_pbd::PbdSystem;
    ///
    /// let mut system = PbdSystem::new();
    /// // Run at 60 FPS with high stability
    /// system.step(1.0 / 60.0, 10);
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
        if p1 >= particles.len() || p2 >= particles.len() {
            return;
        }

        let pos1 = particles[p1].pos;
        let pos2 = particles[p2].pos;
        let w1 = particles[p1].inv_mass;
        let w2 = particles[p2].inv_mass;
        if w1 + w2 == 0.0 {
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
}
