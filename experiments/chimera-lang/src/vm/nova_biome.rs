use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Serialize, Deserialize, Default)]
pub enum Biome {
    #[default]
    Plains,
    Swamp,
    Desert,
    Tundra,
    Volcanic,
    // New Biomes
    Glitch,
    Aether,
    Silicon,
    Garden,
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
            Biome::Glitch => 1,  // Chaotic/Fast
            Biome::Aether => 0,  // Instant/Superfluid
            Biome::Silicon => 8, // Conductive but structured
            Biome::Garden => 5,  // Balanced
        }
    }

    /// Returns the decay percentage per tick (0-100).
    /// 100 means no decay (100% retained).
    /// 90 means 10% decay.
    pub fn decay_rate(&self) -> i64 {
        match self {
            Biome::Plains => 100, // No decay (Standard)
            Biome::Swamp => 100,  // Stagnant
            Biome::Desert => 90,  // High evaporation
            Biome::Tundra => 100, // Frozen
            Biome::Volcanic => 95,
            Biome::Glitch => 80,  // Unstable
            Biome::Aether => 100, // Perfect preservation
            Biome::Silicon => 99, // High retention
            Biome::Garden => 100,
        }
    }

    /// Returns the energy cost modifier for operations in this biome.
    /// 1.0 is base cost.
    pub fn energy_cost_modifier(&self) -> f64 {
        match self {
            Biome::Plains => 1.0,
            Biome::Swamp => 1.2,    // Difficult terrain
            Biome::Desert => 1.5,   // Harsh
            Biome::Tundra => 2.0,   // Very Harsh
            Biome::Volcanic => 1.5, // Hazardous
            Biome::Glitch => 0.8,   // Easy but dangerous
            Biome::Aether => 0.5,   // Magic flows freely
            Biome::Silicon => 1.0,
            Biome::Garden => 0.5,   // Fertile
        }
    }

    /// Returns the probability of spontaneous mutation (0.0 - 1.0).
    /// Used to scale the base mutation rate.
    pub fn mutation_rate(&self) -> f64 {
        match self {
            Biome::Plains => 1.0,
            Biome::Swamp => 1.5,
            Biome::Desert => 1.0,
            Biome::Tundra => 0.5, // Preserved
            Biome::Volcanic => 2.0,
            Biome::Glitch => 10.0, // Highly mutagenic
            Biome::Aether => 0.1,  // Pure
            Biome::Silicon => 0.0, // Immutable
            Biome::Garden => 2.0,  // Rapid evolution
        }
    }

    /// Returns the magic amplification factor.
    pub fn magic_amp(&self) -> i64 {
        match self {
            Biome::Aether => 2,
            Biome::Glitch => -1, // Unpredictable
            _ => 1,
        }
    }

    /// Returns the RGB color for TUI visualization.
    pub fn color(&self) -> (u8, u8, u8) {
        match self {
            Biome::Plains => (0, 0, 0),       // Default/Transparent
            Biome::Swamp => (20, 40, 20),     // Dark Green
            Biome::Desert => (60, 40, 10),    // Dark Orange/Brown
            Biome::Tundra => (30, 50, 60),    // Dark Cyan/Blue
            Biome::Volcanic => (50, 10, 10),  // Dark Red
            Biome::Glitch => (40, 0, 40),     // Dark Magenta
            Biome::Aether => (40, 40, 50),    // Dark Blue/White
            Biome::Silicon => (30, 30, 35),   // Dark Gray/Metallic
            Biome::Garden => (20, 60, 20),    // Lush Green
        }
    }
}
