use super::normalize_coords;
use crate::vm::Value;

pub fn apply_alchemy_runes(
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
    let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
        current_signals[ny][nx].clone()
    } else {
        None
    };

    match rune {
        "t" => {
            // Transmute: West (Value), North (Mode) -> Self
            if let Some(val) = w_sig {
                let mode = match n_sig {
                    Some(Value::Int(m)) => m,
                    _ => 0, // Default mode 0
                };

                let result = match mode {
                    0 => {
                        // ToString
                        match val {
                            Value::Int(n) => Some(Value::Str(n.to_string())),
                            Value::Str(s) => Some(Value::Str(s)),
                            _ => Some(Value::Str(format!("{:?}", val))),
                        }
                    }
                    1 => {
                        // ToInt
                        match val {
                            Value::Int(n) => Some(Value::Int(n)),
                            Value::Str(s) => {
                                if let Ok(n) = s.parse::<i64>() {
                                    Some(Value::Int(n))
                                } else if s.len() == 1 {
                                    Some(Value::Int(s.chars().next().unwrap() as i64))
                                } else {
                                    Some(Value::Int(0)) // Error case
                                }
                            }
                            _ => Some(Value::Int(0)),
                        }
                    }
                    2 => {
                        // Type ID
                        let type_id = match val {
                            Value::Int(_) => 0,
                            Value::Str(_) => 1,
                            Value::Junction(_, _) => 2,
                            Value::Superposition(_) => 3,
                            _ => -1,
                        };
                        Some(Value::Int(type_id))
                    }
                    3 => {
                        // Length
                        match val {
                            Value::Str(s) => Some(Value::Int(s.len() as i64)),
                            Value::Junction(_, items) => Some(Value::Int(items.len() as i64)),
                            _ => Some(Value::Int(0)),
                        }
                    }
                    _ => None,
                };

                if let Some(res) = result {
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(res);
                        changes = true;
                    }
                }
            }
        }
        "f" => {
            // Fuse: West + East -> Self
            if let (Some(left), Some(right)) = (w_sig, e_sig) {
                let result = match (left, right) {
                    (Value::Str(s1), Value::Str(s2)) => Some(Value::Str(format!("{}{}", s1, s2))),
                    (Value::Str(s), Value::Int(n)) => {
                        let count = n.max(0) as usize;
                        Some(Value::Str(s.repeat(count)))
                    }
                    (Value::Int(n), Value::Str(s)) => {
                        let count = n.max(0) as usize;
                        Some(Value::Str(s.repeat(count)))
                    }
                    (Value::Int(n1), Value::Int(n2)) => Some(Value::Int(n1 + n2)),
                    (Value::Junction(t, mut items), val) => {
                        items.push(val);
                        Some(Value::Junction(t, items))
                    }
                    (val, Value::Junction(t, mut items)) => {
                        items.insert(0, val);
                        Some(Value::Junction(t, items))
                    }
                    _ => None,
                };

                if let Some(res) = result {
                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(res);
                        changes = true;
                    }
                }
            }
        }
        "d" => {
            // Distill: West (Value) -> North (Head), South (Tail)
            if let Some(val) = w_sig {
                let (head, tail) = match val {
                    Value::Str(s) => {
                        if !s.is_empty() {
                            let mut chars = s.chars();
                            let h = chars.next().unwrap().to_string();
                            let t = chars.collect::<String>();
                            (Some(Value::Str(h)), Some(Value::Str(t)))
                        } else {
                            (None, None)
                        }
                    }
                    Value::Int(n) => {
                        if n >= 10 {
                            (Some(Value::Int(n / 10)), Some(Value::Int(n % 10)))
                        } else {
                            (Some(Value::Int(0)), Some(Value::Int(n)))
                        }
                    }
                    Value::Junction(t, items) => {
                        if !items.is_empty() {
                            let h = items[0].clone();
                            let rest = items[1..].to_vec();
                            (Some(h), Some(Value::Junction(t, rest)))
                        } else {
                            (None, None)
                        }
                    }
                    _ => (None, None),
                };

                // Output North
                if let Some(h) = head {
                    if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                        if next_signals[ny][nx].is_none() {
                            next_signals[ny][nx] = Some(h);
                            changes = true;
                        }
                    }
                }
                // Output South
                if let Some(t) = tail {
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        if next_signals[sy][sx].is_none() {
                            next_signals[sy][sx] = Some(t);
                            changes = true;
                        }
                    }
                }
                // Light up self if successful? Or maybe not needed if outputs are set.
                // Let's light up self to show activity.
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
