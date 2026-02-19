use super::normalize_coords;
use crate::vm::Value;

pub fn apply_void_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    let mut changes = false;

    match rune {
        "µ" => {
            // Vacuum: West (Empty) -> Self (1)
            // Inverts emptiness.
            let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                current_signals[wy][wx].clone()
            } else {
                None
            };

            if is_empty_signal(&w_sig) {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(1));
                    changes = true;
                }
            } else {
                // If West is NOT empty, we output nothing (or 0?)
                // Prologue generally uses presence of signal as True.
                // Vacuum detects *absence*.
                // If absence -> 1. If presence -> None (or 0).
            }
        }
        "Ø" => {
            // Void Anchor: N, S, E, W (All Empty) -> Self (1)
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            let mut all_empty = true;
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if !is_empty_signal(&current_signals[ny][nx]) {
                        all_empty = false;
                        break;
                    }
                }
            }
            if all_empty {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(1));
                    changes = true;
                }
            }
        }
        "§" => {
            // Singularity: Consumes all neighbor signals.
            // Always active itself? Or only active if it consumed something?
            // "Attracts agents and consumes signals".
            // Let's make it always emit 1 (as a beacon) and consume neighbors.
            if next_signals[y][x].is_none() {
                next_signals[y][x] = Some(Value::Int(1));
                changes = true;
            }

            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    // Clear neighbor in next_signals
                    if next_signals[ny][nx].is_some() {
                        next_signals[ny][nx] = None;
                        changes = true;
                    }
                }
            }
        }
        _ => {}
    }

    changes
}

fn is_empty_signal(sig: &Option<Value>) -> bool {
    match sig {
        None => true,
        Some(Value::Int(0)) => true,
        Some(Value::Str(s)) if s.is_empty() => true,
        _ => false,
    }
}
