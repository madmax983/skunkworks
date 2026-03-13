use super::normalize_coords;
use crate::vm::Value;

pub fn apply_list_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut [Vec<Option<Value>>],
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "[" => {
            // Collect: N, E, S, W -> Junction
            let mut items = Vec::new();
            // Clockwise from North
            let neighbors = [(-1, 0), (0, 1), (1, 0), (0, -1)]; // N E S W
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
                    if let Some(sig) = &current_signals[ny][nx] {
                        items.push(sig.clone());
                    }
                }
            }
            if !items.is_empty() && next_signals[y][x].is_none() {
                next_signals[y][x] = Some(Value::Junction(crate::ast::JunctionType::Any, items));
                changes = true;
            }
        }
        "]" => {
            // Scatter: West (Junction) -> N, E, S
            if let Some(Value::Junction(_, items)) = w_sig {
                // Distribute items
                // Item 0 -> N
                if !items.is_empty() {
                    if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                        if next_signals[ny][nx].is_none() {
                            next_signals[ny][nx] = Some(items[0].clone());
                            changes = true;
                        }
                    }
                }
                // Item 1 -> E
                if items.len() > 1 {
                    if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                        if next_signals[ey][ex].is_none() {
                            next_signals[ey][ex] = Some(items[1].clone());
                            changes = true;
                        }
                    }
                }
                // Item 2 -> S
                if items.len() > 2 {
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        if next_signals[sy][sx].is_none() {
                            next_signals[sy][sx] = Some(items[2].clone());
                            changes = true;
                        }
                    }
                }
            }
        }
        "U" => {
            // Unwrap (Head): West -> Self
            if let Some(Value::Junction(_, items)) = w_sig {
                if let Some(head) = items.first() {
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(head.clone());
                        changes = true;
                    }
                }
            }
        }
        "V" => {
            // Vector (Tail): West -> Self
            if let Some(Value::Junction(t, items)) = w_sig {
                if items.len() > 1 {
                    let tail = items[1..].to_vec();
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(Value::Junction(t, tail));
                        changes = true;
                    }
                }
            }
        }
        "F" => {
            // Filter: West (List), North (Mask) -> Self
            let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                current_signals[ny][nx].clone()
            } else {
                None
            };

            if let (Some(Value::Junction(t, items)), Some(mask)) = (w_sig, n_sig) {
                // How does mask work?
                // If mask is Int(1), pass all?
                // If mask is a Junction of booleans?
                // Or maybe mask is just "Truthiness"?
                // Let's implement: If Mask is truthy, pass list? No that's trivial.
                // Maybe F applies a filter logic... but we don't have lambdas here easily.
                // Let's make it simple: Filter by Type? Or Filter non-empty?

                // Let's make it: Filter out items equal to Mask.
                // Or: Keep items equal to Mask?
                // Let's say: Remove items equal to Mask.

                let filtered: Vec<Value> = items.into_iter().filter(|v| *v != mask).collect();
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Junction(t, filtered));
                    changes = true;
                }
            }
        }
        "T" => {
            // Take: West (List), North (Count) -> Self
            let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                current_signals[ny][nx].clone()
            } else {
                None
            };

            if let (Some(Value::Junction(t, items)), Some(Value::Int(n))) = (w_sig, n_sig) {
                let count = n.max(0) as usize;
                let taken: Vec<Value> = items.into_iter().take(count).collect();
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Junction(t, taken));
                    changes = true;
                }
            }
        }
        _ => {}
    }
    changes
}
