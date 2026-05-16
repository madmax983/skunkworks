use super::{ChimeraVM, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Represents a `MicroscopeData`.
pub struct MicroscopeData {
    /// The `coords` field.
    pub coords: (usize, usize),
    /// The `value` field.
    pub value: Value,
    /// The `hormone_levels` field.
    pub hormone_levels: [i64; 3],
    /// The `waste_level` field.
    pub waste_level: i64,
    /// The `mutagen_level` field.
    pub mutagen_level: i64,
    /// The `light_level` field.
    pub light_level: i64,
    /// The `organelles` field.
    pub organelles: Vec<OrganelleInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
/// Represents a `OrganelleInfo`.
pub struct OrganelleInfo {
    /// The `kind` field.
    pub kind: String,
    /// The `ip` field.
    pub ip: (usize, usize),
    /// The `stack_depth` field.
    pub stack_depth: usize,
    /// The `energy_share` field.
    pub energy_share: i64, // Placeholder for now
}

impl Default for MicroscopeData {
    fn default() -> Self {
        Self {
            coords: (0, 0),
            value: Value::Int(0),
            hormone_levels: [0, 0, 0],
            waste_level: 0,
            mutagen_level: 0,
            light_level: 0,
            organelles: Vec::new(),
        }
    }
}

/// Performs the `scan` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of scan
/// ```
pub fn scan(vm: &ChimeraVM, y: usize, x: usize) -> MicroscopeData {
    let mut data = MicroscopeData {
        coords: (y, x),
        ..Default::default()
    };

    if y < 16 && x < 16 {
        data.value = vm.grid[y][x].clone();

        #[cfg(feature = "nova")]
        {
            data.hormone_levels = vm.hormone_grid[y][x];
            data.waste_level = vm.waste_grid[y][x];
            data.mutagen_level = vm.mutagen_grid[y][x];
            data.light_level = vm.light_grid[y][x];

            for org in &vm.organelles {
                if org.context_loc == (y, x) {
                    data.organelles.push(OrganelleInfo {
                        kind: format!("{:?}", org.kind),
                        ip: org.ip,
                        stack_depth: org.stack.len(),
                        energy_share: 0,
                    });
                }
            }
        }
    }

    data
}
