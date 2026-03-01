use super::normalize_coords;
use crate::vm::Value;

pub fn apply_math_runes(
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
        "A" => {
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (&w_sig, &e_sig) {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(w.wrapping_add(*e)));
                    changes = true;
                }
            }
        }
        "B" => {
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (&w_sig, &e_sig) {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(w.wrapping_sub(*e)));
                    changes = true;
                }
            }
        }
        "P" => {
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (&w_sig, &e_sig) {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(w.wrapping_mul(*e)));
                    changes = true;
                }
            }
        }
        "Q" => {
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (&w_sig, &e_sig) {
                if *e != 0 && next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(w.wrapping_div(*e)));
                    changes = true;
                }
            }
        }
        "=" => {
            if let (Some(w), Some(e)) = (&w_sig, &e_sig) {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(if w == e { 1 } else { 0 }));
                    changes = true;
                }
            }
        }
        ">" => {
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (&w_sig, &e_sig) {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(if w > e { 1 } else { 0 }));
                    changes = true;
                }
            }
        }
        "<" => {
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (&w_sig, &e_sig) {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(if w < e { 1 } else { 0 }));
                    changes = true;
                }
            }
        }
        "%" => {
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (&w_sig, &e_sig) {
                if *e != 0 && next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(w % e));
                    changes = true;
                }
            }
        }
        _ => {}
    }
    changes
}
