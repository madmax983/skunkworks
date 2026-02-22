use super::normalize_coords;
use crate::vm::Value;
use std::collections::VecDeque;

pub fn apply_void_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    void_buffer: &mut VecDeque<Value>,
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
            }
        }
        "Ø" => {
            // Void In: West (Value) -> Push to Void Buffer.
            // Only fires if we haven't already fired in this tick (checked via next_signals).
            if next_signals[y][x].is_none() {
                if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                    let w_sig = &current_signals[wy][wx];
                    if !is_empty_signal(w_sig) {
                        if let Some(val) = w_sig {
                            void_buffer.push_back(val.clone());
                            next_signals[y][x] = Some(Value::Int(1)); // Signal activation
                            changes = true;
                        }
                    }
                }
            }
        }
        "§" => {
            // Void Out: Pop from Void Buffer -> Self.
            // Only fires if we haven't already fired.
            if next_signals[y][x].is_none() {
                if let Some(val) = void_buffer.pop_back() {
                    next_signals[y][x] = Some(val);
                    changes = true;
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
