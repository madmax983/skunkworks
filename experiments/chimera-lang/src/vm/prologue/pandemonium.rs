use super::normalize_coords;
use crate::vm::{ChimeraVM, Value};
use rand::Rng;
use rand::seq::SliceRandom;

pub fn apply_pandemonium_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    let mut changes = false;

    // Common input: West
    let input = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "¿" => {
            // Gamble: Reads West. 50% Double, 50% Zero. Output South.
            if let Some(val) = input {
                let mut rng = rand::thread_rng();
                let result = if rng.gen_bool(0.5) {
                    // Double
                    match val {
                        Value::Int(n) => Value::Int(n.saturating_mul(2)),
                        Value::Str(s) => Value::Str(format!("{}{}", s, s)),
                        _ => val,
                    }
                } else {
                    // Zero
                    Value::Int(0)
                };

                if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                    if next_signals[sy][sx].is_none() {
                        next_signals[sy][sx] = Some(result);
                        changes = true;
                    }
                }
            }
        }
        "≈" => {
            // Flux: Reads West. Output to Random Neighbor.
            if let Some(val) = input {
                let mut rng = rand::thread_rng();
                let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)]; // N, S, W, E
                let (dy, dx) = neighbors[rng.gen_range(0..4)];

                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if next_signals[ny][nx].is_none() {
                        next_signals[ny][nx] = Some(val);
                        changes = true;
                    }
                }
            }
        }
        "¡" => {
            // Scramble: Reads West (List). Shuffles list. Output South.
            if let Some(val) = input {
                if let Value::Junction(t, mut list) = val {
                    let mut rng = rand::thread_rng();
                    list.shuffle(&mut rng);
                    let result = Value::Junction(t, list);

                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        if next_signals[sy][sx].is_none() {
                            next_signals[sy][sx] = Some(result);
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

pub fn apply_pandemonium_sinks(_vm: &mut ChimeraVM, _rune: &str, _y: usize, _x: usize) {
    // Sinks moved to Runes or deprecated.
}
