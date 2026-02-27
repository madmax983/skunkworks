use super::normalize_coords;
use crate::vm::{ChimeraVM, Value};

/// Applies Catalyst Runes during the signal propagation phase.
///
/// Runes:
/// *   `♨` (Heat) - **Thermodynamics**: Increases reaction speed or energy in adjacent cells.
/// *   `Ϡ` (Catalyze) - **Reaction Trigger**: Reads catalyst ID from West, triggers reaction on North/South ingredients.
pub fn apply_catalyst_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    let mut changes = false;

    // Helper to get neighbor signals
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].as_ref()
    } else {
        None
    };

    match rune {
        "♨" => {
            // Heat: If West > 0, emit Heat to all neighbors (N, E, S).
            if let Some(Value::Int(intensity)) = w_sig {
                if *intensity > 0 {
                    let heat_val = Value::Int(*intensity);

                    // Propagate to neighbors
                    let neighbors = [(-1, 0), (0, 1), (1, 0)];
                    for (dy, dx) in neighbors {
                        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                            if next_signals[ny][nx] != Some(heat_val.clone()) {
                                next_signals[ny][nx] = Some(heat_val.clone());
                                changes = true;
                            }
                        }
                    }
                }
            }
        }
        "Ϡ" => {
            // Catalyze: Reads Catalyst ID (West). Checks Ingredients (North, South).
            // Emits Result (East).
            if let Some(Value::Int(_cat_id)) = w_sig {
                // Logic handled in Sink phase for side effects (DNA mutation).
                // Here we just propagate a "Trigger" signal to East if conditions met.

                let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                    current_signals[ny][nx].as_ref()
                } else {
                    None
                };

                let s_sig = if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                    current_signals[sy][sx].as_ref()
                } else {
                    None
                };

                if n_sig.is_some() && s_sig.is_some() {
                    // Conditions met for reaction
                    if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                        if next_signals[ey][ex].is_none() {
                            next_signals[ey][ex] = Some(Value::Int(1)); // Success Signal
                            changes = true;
                        }
                    }
                }
            }
        }
        _ => {}
    }

    changes
}

/// Applies Catalyst Sinks (Side Effects).
///
/// Runes:
/// *   `Ϡ` (Catalyze) - **Reaction**: Consumes ingredients, modifies DNA or Grid.
pub fn apply_catalyst_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    if rune == "Ϡ" {
        // West: Catalyst ID
        let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
            vm.prologue_state.signal_grid[wy][wx].clone()
        } else {
            None
        };

        if let Some(Value::Int(cat_id)) = w_sig {
            // Check for valid catalyst in VM
            // Catalyst IDs are 1-based usually
            let cat_idx = vm.catalysts.iter().position(|c| c.id == cat_id as u64);

            if let Some(idx) = cat_idx {
                // Apply Catalyst Logic
                // Consumes Energy
                if vm.energy >= 5 {
                    vm.energy -= 5;

                    // Example Effect: Transmute North/South based on Recipe
                    // This mirrors `catalyze` op but on grid context

                    // For now, let's just log success and emit visual flash
                    vm.output.push(format!("CATALYST: Triggered #{} at {},{}", cat_id, x, y));

                    // Flash effect
                    #[cfg(feature = "nova")]
                    {
                        vm.reactor_flash[y][x] = 255;
                    }

                    // Decrease charge
                    if vm.catalysts[idx].charge > 0 {
                        vm.catalysts[idx].charge -= 1;
                    }
                }
            }
        }
    }
}
