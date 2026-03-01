use super::normalize_coords;
use crate::vm::{ChimeraVM, Value};

/// Applies Narrative Runes (Logic Phase).
///
/// Runes:
/// *   `α` (Alpha) - **Incipit**: Reads West (Seed). Generates Theme (East).
/// *   `ω` (Omega) - **Terminus**: Reads West (Story). Collapses to Outcome (East).
/// *   `✍` (Hand) - **Revision**: Reads West (Story) + North (Edit). Outputs Edited (East).
/// *   `?` (Twist) - **Plot Twist**: Reads West (Story). Introduces random event (East).
pub fn apply_narrative_runes(
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

    let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
        current_signals[ny][nx].as_ref()
    } else {
        None
    };

    match rune {
        "α" => {
            // Incipit: West (Seed) -> East (Theme)
            if let Some(val) = w_sig {
                let seed = match val {
                    Value::Int(n) => *n,
                    Value::Str(s) => s.len() as i64,
                    _ => 0,
                };

                let themes = [
                    "Hero", "Shadow", "Journey", "Treasure", "Home", "Love", "War", "Time",
                ];
                let idx = (seed.unsigned_abs() as usize) % themes.len();
                let theme = themes[idx];

                let res = Value::Str(theme.to_string());

                if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                    if next_signals[ey][ex] != Some(res.clone()) {
                        next_signals[ey][ex] = Some(res);
                        changes = true;
                    }
                }
            }
        }
        "ω" => {
            // Terminus: West (Story) -> East (Outcome)
            if let Some(Value::Str(story)) = w_sig {
                let outcome = if story.contains("Hero") && story.contains("Shadow") {
                    "Conflict"
                } else if story.contains("Hero") && story.contains("Treasure") {
                    "Victory"
                } else if story.contains("Journey") && story.contains("Home") {
                    "Return"
                } else if story.contains("Love") && story.contains("War") {
                    "Tragedy"
                } else {
                    "Entropy"
                };

                let res = Value::Str(outcome.to_string());

                if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                    if next_signals[ey][ex] != Some(res.clone()) {
                        next_signals[ey][ex] = Some(res);
                        changes = true;
                    }
                }
            }
        }
        "✍" => {
            // Revision: West (Story) + North (Edit) -> East (Edited)
            if let (Some(Value::Str(story)), Some(Value::Str(edit))) = (w_sig, n_sig) {
                let edited = match edit.as_str() {
                    "rev" => story.chars().rev().collect::<String>(),
                    "up" => story.to_uppercase(),
                    "low" => story.to_lowercase(),
                    "cut" => story.chars().take(story.len() / 2).collect(),
                    "len" => format!("{}", story.len()),
                    _ => story.clone(),
                };

                let res = Value::Str(edited);

                if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                    if next_signals[ey][ex] != Some(res.clone()) {
                        next_signals[ey][ex] = Some(res);
                        changes = true;
                    }
                }
            }
        }
        "?" => {
            // Twist: West (Story) -> East (Twist)
            if let Some(Value::Str(story)) = w_sig {
                let twists = [" suddenly died", " woke up", " found a key", " was a dream"];
                // Deterministic pseudorandom based on story hash or length
                let idx = story.len() % twists.len();
                let twist = format!("{}{}", story, twists[idx]);

                if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                    if next_signals[ey][ex].is_none() {
                        next_signals[ey][ex] = Some(Value::Str(twist));
                        changes = true;
                    }
                }
            }
        }
        _ => {}
    }

    changes
}

/// Applies Narrative Sinks (Side Effects).
///
/// Runes:
/// *   `📖` (Book) - **Library**:
///     *   Mode 0 (Read): Reads West (Key). Emits Value to Self.
///     *   Mode 1 (Write): Reads West (Key) + South (Value). Writes to Library.
pub fn apply_narrative_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    if rune == "📖" {
        // Library Access
        // Needs West (Key) and North (Mode)
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

        if let (Some(Value::Str(key)), Some(Value::Int(mode))) = (w_sig, n_sig) {
            if mode == 1 {
                // WRITE: Key (West) + Value (South)
                if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                    // Read from signal grid
                    if let Some(val) = &vm.prologue_state.signal_grid[sy][sx] {
                        vm.prologue_state.library.insert(key.clone(), val.clone());
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1)); // Ack
                                                                                   // Log
                        vm.output
                            .push(format!("NARRATIVE: Wrote '{}' to Library.", key));
                    }
                }
            } else {
                // READ: Key (West) -> Emit to Self
                if let Some(val) = vm.prologue_state.library.get(&key) {
                    vm.prologue_state.signal_grid[y][x] = Some(val.clone());
                }
            }
        }
    }
}
