use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Biome {
    #[default]
    Plains,
    Swamp,
    Desert,
    Tundra,
    Volcanic,
}

impl Biome {
    /// Returns the inertia factor for diffusion.
    /// Higher values mean the cell retains more of its own value (slower diffusion).
    /// Base value is usually 4.
    pub fn diffusion_inertia(&self) -> i64 {
        match self {
            Biome::Plains => 4,
            Biome::Swamp => 12,  // Stagnant
            Biome::Desert => 2,  // Windy/Fast
            Biome::Tundra => 20, // Frozen
            Biome::Volcanic => 4,
        }
    }

    /// Returns the decay percentage per tick (0-100).
    /// 100 means no decay (100% retained).
    /// 90 means 10% decay.
    pub fn decay_rate(&self) -> i64 {
        match self {
            Biome::Plains => 100, // No decay (Standard)
            Biome::Swamp => 100,  // Stagnant
            Biome::Desert => 90, // High evaporation
            Biome::Tundra => 100, // Frozen
            Biome::Volcanic => 95,
        }
    }

    /// Returns the energy cost modifier for operations in this biome.
    /// 100 is base cost.
    pub fn fertility(&self) -> i64 {
        match self {
            Biome::Plains => 100,
            Biome::Swamp => 120, // Difficult terrain
            Biome::Desert => 150, // Harsh
            Biome::Tundra => 200, // Very Harsh
            Biome::Volcanic => 150, // Hazardous
        }
    }
}
