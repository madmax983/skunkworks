use super::{normalize_coords, PrologueAgent};
use crate::vm::{ChimeraVM, Value, GRID_SIZE};
use rand::Rng;
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
    next_signals: &mut [Vec<Option<Value>>],
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

/// Process Logic for the Dream Weaver Agent (💤).
///
/// It seeks high Oneiric intensity and weaves dreams (runes) when intensity peaks.
pub fn process_dream_weaver_logic(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let (y, x) = (agent.y, agent.x);
    let mut best_target = None;
    let mut max_intensity = 0.0;

    // Scan neighbors
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            let intensity = vm.prologue_state.oneiric_grid.cells[ny][nx];
            if intensity > max_intensity {
                max_intensity = intensity;
                best_target = Some((ny, nx));
            }
        }
    }

    // Check current cell intensity
    let current_intensity = vm.prologue_state.oneiric_grid.cells[y][x];

    if current_intensity > 80.0 {
        // WEAVE DREAM!
        // Spawn random runes around
        let mut rng = rand::thread_rng();
        let runes = ["*", "~", "!", "?", "&", "|", "+"];
        for (dy, dx) in neighbors {
            if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                if matches!(vm.grid[ny][nx], Value::Int(0)) {
                    let r = runes[rng.gen_range(0..runes.len())];
                    vm.grid[ny][nx] = Value::Str(r.to_string());
                }
            }
        }
        // Consume intensity
        vm.prologue_state.oneiric_grid.cells[y][x] = 0.0;
        vm.output
            .push(format!("DREAM WEAVER: Wove a dream at {},{}", x, y));
        return Some((agent.clone(), None)); // Stay put to admire the work
    }

    if let Some((ny, nx)) = best_target {
        // Move towards intensity
        // Leave trail
        vm.prologue_state.oneiric_grid.cells[y][x] += 10.0;
        if vm.prologue_state.oneiric_grid.cells[y][x] > 100.0 {
            vm.prologue_state.oneiric_grid.cells[y][x] = 100.0;
        }
        return Some((agent.clone(), Some((ny, nx))));
    }

    // Random wander if no intensity
    let mut rng = rand::thread_rng();
    let (dy, dx) = neighbors[rng.gen_range(0..4)];
    if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
        if matches!(grid_snapshot[ny][nx], Value::Int(0)) {
            return Some((agent.clone(), Some((ny, nx))));
        }
    }

    Some((agent.clone(), None))
}

/// Process Logic for the Nightmare Agent (👹).
///
/// It seeks and destroys Dream Weavers, and consumes Oneiric intensity.
pub fn process_nightmare_logic(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let (y, x) = (agent.y, agent.x);

    // 1. Seek Dream Weaver (💤)
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            if let Value::Str(s) = &grid_snapshot[ny][nx] {
                if s == "💤" {
                    // Kill it!
                    vm.grid[ny][nx] = Value::Int(0);
                    vm.prologue_state.registers.remove(&(ny, nx));
                    vm.output
                        .push(format!("NIGHTMARE: Consumed Dream Weaver at {},{}", nx, ny));
                    return Some((agent.clone(), Some((ny, nx)))); // Move into its spot
                }
            }
        }
    }

    // 2. Seek High Intensity (to drain)
    let mut best_target = None;
    let mut max_intensity = 0.0;

    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            let intensity = vm.prologue_state.oneiric_grid.cells[ny][nx];
            if intensity > max_intensity {
                max_intensity = intensity;
                best_target = Some((ny, nx));
            }
        }
    }

    if let Some((ny, nx)) = best_target {
        // Drain intensity at target
        vm.prologue_state.oneiric_grid.cells[ny][nx] *= 0.5;
        return Some((agent.clone(), Some((ny, nx))));
    }

    // Random wander
    let mut rng = rand::thread_rng();
    let (dy, dx) = neighbors[rng.gen_range(0..4)];
    if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
        if matches!(grid_snapshot[ny][nx], Value::Int(0)) {
            return Some((agent.clone(), Some((ny, nx))));
        }
    }

    Some((agent.clone(), None))
}
