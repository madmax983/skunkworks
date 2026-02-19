use super::normalize_coords;
use crate::ast::JunctionType;
use crate::vm::Value;

pub fn apply_construct_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    delayed_signals: &mut Vec<Vec<Option<Value>>>,
    grid: &[Vec<Value>],
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "B" => {
            // Blueprint: West (Trigger), North (Height), South (Width) -> Self (Blueprint)
            if w_sig.is_some() {
                // Get Height
                let h = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                    if let Some(Value::Int(h)) = &current_signals[ny][nx] {
                        (*h).max(1)
                    } else {
                        1
                    }
                } else {
                    1
                };

                // Get Width
                let w = if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                    if let Some(Value::Int(w)) = &current_signals[sy][sx] {
                        (*w).max(1)
                    } else {
                        1
                    }
                } else {
                    1
                };

                // Scan Area: [y - h/2 .. y + h/2] x [x + 1 .. x + w]
                // Centered vertically, expanding East
                let start_y = (y as i64) - (h / 2);
                let end_y = start_y + h;
                let start_x = (x as i64) + 1;
                let end_x = start_x + w;

                let mut rows = Vec::new();
                for r in start_y..end_y {
                    let mut row_vals = Vec::new();
                    for c in start_x..end_x {
                        if let Some((ny, nx)) = normalize_coords(r, c) {
                            row_vals.push(grid[ny][nx].clone());
                        } else {
                            row_vals.push(Value::Int(0)); // Empty/Void
                        }
                    }
                    rows.push(Value::Junction(JunctionType::Dish, row_vals));
                }

                if delayed_signals[y][x].is_none() {
                    delayed_signals[y][x] = Some(Value::Junction(JunctionType::Dish, rows));
                    changes = true;
                }
            }
        }
        _ => {}
    }
    changes
}

pub fn apply_construct_sinks(vm: &mut crate::vm::ChimeraVM, rune: &str, y: usize, x: usize) {
    match rune {
        "Π" => {
            // Prototyper: West (Blueprint) -> Grid (East)
            let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                vm.prologue_state.signal_grid[wy][wx].clone()
            } else {
                None
            };

            if let Some(Value::Junction(JunctionType::Dish, rows)) = w_sig {
                // Paste
                let h = rows.len() as i64;
                let start_y = (y as i64) - (h / 2);
                let start_x = (x as i64) + 1;

                for (r_idx, row_val) in rows.iter().enumerate() {
                    if let Value::Junction(JunctionType::Dish, cells) = row_val {
                        for (c_idx, cell_val) in cells.iter().enumerate() {
                            if let Some((ny, nx)) =
                                normalize_coords(start_y + r_idx as i64, start_x + c_idx as i64)
                            {
                                // Always overwrite.
                                vm.grid[ny][nx] = cell_val.clone();
                            }
                        }
                    }
                }
                // Light up self
                vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
            }
        }
        _ => {}
    }
}
