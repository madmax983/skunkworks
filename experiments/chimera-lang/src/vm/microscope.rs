use super::{ChimeraVM, Value};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct MicroscopeData {
    pub coords: (usize, usize),
    pub value: Value,
    pub hormone_levels: [i64; 3],
    pub waste_level: i64,
    pub mutagen_level: i64,
    pub light_level: i64,
    pub organelles: Vec<OrganelleInfo>,
}

#[derive(Debug, Clone, Serialize, Deserialize, PartialEq)]
pub struct OrganelleInfo {
    pub kind: String,
    pub ip: (usize, usize),
    pub stack_depth: usize,
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
