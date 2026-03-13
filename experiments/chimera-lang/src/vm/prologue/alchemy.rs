//! # Alchemy Runes ⚗️
//!
//! The Alchemy module provides runes for **Data Transformation** and **Type Casting**.
//! Unlike the elemental alchemy (Fire/Water), these runes operate on the raw `Value` types
//! (Integers, Strings, Lists) flowing through the Prologue grid.
//!
//! ## Philosophy
//!
//! Alchemy allows a circuit to modify the *nature* of a signal, not just its path.
//! It is essential for:
//! *   **Serialization**: converting numbers to strings for logging.
//! *   **Parsing**: converting user input strings to numbers.
//! *   **List Processing**: splitting and joining lists or strings.
//! *   **Type Introspection**: checking if a value is a number or text.
//!
//! ## Rune Reference
//!
//! | Rune | Name | Input | Output | Description |
//! |---|---|---|---|---|
//! | `t` | **Transmute** | West (Val), North (Mode) | Self | Converts types based on Mode (0=Str, 1=Int, 2=Type, 3=Len, 4=Liq, 5=Sol). |
//! | `f` | **Fuse** | West (A), East (B) | Self | Combines A and B (Concat, Add, Push). |
//! | `d` | **Distill** | West (Val) | North (Head), South (Tail) | Splits value into two parts. |

use super::normalize_coords;
use crate::ast::JunctionType;
use crate::vm::{Value, MAX_STRING_LEN};

/// Applies the logic for Alchemy runes (`t`, `f`, `d`).
///
/// This function is called during the propagation phase of the Prologue execution cycle.
///
/// # Arguments
///
/// * `rune` - The character representation of the rune (e.g., "t").
/// * `y`, `x` - The grid coordinates of the rune.
/// * `current_signals` - The state of signals at the start of this propagation step.
/// * `next_signals` - The buffer to write new signals to (Double Buffering).
///
/// # Returns
///
/// Returns `true` if any new signal was generated, prompting another propagation iteration.
///
/// # Examples
///
/// ## Transmutation (Mode 0: To String)
/// ```text
///   42  0
///   !   !
///   ~   t   -> "42"
/// ```
///
/// ## Fusion (String Concatenation)
/// ```text
///   "A" "B"
///    !   !
///    f       -> "AB"
/// ```
///
/// ## Distillation (Splitting)
/// ```text
///      "a" (North)
///       ^
///  "abc"-> d
///       v
///      "bc" (South)
/// ```
pub fn apply_alchemy_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut [Vec<Option<Value>>],
) -> bool {
    let mut changes = false;

    // Gather Inputs
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
            // Rune: Transmute (t)
            // Function: Type Conversion and Introspection
            // Inputs:
            //   - West: The value to transform.
            //   - North: The mode (Integer). Defaults to 0 if missing.
            // Modes:
            //   0: To String (Format)
            //   1: To Int (Parse)
            //   2: Type ID (0=Int, 1=Str, 2=List, 3=Quantum)
            //   3: Length (String len or List len)
            //   4: Liquefy (Str -> List<Int>)
            //   5: Solidify (List<Int> -> Str)

            if let Some(val) = w_sig {
                let mode = match n_sig {
                    Some(Value::Int(m)) => m,
                    _ => 0, // Default mode 0 (ToString)
                };

                let result = match mode {
                    0 => {
                        // Mode 0: To String
                        match val {
                            Value::Int(n) => Some(Value::Str(n.to_string())),
                            Value::Str(s) => Some(Value::Str(s)),
                            _ => Some(Value::Str(format!("{:?}", val))),
                        }
                    }
                    1 => {
                        // Mode 1: To Int
                        match val {
                            Value::Int(n) => Some(Value::Int(n)),
                            Value::Str(s) => {
                                if let Ok(n) = s.parse::<i64>() {
                                    Some(Value::Int(n))
                                } else if s.len() == 1 {
                                    // Char code
                                    Some(Value::Int(s.chars().next().unwrap() as i64))
                                } else {
                                    Some(Value::Int(0)) // Parse Error / Empty
                                }
                            }
                            _ => Some(Value::Int(0)),
                        }
                    }
                    2 => {
                        // Mode 2: Type ID
                        let type_id = match val {
                            Value::Int(_) => 0,
                            Value::Str(_) => 1,
                            Value::Junction(_, _) => 2,
                            Value::Superposition(_) => 3,
                            _ => -1, // Unknown
                        };
                        Some(Value::Int(type_id))
                    }
                    3 => {
                        // Mode 3: Length
                        match val {
                            Value::Str(s) => Some(Value::Int(s.len() as i64)),
                            Value::Junction(_, items) => Some(Value::Int(items.len() as i64)),
                            _ => Some(Value::Int(0)),
                        }
                    }
                    4 => {
                        // Mode 4: Liquefy (Str -> List of Char Codes)
                        match val {
                            Value::Str(s) => {
                                let chars: Vec<Value> =
                                    s.chars().map(|c| Value::Int(c as i64)).collect();
                                Some(Value::Junction(JunctionType::All, chars))
                            }
                            _ => None,
                        }
                    }
                    5 => {
                        // Mode 5: Solidify (List of Char Codes -> Str)
                        match val {
                            Value::Junction(_, items) => {
                                let mut s = String::new();
                                for item in items {
                                    if let Value::Int(n) = item {
                                        if let Some(c) = std::char::from_u32(n as u32) {
                                            s.push(c);
                                        }
                                    }
                                }
                                if s.len() > MAX_STRING_LEN {
                                    s.truncate(MAX_STRING_LEN);
                                }
                                Some(Value::Str(s))
                            }
                            _ => None,
                        }
                    }
                    _ => None, // Invalid Mode
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
            // Rune: Fuse (f)
            // Function: Combine two values.
            // Inputs: West (A) + East (B)
            // Operations:
            //   Str + Str -> Concat ("A" + "B" = "AB")
            //   Str + Int -> Repeat ("A" * 3 = "AAA")
            //   Int + Int -> Add (Arithmetic fallback)
            //   List + Val -> Push (Append to end)
            //   Val + List -> Unshift (Prepend to start)

            if let (Some(left), Some(right)) = (w_sig, e_sig) {
                let result = match (left, right) {
                    (Value::Str(s1), Value::Str(s2)) => {
                        if s1.len() + s2.len() > MAX_STRING_LEN {
                            Some(Value::Str(s1.clone())) // Or truncated? Let's just block growth.
                        } else {
                            Some(Value::Str(format!("{}{}", s1, s2)))
                        }
                    }
                    (Value::Str(s), Value::Int(n)) => {
                        let count = n.max(0) as usize;
                        if s.len().saturating_mul(count) > MAX_STRING_LEN {
                            Some(Value::Str(s.clone()))
                        } else {
                            Some(Value::Str(s.repeat(count)))
                        }
                    }
                    (Value::Int(n), Value::Str(s)) => {
                        let count = n.max(0) as usize;
                        if s.len().saturating_mul(count) > MAX_STRING_LEN {
                            Some(Value::Str(s.clone()))
                        } else {
                            Some(Value::Str(s.repeat(count)))
                        }
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
                    _ => None, // Incompatible types
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
            // Rune: Distill (d)
            // Function: Split a value into Head and Tail.
            // Input: West
            // Outputs:
            //   - North: Head (First char, Digit, or Element)
            //   - South: Tail (Remaining chars, Digits, or Elements)
            // Examples:
            //   "abc" -> "a" (N) + "bc" (S)
            //   123   -> 12 (N)  + 3 (S)  <-- Wait, logic below is N=Div, S=Mod
            // Logic Check:
            //   Int(n) >= 10: North = n/10, South = n%10.
            //   So 123 -> 12 (N), 3 (S). Correct.

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

                // Output North (Head)
                if let Some(h) = head {
                    if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                        if next_signals[ny][nx].is_none() {
                            next_signals[ny][nx] = Some(h);
                            changes = true;
                        }
                    }
                }
                // Output South (Tail)
                if let Some(t) = tail {
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        if next_signals[sy][sx].is_none() {
                            next_signals[sy][sx] = Some(t);
                            changes = true;
                        }
                    }
                }

                // Light up self to indicate activity
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

#[cfg(test)]
mod tests {
    use super::*;
    use crate::vm::GRID_SIZE;

    #[test]
    fn test_liquefy_and_solidify() {
        // Test Mode 4: Liquefy
        let mut signals = vec![vec![None; GRID_SIZE]; GRID_SIZE];
        let mut next = vec![vec![None; GRID_SIZE]; GRID_SIZE];

        // Setup:
        // (y,x) = (1,1) is Rune 't'
        // West (1,0) = "ABC"
        // North (0,1) = 4 (Liquefy)
        signals[1][0] = Some(Value::Str("ABC".to_string()));
        signals[0][1] = Some(Value::Int(4));

        apply_alchemy_runes("t", 1, 1, &signals, &mut next);

        let res = next[1][1].clone().expect("Should produce result");
        if let Value::Junction(JunctionType::All, items) = res {
            assert_eq!(items.len(), 3);
            assert_eq!(items[0], Value::Int(65)); // A
            assert_eq!(items[1], Value::Int(66)); // B
            assert_eq!(items[2], Value::Int(67)); // C
        } else {
            panic!("Expected Junction");
        }

        // Test Mode 5: Solidify
        // West (1,0) = List[65, 66, 67]
        // North (0,1) = 5 (Solidify)
        signals[1][0] = Some(Value::Junction(
            JunctionType::All,
            vec![Value::Int(65), Value::Int(66), Value::Int(67)],
        ));
        signals[0][1] = Some(Value::Int(5));
        next[1][1] = None; // Reset output

        apply_alchemy_runes("t", 1, 1, &signals, &mut next);

        let res = next[1][1].clone().expect("Should produce result");
        if let Value::Str(s) = res {
            assert_eq!(s, "ABC");
        } else {
            panic!("Expected String");
        }
    }
}
