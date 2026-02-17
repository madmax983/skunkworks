use serde::{Deserialize, Serialize};

#[derive(Serialize, Deserialize, Debug, Clone)]
pub struct AttackConfig {
    pub target_x: f32,      // Normalized 0.0 to 1.0
    pub target_y: f32,
    pub agent_speed: f32,
    pub dissolve_rate: f32, // 0.0 to 1.0 per frame (alpha reduction)
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
