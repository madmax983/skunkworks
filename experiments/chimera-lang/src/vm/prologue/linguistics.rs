use super::normalize_coords;
use crate::vm::Value;
use regex::Regex;

pub fn apply_linguistics_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    grid: &[Vec<Value>],
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };
    let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
        current_signals[ny][nx].clone()
    } else {
        None
    };

    match rune {
        "\"" => {
            // Quote:
            // 1. Scan Westward on the GRID for a matching Quote. Capture range.
            // 2. If no matching Quote found, Stringify the West SIGNAL.

            let mut captured_chars = Vec::new();
            let mut scan_x = x as i64 - 1;
            let mut found_start = false;

            while scan_x >= 0 {
                if let Some((sy, sx)) = normalize_coords(y as i64, scan_x) {
                    let val = &grid[sy][sx];
                    match val {
                        Value::Str(s) if s == "\"" => {
                            found_start = true;
                            break;
                        }
                        Value::Str(s) => {
                             captured_chars.push(s.clone());
                        }
                        Value::Int(n) => {
                             captured_chars.push(n.to_string());
                        }
                        _ => {}
                    }
                }
                scan_x -= 1;
            }

            if found_start {
                captured_chars.reverse();
                let result_str = captured_chars.join("");

                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Str(result_str));
                    changes = true;
                }
            } else if let Some(val) = w_sig {
                // Stringify Fallback
                let s = val.to_string();

                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Str(s));
                    changes = true;
                }
            }
        }
        "®" => {
            // Regex Match: West (Text), North (Pattern) -> List of Matches
            if let (Some(Value::Str(text)), Some(Value::Str(pattern))) = (w_sig, n_sig) {
                if let Ok(re) = Regex::new(&pattern) {
                    let matches: Vec<Value> = re.find_iter(&text)
                        .map(|m| Value::Str(m.as_str().to_string()))
                        .collect();

                    if next_signals[y][x].is_none() {
                        next_signals[y][x] = Some(Value::Junction(crate::ast::JunctionType::Any, matches));
                        changes = true;
                    }
                }
            }
        }
        ";" => {
            // Parse/Split: West (Text), North (Delimiter) -> List
            if let (Some(Value::Str(text)), Some(Value::Str(delim))) = (w_sig, n_sig) {
                let parts: Vec<Value> = text.split(&delim)
                    .map(|s| Value::Str(s.to_string()))
                    .collect();

                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Junction(crate::ast::JunctionType::Any, parts));
                    changes = true;
                }
            }
        }
        "©" => {
            // Concat/Join: West (List), North (Separator) -> String

            let sep = if let Some(Value::Str(s)) = n_sig {
                s
            } else {
                "".to_string()
            };

            if let Some(val) = w_sig {
                let result = match val {
                    Value::Junction(_, items) => {
                        let strings: Vec<String> = items.iter().map(|v| {
                            match v {
                                Value::Str(s) => s.clone(),
                                _ => v.to_string(),
                            }
                        }).collect();
                        strings.join(&sep)
                    }
                    Value::Str(s) => s,
                    Value::Int(n) => n.to_string(),
                    _ => "".to_string(),
                };

                if next_signals[y][x].is_none() {
                    next_signals[y][x] = Some(Value::Str(result));
                    changes = true;
                }
            }
        }
        _ => {}
    }
    changes
}
