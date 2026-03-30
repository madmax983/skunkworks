use glam::Vec3;

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
