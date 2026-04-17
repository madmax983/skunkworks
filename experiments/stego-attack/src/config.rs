use serde::{Deserialize, Serialize};

/// Configuration for the swarm attack simulation.
///
/// Determines the speed, target location, and rate at which the background
/// dissolves during the animation.
///
/// # Examples
///
/// ```
/// use stego_attack::config::AttackConfig;
///
/// let config = AttackConfig {
///     target_x: 0.5,
///     target_y: 0.5,
///     agent_speed: 10.0,
///     dissolve_rate: 0.05,
/// };
/// ```
#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttackConfig {
    /// The normalized x-coordinate (0.0 to 1.0) for the agents' destination.
    pub target_x: f32,
    /// The normalized y-coordinate (0.0 to 1.0) for the agents' destination.
    pub target_y: f32,
    /// The maximum speed at which the agents can move per frame.
    pub agent_speed: f32,
    /// The probability (0.0 to 1.0) per frame that a background pixel loses alpha.
    pub dissolve_rate: f32,
}

impl Default for AttackConfig {
    fn default() -> Self {
        Self {
            target_x: 0.5,
            target_y: 0.5,
            agent_speed: 2.0,
            dissolve_rate: 0.01,
        }
    }
}
