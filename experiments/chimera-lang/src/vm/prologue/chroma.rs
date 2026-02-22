use super::normalize_coords;
use crate::vm::Value;

/// Applies signal propagation rules for Chromatics.
///
/// Handles `🎨` (Palette), `🔴`/`🟢`/`🔵` (Extractors), and `👁` (Eye).
///
/// Also intercepts Standard Runes (`+`, `*`, `?`) if they are colored,
/// applying Spectral Logic (Prism Language).
pub fn apply_chroma_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    chroma_grid: &[Vec<crate::vm::ChromaCell>],
) -> bool {
    let mut changes = false;

    // Helper to read signal from a neighbor
    let read_sig = |dy: i64, dx: i64| -> Option<Value> {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            current_signals[ny][nx].clone()
        } else {
            None
        }
    };

    // Check for Spectral Context (Color)
    if let Some((r, g, b)) = chroma_grid[y][x].fg {
        // Determine dominant channel
        // Simple heuristic: Max value is dominant.
        let max = r.max(g).max(b);
        if max > 0 {
            let is_red = r == max;
            let is_green = g == max && !is_red;
            let is_blue = b == max && !is_red && !is_green;

            if is_red {
                if apply_red_logic(rune, y, x, current_signals, next_signals) {
                    return true;
                }
            } else if is_green {
                if apply_green_logic(rune, y, x, current_signals, next_signals) {
                    return true;
                }
            } else if is_blue {
                if apply_blue_logic(rune, y, x, current_signals, next_signals) {
                    return true;
                }
            }
        }
    }

    match rune {
        "🎨" => {
            // Palette: Mix inputs into Color
            // West: Red, North: Green, East: Blue

            let r_val = read_sig(0, -1);
            let g_val = read_sig(-1, 0);
            let b_val = read_sig(0, 1);

            let r = extract_channel(r_val, 0); // Red channel
            let g = extract_channel(g_val, 1); // Green channel
            let b = extract_channel(b_val, 2); // Blue channel

            // Emit Color to Self
            let new_val = Value::Color(r, g, b);
            if next_signals[y][x] != Some(new_val.clone()) {
                next_signals[y][x] = Some(new_val);
                changes = true;
            }
        }
        "🔴" => {
            // Extract Red from West
            if let Some(val) = read_sig(0, -1) {
                let r = extract_channel(Some(val), 0);
                let out = Value::Int(r as i64);
                // Output to North
                if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                    if next_signals[ny][nx] != Some(out.clone()) {
                        next_signals[ny][nx] = Some(out);
                        changes = true;
                    }
                }
            }
        }
        "🟢" => {
            // Extract Green from West
            if let Some(val) = read_sig(0, -1) {
                let g = extract_channel(Some(val), 1);
                let out = Value::Int(g as i64);
                if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                    if next_signals[ny][nx] != Some(out.clone()) {
                        next_signals[ny][nx] = Some(out);
                        changes = true;
                    }
                }
            }
        }
        "🔵" => {
            // Extract Blue from West
            if let Some(val) = read_sig(0, -1) {
                let b = extract_channel(Some(val), 2);
                let out = Value::Int(b as i64);
                if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                    if next_signals[ny][nx] != Some(out.clone()) {
                        next_signals[ny][nx] = Some(out);
                        changes = true;
                    }
                }
            }
        }
        "👁" => {
            // Eye: Reads Chroma at Self.
            let cell = chroma_grid[y][x];
            if let Some((r, g, b)) = cell.fg {
                let val = Value::Color(r, g, b);
                if next_signals[y][x] != Some(val.clone()) {
                    next_signals[y][x] = Some(val);
                    changes = true;
                }
            }
        }
        _ => {}
    }

    changes
}

/// Applies Sink effects for Chromatics.
///
/// Handles `🖌` (Brush).
pub fn apply_chroma_sinks(vm: &mut crate::vm::ChimeraVM, rune: &str, y: usize, x: usize) {
    match rune {
        "🖌" => {
            // Brush: Reads West (Color), Paints South Grid Cell
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(sig) = &vm.prologue_state.signal_grid[wy][wx] {
                    if let Value::Color(r, g, b) = sig {
                        if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                            vm.chroma_grid[sy][sx].fg = Some((*r, *g, *b));
                            // Light up self
                            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                        }
                    } else if let Value::Int(n) = sig {
                        // Grayscale paint?
                        if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                            let gray = (*n).clamp(0, 255) as u8;
                            vm.chroma_grid[sy][sx].fg = Some((gray, gray, gray));
                            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                        }
                    }
                }
            }
        }
        _ => {}
    }
}

/// Helper to extract a channel (0=R, 1=G, 2=B) from a Value.
fn extract_channel(val: Option<Value>, channel: usize) -> u8 {
    match val {
        Some(Value::Color(r, g, b)) => match channel {
            0 => r,
            1 => g,
            2 => b,
            _ => 0,
        },
        Some(Value::Int(n)) => n.clamp(0, 255) as u8,
        _ => 0,
    }
}

// --- Spectral Logic Handlers ---

fn apply_red_logic(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    // Red: Amplification, Force, Destruction
    match rune {
        "+" => {
            // Red Add: (West + East) * 2
            let w_sig = get_sig(current_signals, y, x, 0, -1);
            let e_sig = get_sig(current_signals, y, x, 0, 1);
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (w_sig, e_sig) {
                let res = (w + e) * 2;
                set_sig(next_signals, y, x, 1, 0, Value::Int(res));
                true
            } else {
                false
            }
        }
        "*" => {
            // Red Split: West -> N, E, S (Explosive)
            if let Some(val) = get_sig(current_signals, y, x, 0, -1) {
                set_sig(next_signals, y, x, -1, 0, val.clone());
                set_sig(next_signals, y, x, 0, 1, val.clone());
                set_sig(next_signals, y, x, 1, 0, val);
                true
            } else {
                false
            }
        }
        _ => false,
    }
}

fn apply_green_logic(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    // Green: Life, Growth, Crossover
    match rune {
        "+" => {
            // Green Add: Crossover (String Splicing)
            let w_sig = get_sig(current_signals, y, x, 0, -1);
            let e_sig = get_sig(current_signals, y, x, 0, 1);
            if let (Some(Value::Str(w)), Some(Value::Str(e))) = (w_sig, e_sig) {
                // Unicode-safe splicing
                let w_chars: Vec<char> = w.chars().collect();
                let e_chars: Vec<char> = e.chars().collect();
                let split_w = w_chars.len() / 2;
                let split_e = e_chars.len() / 2;

                let head: String = w_chars.into_iter().take(split_w).collect();
                let tail: String = e_chars.into_iter().skip(split_e).collect();
                let new_s = format!("{}{}", head, tail);

                set_sig(next_signals, y, x, 1, 0, Value::Str(new_s));
                true
            } else {
                false
            }
        }
        "*" => {
            // Green Split: Spore (Spawn Agent?)
             if let Some(val) = get_sig(current_signals, y, x, 0, -1) {
                set_sig(next_signals, y, x, 0, 1, val);
                true
             } else {
                 false
             }
        }
        _ => false,
    }
}

fn apply_blue_logic(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    // Blue: Logic, Time, Control
    match rune {
        "+" => {
            // Blue Add: Logical AND (Bitwise or Boolean)
            let w_sig = get_sig(current_signals, y, x, 0, -1);
            let e_sig = get_sig(current_signals, y, x, 0, 1);
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (w_sig, e_sig) {
                // If both are non-zero, output 1. Else 0.
                let res = if w != 0 && e != 0 { 1 } else { 0 };
                set_sig(next_signals, y, x, 1, 0, Value::Int(res));
                true
            } else {
                false
            }
        }
        "*" => {
            // Blue Split: Delay (Time Dilation)
             if let Some(Value::Int(w)) = get_sig(current_signals, y, x, 0, -1) {
                 let res = if w == 0 { 1 } else { 0 };
                 set_sig(next_signals, y, x, 0, 1, Value::Int(res));
                 true
             } else {
                 false
             }
        }
        _ => false,
    }
}

// Helpers

fn get_sig(
    signals: &[Vec<Option<Value>>],
    y: usize,
    x: usize,
    dy: i64,
    dx: i64,
) -> Option<Value> {
    if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
        signals[ny][nx].clone()
    } else {
        None
    }
}

fn set_sig(
    signals: &mut Vec<Vec<Option<Value>>>,
    y: usize,
    x: usize,
    dy: i64,
    dx: i64,
    val: Value,
) -> bool {
    if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
        if signals[ny][nx] != Some(val.clone()) {
            signals[ny][nx] = Some(val);
            return true;
        }
    }
    false
}
