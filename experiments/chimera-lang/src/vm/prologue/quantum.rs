use crate::vm::Value;
use super::normalize_coords;
use rand::Rng;

pub fn apply_quantum_runes(
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
        "q" => {
            // Quantum Source: West -> Self (Superposition)
            if let Some(val) = w_sig {
                if next_signals[y][x].is_none() {
                    let new_val = match val {
                        Value::Int(n) => {
                            // Create superposition of n and n+1
                            Value::Superposition(vec![
                                (Value::Int(n), 0.5),
                                (Value::Int(n + 1), 0.5),
                            ])
                        }
                        _ => val, // Pass through others
                    };
                    next_signals[y][x] = Some(new_val);
                    changes = true;
                }
            }
        }
        "m" => {
            // Measure: West -> Self (Collapsed)
            if let Some(val) = w_sig {
                if next_signals[y][x].is_none() {
                    let collapsed = match val {
                        Value::Superposition(states) => {
                            let mut rng = rand::thread_rng();
                            let r: f64 = rng.gen();
                            let mut cumulative = 0.0;
                            let mut selected = states.last().map(|(v, _)| v.clone()).unwrap_or(Value::Int(0));

                            for (v, p) in states {
                                cumulative += p;
                                if r <= cumulative {
                                    selected = v;
                                    break;
                                }
                            }
                            selected
                        }
                        _ => val, // Already measured
                    };
                    next_signals[y][x] = Some(collapsed);
                    changes = true;
                }
            }
        }
        _ => {}
    }
    changes
}
