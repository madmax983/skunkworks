use super::normalize_coords;
use crate::vm::Value;
use std::collections::HashMap;

pub fn apply_teleport_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut [Vec<Option<Value>>],
    teleport_channels: &mut HashMap<i64, Value>,
) -> bool {
    let mut changes = false;
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

    match rune {
        "{" => {
            // Teleport Send: West (Value), North (Channel) -> Void
            if let (Some(val), Some(Value::Int(channel))) = (w_sig, n_sig) {
                teleport_channels.insert(channel, val.clone());
                // Light up self to indicate activity
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(1));
                    changes = true;
                }
            }
        }
        "}" => {
            // Teleport Recv: North (Channel) -> Self (Value)
            if let Some(Value::Int(channel)) = n_sig {
                if let Some(val) = teleport_channels.get(&channel) {
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(val.clone());
                        changes = true;
                    }
                }
            }
        }
        _ => {}
    }
    changes
}
