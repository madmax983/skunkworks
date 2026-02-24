use super::normalize_coords;
use crate::ast::JunctionType;
use crate::vm::{Value, MAX_COMPLEX_STRING_LEN};
use regex::Regex;
use serde_json;

pub fn apply_linguistics_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut [Vec<Option<Value>>],
) -> bool {
    let mut changes = false;

    // Helper to get neighbor signals
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].as_ref()
    } else {
        None
    };

    let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
        current_signals[ny][nx].as_ref()
    } else {
        None
    };

    match rune {
        "\"" => {
            // Stringify: West -> Self (as String)
            if let Some(val) = w_sig {
                let new_val = match val {
                    Value::Str(s) => Value::Str(s.clone()),
                    Value::Junction(JunctionType::Dish, _) => {
                        if let Ok(json) = serde_json::to_string(val) {
                            Value::Str(json)
                        } else {
                            Value::Str(format!("{}", val))
                        }
                    }
                    _ => Value::Str(format!("{}", val)),
                };

                if next_signals[y][x] != Some(new_val.clone()) {
                    next_signals[y][x] = Some(new_val);
                    changes = true;
                }
            }
        }
        "®" => {
            // Regex Match: West (Target), North (Pattern) -> Self (1/0)
            if let (Some(Value::Str(target)), Some(Value::Str(pattern))) = (w_sig, n_sig) {
                let is_match = if let Ok(re) = Regex::new(pattern) {
                    re.is_match(target)
                } else {
                    false
                };

                let res = Value::Int(if is_match { 1 } else { 0 });
                if next_signals[y][x] != Some(res.clone()) {
                    next_signals[y][x] = Some(res);
                    changes = true;
                }
            }
        }
        ";" => {
            // Split: West (String), North (Delim) -> Self (List)
            if let (Some(Value::Str(target)), Some(Value::Str(delim))) = (w_sig, n_sig) {
                let parts: Vec<Value> = target
                    .split(delim)
                    .map(|s| Value::Str(s.to_string()))
                    .collect();

                let res = Value::Junction(JunctionType::Any, parts);
                if next_signals[y][x] != Some(res.clone()) {
                    next_signals[y][x] = Some(res);
                    changes = true;
                }
            }
        }
        "©" => {
            // Join: West (List), North (Delim) -> Self (String)
            if let (Some(Value::Junction(_, list)), Some(Value::Str(delim))) = (w_sig, n_sig) {
                let strings: Vec<String> = list
                    .iter()
                    .map(|v| match v {
                        Value::Str(s) => s.clone(),
                        _ => format!("{}", v),
                    })
                    .collect();
                let joined = strings.join(delim);

                let res = Value::Str(joined);
                if next_signals[y][x] != Some(res.clone()) {
                    next_signals[y][x] = Some(res);
                    changes = true;
                }
            }
        }
        "↑" => {
            // Shift Up: West (String) -> Self (Upper)
            if let Some(val) = w_sig {
                let s = match val {
                    Value::Str(s) => s.clone(),
                    _ => format!("{}", val),
                };
                let res = Value::Str(s.to_uppercase());
                if next_signals[y][x] != Some(res.clone()) {
                    next_signals[y][x] = Some(res);
                    changes = true;
                }
            }
        }
        "↓" => {
            // Shift Down: West (String) -> Self (Lower)
            if let Some(val) = w_sig {
                let s = match val {
                    Value::Str(s) => s.clone(),
                    _ => format!("{}", val),
                };
                let res = Value::Str(s.to_lowercase());
                if next_signals[y][x] != Some(res.clone()) {
                    next_signals[y][x] = Some(res);
                    changes = true;
                }
            }
        }
        "≅" => {
            // Approx Equal: West (A), North (B) -> Self (Distance)
            if let (Some(w_val), Some(n_val)) = (w_sig, n_sig) {
                let s1 = match w_val {
                    Value::Str(s) => s.clone(),
                    _ => format!("{}", w_val),
                };
                let s2 = match n_val {
                    Value::Str(s) => s.clone(),
                    _ => format!("{}", n_val),
                };

                // 🔒 WARDEN: DoS Protection
                if s1.len() > MAX_COMPLEX_STRING_LEN || s2.len() > MAX_COMPLEX_STRING_LEN {
                    // Do not produce output (silence) or output Error code (-1)
                    // Silence is safer.
                    // But if we want to signal error, maybe -1?
                    // Previous behavior was crash/hang. Silence is fine.
                    // If we return, we skip update.
                    return false;
                }

                let dist = levenshtein_distance(&s1, &s2);
                let res = Value::Int(dist as i64);
                if next_signals[y][x] != Some(res.clone()) {
                    next_signals[y][x] = Some(res);
                    changes = true;
                }
            }
        }
        _ => {}
    }

    changes
}

fn levenshtein_distance(s1: &str, s2: &str) -> usize {
    let s1_chars: Vec<char> = s1.chars().collect();
    let s2_chars: Vec<char> = s2.chars().collect();
    let n = s1_chars.len();
    let m = s2_chars.len();

    if n == 0 {
        return m;
    }
    if m == 0 {
        return n;
    }

    // Optimization: Use 2 rows to reduce memory from O(N*M) to O(min(N,M))
    let (short, long) = if n < m {
        (&s1_chars, &s2_chars)
    } else {
        (&s2_chars, &s1_chars)
    };

    let min_len = short.len();
    let max_len = long.len();

    let mut prev_row: Vec<usize> = (0..=min_len).collect();
    let mut curr_row: Vec<usize> = vec![0; min_len + 1];

    for i in 1..=max_len {
        curr_row[0] = i;
        for j in 1..=min_len {
            let cost = if long[i - 1] == short[j - 1] { 0 } else { 1 };
            curr_row[j] = std::cmp::min(
                std::cmp::min(curr_row[j - 1] + 1, prev_row[j] + 1),
                prev_row[j - 1] + cost,
            );
        }
        prev_row.clone_from(&curr_row);
    }

    prev_row[min_len]
}
