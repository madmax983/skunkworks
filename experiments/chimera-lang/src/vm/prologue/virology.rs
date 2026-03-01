use super::normalize_coords;
use crate::vm::Value;

pub fn apply_virology_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "v" | "i" | "a" => {
            // Virus/Infect/Antibody: West (Trigger) -> Self (Active)
            if w_sig.is_some() && next_signals[y][x].is_none() {
                next_signals[y][x] = Some(Value::Int(1));
                changes = true;
            }
        }
        _ => {}
    }
    changes
}

pub fn apply_virology_sinks(vm: &mut crate::vm::ChimeraVM, rune: &str, y: usize, x: usize) {
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx].clone()
    } else {
        None
    };

    if w_sig.is_none() {
        return;
    }

    match rune {
        "v" => {
            // Virus: Replicate to East
            if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                let target_val = &vm.grid[ey][ex];
                // Only overwrite if empty or explicitly overwritable?
                // For a virus, let's say it overwrites Empty (0) or Empty String.
                let is_empty = match target_val {
                    Value::Int(0) => true,
                    Value::Str(s) => s.is_empty(),
                    _ => false,
                };

                if is_empty {
                    vm.grid[ey][ex] = Value::Str("v".to_string());
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1)); // Light up
                }
            }
        }
        "i" => {
            // Infect: North (Payload) + South (Direction) -> Target
            let n_val = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                Some(vm.grid[ny][nx].clone())
            } else {
                None
            };

            let s_val = if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                Some(vm.grid[sy][sx].clone())
            } else {
                None
            };

            if let (Some(payload), Some(Value::Int(dir))) = (n_val, s_val) {
                let (dy, dx) = match dir {
                    0 => (-1, 0), // N
                    1 => (0, 1),  // E
                    2 => (1, 0),  // S
                    3 => (0, -1), // W
                    _ => (0, 0),
                };

                if (dy, dx) != (0, 0) {
                    if let Some((ty, tx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                        vm.grid[ty][tx] = payload;
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                    }
                }
            }
        }
        "a" => {
            // Antibody: Clean neighbors of 'v'
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if let Value::Str(s) = &vm.grid[ny][nx] {
                        if s == "v" {
                            vm.grid[ny][nx] = Value::Int(0); // Kill virus
                            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                        }
                    }
                }
            }
        }
        _ => {}
    }
}
