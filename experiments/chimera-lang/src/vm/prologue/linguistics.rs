use super::normalize_coords;
use crate::ast::JunctionType;
use crate::vm::Value;
use regex::Regex;

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

fn levenshtein_distance(a: &str, b: &str) -> usize {
    let len_a = a.chars().count();
    let len_b = b.chars().count();
    // Use 2 rows for space optimization if needed, but matrix is fine for small strings
    let mut matrix = vec![vec![0; len_b + 1]; len_a + 1];

    for i in 0..=len_a {
        matrix[i][0] = i;
    }
    for j in 0..=len_b {
        matrix[0][j] = j;
    }

    for (i, ca) in a.chars().enumerate() {
        for (j, cb) in b.chars().enumerate() {
            let cost = if ca == cb { 0 } else { 1 };
            matrix[i + 1][j + 1] = std::cmp::min(
                std::cmp::min(matrix[i][j + 1] + 1, matrix[i + 1][j] + 1),
                matrix[i][j] + cost,
            );
        }
    }
    matrix[len_a][len_b]
}
