use super::normalize_coords;
use crate::vm::Value;

/// Performs the `apply_list_runes` operation.
///
/// ## Examples
///
/// ```text
/// // Example usage of apply_list_runes
/// ```
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

            if let (Some(Value::Junction(t, mut items)), Some(mask)) = (w_sig, n_sig) {
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

                // Optimization: using `.retain` avoids an intermediate `.collect::<Vec<_>>()` allocation.
                items.retain(|v| *v != mask);
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Junction(t, items));
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

            if let (Some(Value::Junction(t, mut items)), Some(Value::Int(n))) = (w_sig, n_sig) {
                let count = n.max(0) as usize;
                // Optimization: using `.truncate` on an already-owned Vec avoids an intermediate `.collect::<Vec<_>>()` allocation.
                items.truncate(count);
                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Junction(t, items));
                    changes = true;
                }
            }
        }
        "E" => {
            // Exists: West (List), North (Pattern) -> Self (Int 1 if found)
            let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                current_signals[ny][nx].clone()
            } else {
                None
            };
            if let (Some(Value::Junction(_t, items)), Some(pattern)) = (w_sig, n_sig) {
                let exists = items.into_iter().any(|v| v == pattern);
                if exists && next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Int(1));
                    changes = true;
                }
            }
        }
        "Z" => {
            // Zip (Hyper-Op): West (List), North (Operator), East (List) -> Self (Zipped List)
            let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                current_signals[ny][nx].clone()
            } else {
                None
            };
            let e_sig = if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                current_signals[ey][ex].clone()
            } else {
                None
            };

            if let (
                Some(Value::Junction(t1, items_w)),
                Some(Value::Str(op)),
                Some(Value::Junction(t2, items_e)),
            ) = (w_sig, n_sig, e_sig)
            {
                let len = items_w.len().min(items_e.len());
                let mut zipped = Vec::with_capacity(len);
                for i in 0..len {
                    let w = items_w[i].clone();
                    let e = items_e[i].clone();
                    // Apply operator directly since apply_binary_op logic supports Values
                    // For zip, we generally expect Int operations for + - * / %
                    match (w, op.as_str(), e) {
                        (Value::Int(w_val), "+", Value::Int(e_val)) => {
                            zipped.push(Value::Int(w_val.wrapping_add(e_val)));
                        }
                        (Value::Int(w_val), "-", Value::Int(e_val)) => {
                            zipped.push(Value::Int(w_val.wrapping_sub(e_val)));
                        }
                        (Value::Int(w_val), "*", Value::Int(e_val)) => {
                            zipped.push(Value::Int(w_val.wrapping_mul(e_val)));
                        }
                        (Value::Int(w_val), "/", Value::Int(e_val)) => {
                            if e_val != 0 {
                                zipped.push(Value::Int(w_val.wrapping_div(e_val)));
                            } else {
                                zipped.push(Value::Int(0));
                            }
                        }
                        (Value::Int(w_val), "%", Value::Int(e_val)) => {
                            if e_val != 0 {
                                zipped.push(Value::Int(w_val.wrapping_rem(e_val)));
                            } else {
                                zipped.push(Value::Int(0));
                            }
                        }
                        (Value::Str(w_val), "+", Value::Str(e_val)) => {
                            zipped.push(Value::Str(format!("{}{}", w_val, e_val)));
                        }
                        // Fallback: If operation doesn't match or is unsupported, push a 0 Int
                        _ => zipped.push(Value::Int(0)),
                    }
                }

                // Determine resultant junction type
                let res_type = match (t1, t2) {
                    (crate::ast::JunctionType::All, _) => crate::ast::JunctionType::All,
                    (_, crate::ast::JunctionType::All) => crate::ast::JunctionType::All,
                    (crate::ast::JunctionType::Dish, _) => crate::ast::JunctionType::Dish,
                    (_, crate::ast::JunctionType::Dish) => crate::ast::JunctionType::Dish,
                    _ => crate::ast::JunctionType::Any,
                };

                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Junction(res_type, zipped));
                    changes = true;
                }
            }
        }
        _ => {}
    }
    changes
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::ast::JunctionType;
    use crate::vm::Value;

    #[test]
    fn apply_list_runes_test() {
        let mut current_signals = vec![vec![None; 16]; 16];
        let mut next_signals = vec![vec![None; 16]; 16];

        // Setup E Test: West=Junction(Any, [Int(5), Int(10)]), North=Int(10)
        let j_val = Value::Junction(JunctionType::Any, vec![Value::Int(5), Value::Int(10)]);
        current_signals[2][1] = Some(j_val.clone()); // West
        current_signals[1][2] = Some(Value::Int(10)); // North

        // Run E
        let changed = apply_list_runes("E", 2, 2, &current_signals, &mut next_signals);
        assert!(changed);
        assert_eq!(next_signals[2][2], Some(Value::Int(1))); // Exists

        // Reset
        let mut next_signals = vec![vec![None; 16]; 16];

        // Setup E Test (Not Found): North=Int(11)
        current_signals[1][2] = Some(Value::Int(11));
        let changed = apply_list_runes("E", 2, 2, &current_signals, &mut next_signals);
        assert!(!changed);
        assert_eq!(next_signals[2][2], None);

        // Reset
        let mut next_signals = vec![vec![None; 16]; 16];

        // Setup Z Test: West=Junction([1, 2, 3]), North=Str("+"), East=Junction([4, 5, 6])
        let j_w = Value::Junction(
            JunctionType::Any,
            vec![Value::Int(1), Value::Int(2), Value::Int(3)],
        );
        let j_e = Value::Junction(
            JunctionType::Any,
            vec![Value::Int(4), Value::Int(5), Value::Int(6)],
        );
        current_signals[2][1] = Some(j_w); // West
        current_signals[1][2] = Some(Value::Str("+".to_string())); // North
        current_signals[2][3] = Some(j_e); // East

        // Run Z
        let changed = apply_list_runes("Z", 2, 2, &current_signals, &mut next_signals);
        assert!(changed);

        let expected_zip = Value::Junction(
            JunctionType::Any,
            vec![Value::Int(5), Value::Int(7), Value::Int(9)],
        );
        assert_eq!(next_signals[2][2], Some(expected_zip));
    }
}
