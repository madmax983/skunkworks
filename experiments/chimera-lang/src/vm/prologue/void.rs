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

    // Check neighbors
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
    let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
        current_signals[ny][nx].clone()
    } else {
        None
    };
    let s_sig = if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
        current_signals[sy][sx].clone()
    } else {
        None
    };

    match rune {
        "µ" => {
            // Vacuum: West (None) -> Self (1)
            // Inverter for signal presence
            if w_sig.is_none() {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(1));
                    changes = true;
                }
            }
        }
        "Ø" => {
            // Void Anchor: All Neighbors (None) -> Self (1)
            // 4-way NOR gate / Oscillator base
            if w_sig.is_none() && e_sig.is_none() && n_sig.is_none() && s_sig.is_none() {
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(1));
                    changes = true;
                }
            }
        }
        _ => {}
    }
    changes
}
