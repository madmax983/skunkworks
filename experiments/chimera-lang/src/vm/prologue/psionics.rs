use super::normalize_coords;
use crate::vm::{ChimeraVM, Value};

/// Applies Psionics propagation runes.
///
/// *   `Θ` (Theta): Telepathy. Reads West (Y), North (X). Output Value at `grid[Y][X]` to Self.
pub fn apply_psionics_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut [Vec<Option<Value>>],
    grid: &[Vec<Value>],
) -> bool {
    let mut changes = false;
    if rune == "Θ" {
        // Theta: Telepathy. Read Remote.
        // West: Y
        // North: X
        let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
            current_signals[wy][wx].clone()
        } else {
            None
        };
        let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
            current_signals[ny][nx].clone()
        } else {
            None
        };

        // println!("Θ at ({}, {}): w_sig={:?}, n_sig={:?}", y, x, w_sig, n_sig);

        if let (Some(Value::Int(ry)), Some(Value::Int(rx))) = (w_sig, n_sig) {
            if let Some((ty, tx)) = normalize_coords(ry, rx) {
                let val = grid[ty][tx].clone();
                // println!("Θ reading ({}, {}) -> {:?}", ty, tx, val);
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(val);
                    changes = true;
                } else if let Some(existing) = &next_signals[y][x] {
                    if *existing != val {
                        next_signals[y][x] = Some(val);
                        changes = true;
                    }
                }
            }
        }
    }
    changes
}

/// Applies Psionics Sink runes.
///
/// *   `Ξ` (Xi): Telekinesis. West (Dir), North (Y), East (X). Move `grid[Y][X]` in Dir.
/// *   `Σ` (Sigma): Suggestion. West (Val), North (Y), East (X). Write `Val` to `grid[Y][X]`.
pub fn apply_psionics_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    match rune {
        "Ξ" => {
            // Xi: Telekinesis
            // West: Dir (0=N, 1=E, 2=S, 3=W)
            // North: Y
            // East: X

            let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                vm.prologue_state.signal_grid[wy][wx].clone()
            } else {
                None
            };

            let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                vm.prologue_state.signal_grid[ny][nx].clone()
            } else {
                None
            };

            let e_sig = if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                vm.prologue_state.signal_grid[ey][ex].clone()
            } else {
                None
            };

            if let (Some(Value::Int(dir)), Some(Value::Int(ry)), Some(Value::Int(rx))) =
                (w_sig, n_sig, e_sig)
            {
                if let Some((ty, tx)) = normalize_coords(ry, rx) {
                    // Move logic
                    let (dy, dx) = match dir % 4 {
                        0 => (-1, 0),
                        1 => (0, 1),
                        2 => (1, 0),
                        3 => (0, -1),
                        _ => (0, 0),
                    };

                    if let Some((ny, nx)) = normalize_coords(ty as i64 + dy, tx as i64 + dx) {
                        // Check if dest is empty (0)
                        if matches!(vm.grid[ny][nx], Value::Int(0)) {
                            let val = vm.grid[ty][tx].clone();
                            vm.grid[ny][nx] = val;
                            vm.grid[ty][tx] = Value::Int(0);
                            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                            // Success
                        }
                    }
                }
            }
        }
        "Σ" => {
            // Sigma: Suggestion
            // West: Val
            // North: Y
            // East: X

            let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                vm.prologue_state.signal_grid[wy][wx].clone()
            } else {
                None
            };

            let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                vm.prologue_state.signal_grid[ny][nx].clone()
            } else {
                None
            };

            let e_sig = if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                vm.prologue_state.signal_grid[ey][ex].clone()
            } else {
                None
            };

            if let (Some(val), Some(Value::Int(ry)), Some(Value::Int(rx))) = (w_sig, n_sig, e_sig) {
                if let Some((ty, tx)) = normalize_coords(ry, rx) {
                    vm.grid[ty][tx] = val;
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1)); // Success
                }
            }
        }
        _ => {}
    }
}
