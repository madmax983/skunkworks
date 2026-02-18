use crate::vm::Value;
use super::normalize_coords;

pub fn apply_optics_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    let mut changes = false;
    // Helper to get signal from neighbor
    let get_sig = |dy: i64, dx: i64| -> Option<Value> {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            current_signals[ny][nx].clone()
        } else {
            None
        }
    };

    // Helper to set signal to neighbor
    let mut set_sig = |dy: i64, dx: i64, val: Value| {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            if next_signals[ny][nx].is_none() {
                next_signals[ny][nx] = Some(val);
                changes = true;
            }
        }
    };

    match rune {
        "\\" => {
            // Mirror Back
            // N -> E, E -> N, S -> W, W -> S
            if let Some(s) = get_sig(-1, 0) { set_sig(0, 1, s); } // N -> E
            if let Some(s) = get_sig(0, 1) { set_sig(-1, 0, s); } // E -> N
            if let Some(s) = get_sig(1, 0) { set_sig(0, -1, s); } // S -> W
            if let Some(s) = get_sig(0, -1) { set_sig(1, 0, s); } // W -> S
        }
        "/" => {
            // Mirror Forward
            // N -> W, W -> N, S -> E, E -> S
            if let Some(s) = get_sig(-1, 0) { set_sig(0, -1, s); } // N -> W
            if let Some(s) = get_sig(0, -1) { set_sig(-1, 0, s); } // W -> N
            if let Some(s) = get_sig(1, 0) { set_sig(0, 1, s); } // S -> E
            if let Some(s) = get_sig(0, 1) { set_sig(1, 0, s); } // E -> S
        }
        "-" => {
            // Horizontal Beam
            // W <-> E
            if let Some(s) = get_sig(0, -1) { set_sig(0, 1, s); } // W -> E
            if let Some(s) = get_sig(0, 1) { set_sig(0, -1, s); } // E -> W
        }
        _ => {}
    }
    changes
}
