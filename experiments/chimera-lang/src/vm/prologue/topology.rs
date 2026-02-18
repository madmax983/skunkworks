use super::normalize_coords;
use crate::vm::Value;

pub fn apply_topology_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    next_delayed: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    let mut changes = false;
    match rune {
        "~" => {
            // Wires: OR of all neighbors
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if let Some(sig) = &current_signals[ny][nx] {
                        if next_signals[y][x].is_none() {
                            next_signals[y][x] = Some(sig.clone());
                            changes = true;
                        }
                    }
                }
            }
        }
        "^" | "J" => {
            // Jump: Input West -> Output East (Skipping Self)
            if let (Some((wy, wx)), Some((ey, ex))) = (
                normalize_coords(y as i64, x as i64 - 1),
                normalize_coords(y as i64, x as i64 + 1),
            ) {
                if let Some(sig) = &current_signals[wy][wx] {
                    if next_signals[ey][ex].is_none() {
                        next_signals[ey][ex] = Some(sig.clone());
                        changes = true;
                    }
                }
            }
        }
        "*" => {
            // Splitter: Input North -> Output Self
            if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                if let Some(sig) = &current_signals[ny][nx] {
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(sig.clone());
                        changes = true;
                    }
                }
            }
        }
        "#" => {
            // Delay: Input North -> Output to next_delayed
            if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                if let Some(sig) = &current_signals[ny][nx] {
                    if next_delayed[y][x].is_none() {
                        next_delayed[y][x] = Some(sig.clone());
                    }
                }
            }
        }
        _ => {}
    }
    changes
}
