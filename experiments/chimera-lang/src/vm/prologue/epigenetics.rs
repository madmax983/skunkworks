use crate::vm::ChimeraVM;
use crate::vm::prologue::normalize_coords;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Serialize, Deserialize)]
pub enum EpigeneticMark {
    None,
    Methylated,    // Silenced
    Phosphorylated, // Amplified
}

impl Default for EpigeneticMark {
    fn default() -> Self {
        Self::None
    }
}

pub fn apply_epigenetic_runes(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    // Check if triggered from West
    let triggered = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx].is_some()
    } else {
        false
    };

    if triggered {
        // Target is South
        if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
            match rune {
                "." => {
                    vm.prologue_state.epigenetic_grid[sy][sx] = EpigeneticMark::Methylated;
                    vm.prologue_state.signal_grid[y][x] = Some(crate::vm::Value::Int(1)); // Light up
                }
                ":" => {
                    vm.prologue_state.epigenetic_grid[sy][sx] = EpigeneticMark::Phosphorylated;
                    vm.prologue_state.signal_grid[y][x] = Some(crate::vm::Value::Int(1));
                }
                "," => {
                    vm.prologue_state.epigenetic_grid[sy][sx] = EpigeneticMark::None;
                    vm.prologue_state.signal_grid[y][x] = Some(crate::vm::Value::Int(1));
                }
                _ => {}
            }
        }
    }
}
