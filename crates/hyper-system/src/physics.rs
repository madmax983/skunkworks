//! A 4D Position-Based Dynamics (Pbd) physics engine.
//!
//! This module provides the [`PbdSystem4D`] struct, which simulates the physical
//! behavior of 4-dimensional particles interconnected by various constraints.
//!
//! Unlike traditional force-based physics engines that integrate acceleration into
//! velocity and then into position, Position-Based Dynamics directly modifies the
//! positions of particles to satisfy constraints. This approach is highly stable,
//! making it ideal for real-time visualization experiments in the "Hyper" series
//! where visual stability is often prioritized over strict physical accuracy.
//!
//! # Core Concepts
//!
//! - **Particles:** Represent discrete points in 4D space with mass and velocity.
//! - **Constraints:** Define rules that particle positions must obey (e.g., staying a certain distance apart).
//! - **Integration:** The system moves particles forward in time based on their velocities.
//! - **Solving:** The system iteratively adjusts particle positions until all constraints are satisfied (or approximately satisfied).

use crate::math::Vec4;

/// Represents a discrete physical point in 4-dimensional space.
///
/// In Position-Based Dynamics, particles are primarily defined by their current
/// position and their previous position. The velocity is implicitly derived
/// from the difference between these two points over time.
///
/// # Inverse Mass
/// Notice the use of `inv_mass` (inverse mass) instead of mass. This is a common
/// pattern in game physics:
/// - A mass of `1.0` has an inverse mass of `1.0`.
/// - A mass of `0.0` (infinite mass, like a wall) has an inverse mass of `0.0`.
///
/// This avoids divide-by-zero errors when solving constraints for static objects.
///
/// # Examples
/// ```
/// use hyper_system::math::Vec4;
/// use hyper_system::physics::Particle4D;
///
/// let particle = Particle4D {
///     pos: Vec4::new(1.0, 2.0, 3.0, 4.0),
///     prev_pos: Vec4::new(1.0, 2.0, 3.0, 4.0), // Starts stationary
///     inv_mass: 1.0,                           // 1 / 1.0 mass
///     vel: Vec4::zero(),
///     user_data: Vec4::zero(),                 // Empty payload
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub struct Particle4D {
    /// The current 4D spatial position.
    pub pos: Vec4,
    /// The 4D spatial position at the end of the previous time step.
    pub prev_pos: Vec4,
    /// Inverse mass ($1 / mass$). `0.0` indicates a static/immovable particle.
    pub inv_mass: f32,
    /// The derived velocity, calculated post-integration: `(pos - prev_pos) / dt`.
    pub vel: Vec4,
    /// Arbitrary 4D payload. Often used for experiment-specific data like `magnetic_polarity` in `hyper-fold`.
    pub user_data: Vec4,
}

/// A physical rule that constrains the relative positions of particles.
///
/// Constraints are the heart of a Position-Based Dynamics system. They are
/// iteratively solved to pull particles into valid configurations.
///
/// # Examples
/// ```
/// use hyper_system::math::Vec4;
/// use hyper_system::physics::Constraint4D;
///
/// // Create a simple rod holding two particles exactly 2.0 units apart.
/// let rod = Constraint4D::Distance {
///     p1: 0, // Index of the first particle
///     p2: 1, // Index of the second particle
///     rest_length: 2.0,
///     stiffness: 1.0, // 100% rigid
/// };
/// ```
#[derive(Debug, Clone, Copy)]
pub enum Constraint4D {
    /// Forces two particles to maintain a specific distance from each other.
    Distance {
        /// Index of the first particle in the simulation.
        p1: usize,
        /// Index of the second particle in the simulation.
        p2: usize,
        /// The ideal distance the particles should maintain.
        rest_length: f32,
        /// How strictly this constraint is enforced. `1.0` is completely rigid.
        stiffness: f32,
    },
    /// A dynamic distance constraint that acts like a muscle or hydraulic cylinder.
    Actuator {
        /// Index of the first particle.
        p1: usize,
        /// Index of the second particle.
        p2: usize,
        /// The minimum possible length of the actuator.
        min_len: f32,
        /// The maximum possible length of the actuator.
        max_len: f32,
        /// A normalized value (`0.0` to `1.0`) driving the current target length.
        factor: f32,
        /// How strictly this constraint is enforced.
        stiffness: f32,
    },
    /// Anchors a particle to a specific fixed location in 4D space.
    Pin {
        /// Index of the particle to anchor.
        p: usize,
        /// The exact 4D coordinate to force the particle to inhabit.
        pos: Vec4,
    },
}

/// A 4D Position-Based Dynamics (Pbd) physics system.
///
/// This engine manages a collection of `Particle4D`s and `Constraint4D`s, iteratively solving
/// the latter to create stable, fluid motion suitable for hyper-dimensional visualizations.
///
/// # Examples
/// ```
/// use hyper_system::math::Vec4;
/// use hyper_system::physics::PbdSystem4D;
///
/// let mut system = PbdSystem4D::new();
///
/// // Create two 1kg particles 2.0 units apart on the X-axis
/// let p1 = system.add_particle(Vec4::zero(), 1.0);
/// let p2 = system.add_particle(Vec4::new(2.0, 0.0, 0.0, 0.0), 1.0);
///
/// // Create a slightly squishy distance constraint between them
/// system.add_distance_constraint(p1, p2, 0.8);
///
/// // Step the simulation forward by 0.1s using 10 iterations per step with 2% friction.
/// system.step(0.1, 10, 0.98);
/// ```
pub struct PbdSystem4D {
    /// The collection of all simulated point masses.
    pub particles: Vec<Particle4D>,
    /// The physical rules connecting and limiting those particles.
    pub constraints: Vec<Constraint4D>,
}

impl PbdSystem4D {
    /// Creates a new, empty 4D physics system.
    pub fn new() -> Self {
        Self {
            particles: Vec::new(),
            constraints: Vec::new(),
        }
    }

    /// Adds a new particle to the simulation and returns its index.
    ///
    /// # Mass Handling
    /// Passing a `mass` of `0.0` will result in an inverse mass of `0.0`, effectively
    /// marking the particle as completely immobile ("static"). Any positive `mass` will
    /// be stored as `1.0 / mass`.
    ///
    /// # Arguments
    /// * `pos` - The starting position in 4D space.
    /// * `mass` - The object's weight. Pass `0.0` for an immovable anchor.
    pub fn add_particle(&mut self, pos: Vec4, mass: f32) -> usize {
        let idx = self.particles.len();
        self.particles.push(Particle4D {
            pos,
            prev_pos: pos,
            inv_mass: if mass == 0.0 { 0.0 } else { 1.0 / mass },
            vel: Vec4::zero(),
            user_data: Vec4::zero(),
        });
        idx
    }

    /// Creates a permanent distance constraint between two particles.
    ///
    /// The target distance (`rest_length`) is automatically computed based on
    /// the particles' initial positions at the moment this function is called.
    ///
    /// # Arguments
    /// * `p1` - Index of the first particle.
    /// * `p2` - Index of the second particle.
    /// * `stiff` - Rigidity multiplier. `1.0` is solid, `0.5` acts like a soft spring.
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

    /// Creates a dynamically sizing actuator constraint between two particles.
    ///
    /// This acts like a hydraulic piston. The actual target length is calculated by
    /// interpolating between `min_len` and `max_len` using the `factor` (0.0 to 1.0).
    ///
    /// # Arguments
    /// * `p1` - Index of the first particle.
    /// * `p2` - Index of the second particle.
    /// * `min_len` - The actuator's fully retracted length.
    /// * `max_len` - The actuator's fully extended length.
    /// * `stiff` - How aggressively the actuator enforces its target length.
    /// * `initial_factor` - Normalized starting extension amount (`0.0` to `1.0`).
    pub fn add_actuator_constraint(
        &mut self,
        p1: usize,
        p2: usize,
        min_len: f32,
        max_len: f32,
        stiff: f32,
        initial_factor: f32,
    ) {
        self.constraints.push(Constraint4D::Actuator {
            p1,
            p2,
            min_len,
            max_len,
            factor: initial_factor,
            stiffness: stiff,
        });
    }

    /// Forces a specific particle to remain pinned at a designated 4D coordinate.
    ///
    /// The particle will instantly snap to this position every tick, regardless
    /// of its velocity, mass, or other constraints pulling on it.
    ///
    /// # Arguments
    /// * `p` - Index of the particle to pin.
    /// * `pos` - The 4D coordinate it must occupy.
    pub fn add_pin_constraint(&mut self, p: usize, pos: Vec4) {
        self.constraints.push(Constraint4D::Pin { p, pos });
    }

    /// Steps the simulation forward in time.
    ///
    /// This executes the core PBD algorithm:
    /// 1. Damps velocity with friction.
    /// 2. Extrapolates predicted positions based on current velocity (Integration).
    /// 3. Iteratively nudges positions to satisfy constraints (`iterations` times).
    /// 4. Updates actual velocities based on how far the positions ultimately moved.
    ///
    /// # Arguments
    /// * `dt` - Delta time (in seconds). Pass `0.0` or less to pause simulation.
    /// * `iterations` - Precision solver passes. Higher is stiffer but slower. `10` to `20` is typical.
    /// * `friction` - Velocity multiplier applied *before* integration. `0.98` represents 2% energy loss per tick.
    pub fn step(&mut self, dt: f32, iterations: usize, friction: f32) {
        if dt <= f32::EPSILON {
            return;
        }

        // Integrate
        for p in &mut self.particles {
            if p.inv_mass == 0.0 {
                continue;
            }
            // Apply damping/friction
            p.vel = p.vel.scale(friction);

            p.prev_pos = p.pos;
            #[allow(clippy::assign_op_pattern)]
            {
                p.pos = p.pos + p.vel.scale(dt);
            }

            // Optional: floor constraints or other environmental boundaries
            // can be handled outside by iterating over particles.
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

    /// Internal solver routine adjusting two particles towards a target distance.
    ///
    /// Modifies actual particle positions proportionally based on their inverse masses
    /// and the specified constraint `stiffness`.
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
            #[allow(clippy::assign_op_pattern)]
            {
                particles[p1].pos = particles[p1].pos - correction.scale(w1);
            }
        }
        if w2 > 0.0 {
            #[allow(clippy::assign_op_pattern)]
            {
                particles[p2].pos = particles[p2].pos + correction.scale(w2);
            }
        }
    }
}

impl Default for PbdSystem4D {
    /// Identical to [`PbdSystem4D::new`].
    fn default() -> Self {
        Self::new()
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

        system.step(1.0, 1, 0.98);

        let pos = system.particles[p].pos;
        assert!((pos.x - 0.98).abs() < 1e-6);
    }

    #[test]
    fn test_distance_constraint() {
        let mut system = PbdSystem4D::new();
        let p1 = system.add_particle(Vec4::zero(), 1.0);
        let p2 = system.add_particle(Vec4::new(2.0, 0.0, 0.0, 0.0), 1.0);

        system.constraints.push(Constraint4D::Distance {
            p1,
            p2,
            rest_length: 1.0,
            stiffness: 1.0,
        });

        system.step(0.1, 10, 0.98);

        let dist = system.particles[p1]
            .pos
            .distance_squared(system.particles[p2].pos)
            .sqrt();
        assert!((dist - 1.0).abs() < 0.1);
    }
}
