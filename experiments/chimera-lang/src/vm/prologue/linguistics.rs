use crate::vm::Value;
use crate::ast::JunctionType;
use super::normalize_coords;
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
                let strings: Vec<String> = list.iter().map(|v| match v {
                    Value::Str(s) => s.clone(),
                    _ => format!("{}", v),
                }).collect();
                let joined = strings.join(delim);

                let res = Value::Str(joined);
                if next_signals[y][x] != Some(res.clone()) {
                    next_signals[y][x] = Some(res);
                    changes = true;
                }
            } else if let (Some(Value::Str(_s1)), Some(Value::Str(_s2))) = (w_sig, n_sig) {
                 // Fallback: If West is String, concat with North (using empty delim implied or just direct concat)
                 // But rune implies Join. If North is delim, we might expect Concat.
                 // Let's support simple string concat if West is String.
                 // Actually, if West is String, Join with Delim doesn't make much sense unless we interpret West as a list of chars?
                 // Let's strict to List. Or maybe "Concat" behavior if West is Str.
                 // "Concat" behavior: West + Delim + (Wait, where is the other string?)
                 // Let's keep it strictly Join for List.
            }
        }
        _ => {}
    }

    changes
}
