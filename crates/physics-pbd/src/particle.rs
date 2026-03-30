use glam::Vec3;

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
