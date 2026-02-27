use super::normalize_coords;
use crate::vm::{ChimeraVM, Value, GRID_SIZE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Copy, PartialEq, Eq, Default, Serialize, Deserialize)]
pub enum RealityMode {
    #[default]
    Prologue,
    Orca,
    Silicon,
    Life,
}

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RealityState {
    pub reality_map: Vec<Vec<RealityMode>>,
}

impl Default for RealityState {
    fn default() -> Self {
        Self {
            reality_map: vec![vec![RealityMode::Prologue; GRID_SIZE]; GRID_SIZE],
        }
    }
}

impl RealityState {
    pub fn new() -> Self {
        Self::default()
    }

    pub fn get_mode(&self, y: usize, x: usize) -> RealityMode {
        if y < GRID_SIZE && x < GRID_SIZE {
            self.reality_map[y][x]
        } else {
            RealityMode::Prologue
        }
    }
}

pub fn scan_reality_bubbles(vm: &mut ChimeraVM) {
    // Reset map to Prologue
    for row in vm.prologue_state.reality_state.reality_map.iter_mut() {
        for cell in row.iter_mut() {
            *cell = RealityMode::Prologue;
        }
    }

    // We scan for 🌐 runes
    let runes: Vec<(usize, usize)> = vm.prologue_state.runes.iter().cloned().collect();

    for (y, x) in runes {
        if let Value::Str(s) = &vm.grid[y][x] {
            if s == "🌐" {
                apply_reality_bubble(vm, y, x);
            }
        }
    }
}

fn apply_reality_bubble(vm: &mut ChimeraVM, y: usize, x: usize) {
    // West: Radius
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx].clone()
            .or_else(|| Some(vm.grid[wy][wx].clone()))
    } else {
        None
    };

    // North: Mode
    let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
        vm.prologue_state.signal_grid[ny][nx].clone()
            .or_else(|| Some(vm.grid[ny][nx].clone()))
    } else {
        None
    };

    // Resolve Radius (Int or Str->Int)
    let radius = match w_sig {
        Some(Value::Int(n)) => Some(n),
        Some(Value::Str(s)) => s.parse::<i64>().ok(),
        _ => None,
    };

    if let (Some(r), Some(mode_val)) = (radius, n_sig) {
        let mode = match mode_val {
            Value::Int(1) => RealityMode::Orca,
            Value::Str(s) if s == "O" => RealityMode::Orca,
            Value::Int(2) => RealityMode::Silicon,
            Value::Str(s) if s == "S" => RealityMode::Silicon,
            Value::Int(3) => RealityMode::Life,
            Value::Str(s) if s == "L" => RealityMode::Life,
            _ => RealityMode::Prologue,
        };

        if mode == RealityMode::Prologue {
            return;
        }

        // Apply Bubble
        let r = r.max(0);

        crate::vm::iterate_circle(
            #[cfg(feature = "nova")]
            crate::vm::Topology::Torus,
            #[cfg(not(feature = "nova"))]
            crate::vm::Topology::Plane,
            x as i64,
            y as i64,
            r,
            |cx, cy| {
                vm.prologue_state.reality_state.reality_map[cy][cx] = mode;
            }
        );
    }
}
