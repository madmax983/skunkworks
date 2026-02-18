use super::normalize_coords;
use crate::vm::Value;

pub fn apply_logic_runes(
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
    let e_sig = if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
        current_signals[ey][ex].clone()
    } else {
        None
    };

    match rune {
        "&" => {
            if w_sig.is_some() && e_sig.is_some() {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(1));
                    changes = true;
                }
            }
        }
        "|" => {
            if w_sig.is_some() || e_sig.is_some() {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(1));
                    changes = true;
                }
            }
        }
        "+" => {
            // XOR
            if w_sig.is_some() ^ e_sig.is_some() {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(1));
                    changes = true;
                }
            }
        }
        "I" => {
            // If: West (Condition) != 0 -> Output North (Value) to Self
            // Need N input too
            let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                current_signals[ny][nx].clone()
            } else {
                None
            };

            if let Some(Value::Int(cond)) = w_sig {
                if cond != 0 {
                    if let Some(val) = n_sig {
                        if next_signals[y][x].is_none() {
                            next_signals[y][x] = Some(val);
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
