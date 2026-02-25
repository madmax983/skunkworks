use super::normalize_coords;
use crate::vm::nova_void::VoidRift;
use crate::vm::Value;
use std::collections::VecDeque;

pub fn apply_void_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    void_buffer: &mut VecDeque<Value>,
    void_rifts: &mut Vec<VoidRift>,
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
                            if void_buffer.len() < crate::vm::MAX_VOID_BUFFER_SIZE {
                                void_buffer.push_back(val.clone());
                                next_signals[y][x] = Some(Value::Int(1)); // Signal activation
                                changes = true;
                            }
                        }
                    }
                }
            }
        }
        "§" => {
            // Void Out: Pop from Void Buffer -> Self.
            // If buffer empty, check for local Rift to drain.
            if next_signals[y][x].is_none() {
                if let Some(val) = void_buffer.pop_back() {
                    next_signals[y][x] = Some(val);
                    changes = true;
                } else {
                    // Buffer empty, check for Rift at (y, x)
                    if let Some(rift) = void_rifts.iter_mut().find(|r| r.location == (y, x)) {
                        if rift.severity > 0 {
                            rift.severity = rift.severity.saturating_sub(1);
                            // Emit Rift Power (Severity)
                            next_signals[y][x] = Some(Value::Int(rift.severity as i64));
                            changes = true;
                        }
                    }
                }
            }
        }
        "ꝏ" => {
            // Infinity: Peek Void Buffer (Non-destructive).
            // LIFO Top is back().
            if next_signals[y][x].is_none() {
                if let Some(val) = void_buffer.back() {
                    next_signals[y][x] = Some(val.clone());
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
