use super::normalize_coords;
use crate::vm::{ChimeraVM, Value, GRID_SIZE};
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct OneiricGrid {
    pub cells: Vec<Vec<f32>>,
}

impl Default for OneiricGrid {
    fn default() -> Self {
        Self::new()
    }
}

impl OneiricGrid {
    pub fn new() -> Self {
        Self {
            cells: vec![vec![0.0; GRID_SIZE]; GRID_SIZE],
        }
    }
}

/// Updates the Oneiric state based on runes.
pub fn apply_oneiric_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    oneiric_grid: &mut OneiricGrid,
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "☾" => {
            // Moon (Dream In): Input West increases Dream Intensity at location
            if let Some(Value::Int(v)) = w_sig {
                if v > 0 {
                    // Only process once per tick (when we haven't emitted yet)
                    // Accumulate intensity
                    oneiric_grid.cells[y][x] += v as f32 * 0.5;
                    if oneiric_grid.cells[y][x] > 100.0 {
                        oneiric_grid.cells[y][x] = 100.0;
                    }
                    // Emits current intensity as signal
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(Value::Int(oneiric_grid.cells[y][x] as i64));
                        changes = true;
                    }
                }
            }
        }
        "☀" => {
            // Sun (Dream Out): Input from Oneiric Grid -> Output Self
            let intensity = oneiric_grid.cells[y][x];
            if intensity > 1.0 {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(intensity as i64));
                    changes = true;
                }
                // Drain slightly when read
                oneiric_grid.cells[y][x] *= 0.8;
            }
        }
        "☁" => {
            // Cloud (Sustain): If intensity is present, sustain it against decay.
            if oneiric_grid.cells[y][x] > 0.1 {
                oneiric_grid.cells[y][x] += 2.0;
                if oneiric_grid.cells[y][x] > 100.0 {
                    oneiric_grid.cells[y][x] = 100.0;
                }
            }
        }
        _ => {}
    }
    changes
}

/// Applies global dream physics (Diffusion and Decay).
/// This should be called once per tick in `exec_prologue_tick`.
pub fn process_oneiric_tick(vm: &mut ChimeraVM) {
    let mut next_cells = vm.prologue_state.oneiric_grid.cells.clone();
    let decay = 0.90; // Fast decay

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            let mut sum = 0.0;
            let mut count = 0.0;

            // 4-neighbor Diffusion
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    sum += vm.prologue_state.oneiric_grid.cells[ny][nx];
                    count += 1.0;
                }
            }

            if count > 0.0 {
                // Blur: Average of neighbors and self
                let avg = (sum + vm.prologue_state.oneiric_grid.cells[y][x]) / (count + 1.0);
                next_cells[y][x] = avg * decay;
            } else {
                next_cells[y][x] *= decay;
            }

            // Cutoff
            if next_cells[y][x] < 0.1 {
                next_cells[y][x] = 0.0;
            }
        }
    }

    vm.prologue_state.oneiric_grid.cells = next_cells;
}
