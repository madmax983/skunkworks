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
        "*" => {
            // Spectral Splitters (Side Effects)
            // Check for Spectral Context (Color)
            if let Some((r, g, b)) = vm.chroma_grid[y][x].fg {
                // Determine dominant channel
                let max = r.max(g).max(b);
                if max > 0 {
                    let is_red = r == max;
                    let is_green = g == max && !is_red;

                    if is_green {
                        // Green Spore: Spawn Agent based on input (West)
                        if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                            if let Some(sig) = &vm.prologue_state.signal_grid[wy][wx] {
                                // Spawn to South
                                if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                                    let agent_type = match sig {
                                        Value::Int(2) => "K", // Chaos
                                        Value::Int(3) => "H", // Hunter
                                        _ => "@",             // Seeker
                                    };

                                    // Only overwrite if empty or weak
                                    let can_spawn = match &vm.grid[sy][sx] {
                                        Value::Int(0) => true,
                                        _ => false,
                                    };

                                    if can_spawn {
                                        vm.grid[sy][sx] = Value::Str(agent_type.to_string());
                                        // Light up self
                                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                                    }
                                }
                            }
                        }
                    }
                }
            }
        }
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
    let w_sig = get_sig(current_signals, y, x, 0, -1);
    let e_sig = get_sig(current_signals, y, x, 0, 1);

    match rune {
        "+" => {
            // Red Add: (West + East) * 2
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (w_sig, e_sig) {
                let res = (w + e) * 2;
                set_sig(next_signals, y, x, 1, 0, Value::Int(res));
            }
            true
        }
        "-" => {
            // Red Sub: Clamped Sub (Max(0, W-E))
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (w_sig, e_sig) {
                let res = (w - e).max(0);
                set_sig(next_signals, y, x, 1, 0, Value::Int(res));
            }
            true
        }
        "*" => {
            // Red Split: West -> N, E, S (Explosive)
            if let Some(val) = w_sig {
                set_sig(next_signals, y, x, -1, 0, val.clone());
                set_sig(next_signals, y, x, 0, 1, val.clone());
                set_sig(next_signals, y, x, 1, 0, val);
            }
            true
        }
        "/" => {
            // Red Div: Fragmentation (DivRem)
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (w_sig, e_sig) {
                if e != 0 {
                    let div = w / e;
                    let rem = w % e;
                    set_sig(next_signals, y, x, -1, 0, Value::Int(div)); // North
                    set_sig(next_signals, y, x, 1, 0, Value::Int(rem));  // South
                }
            }
            true
        }
        "%" => {
            // Red Mod: Atomize (1 if A%B==0 else 0)
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (w_sig, e_sig) {
                if e != 0 {
                    let res = if w % e == 0 { 1 } else { 0 };
                    set_sig(next_signals, y, x, 1, 0, Value::Int(res));
                }
            }
            true
        }
        "^" => {
            // Red XOR: Annihilation (Sum if A != B, else 0)
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (w_sig, e_sig) {
                let res = if w != e { w + e } else { 0 };
                set_sig(next_signals, y, x, 1, 0, Value::Int(res));
            }
            true
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
    let w_sig = get_sig(current_signals, y, x, 0, -1);
    let e_sig = get_sig(current_signals, y, x, 0, 1);

    match rune {
        "+" => {
            // Green Add: Crossover (String Splicing)
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
            }
            true
        }
        "-" => {
            // Green Sub: Pruning (Remove chars of E from W)
             if let (Some(Value::Str(w)), Some(Value::Str(e))) = (w_sig, e_sig) {
                 let res: String = w.chars().filter(|c| !e.contains(*c)).collect();
                 set_sig(next_signals, y, x, 1, 0, Value::Str(res));
             }
             true
        }
        "*" => {
            // Green Split: Spore (Clone to North and South? Or just Split value)
             if let Some(val) = w_sig {
                set_sig(next_signals, y, x, 0, 1, val);
             }
             true
        }
        "/" => {
            // Green Div: Mitosis (Split W into two W/2 signals)
            if let Some(Value::Int(w)) = w_sig {
                let half = w / 2;
                set_sig(next_signals, y, x, -1, 0, Value::Int(half)); // North
                set_sig(next_signals, y, x, 1, 0, Value::Int(half));  // South
            }
            true
        }
        "%" => {
            // Green Mod: Mutation (XOR with E or Mask)
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (w_sig, e_sig) {
                let res = w ^ e;
                set_sig(next_signals, y, x, 1, 0, Value::Int(res));
            }
            true
        }
        "&" => {
            // Green AND: Symbiosis (Max(A, B))
             if let (Some(Value::Int(w)), Some(Value::Int(e))) = (w_sig, e_sig) {
                let res = w.max(e);
                set_sig(next_signals, y, x, 1, 0, Value::Int(res));
            }
            true
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
    let w_sig = get_sig(current_signals, y, x, 0, -1);
    let e_sig = get_sig(current_signals, y, x, 0, 1);

    match rune {
        "+" => {
            // Blue Add: Logical AND (Bitwise or Boolean)
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (w_sig, e_sig) {
                // If both are non-zero, output 1. Else 0.
                let res = if w != 0 && e != 0 { 1 } else { 0 };
                set_sig(next_signals, y, x, 1, 0, Value::Int(res));
            }
            true
        }
        "-" => {
            // Blue Sub: Inverse (E - W)
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (w_sig, e_sig) {
                let res = e - w;
                set_sig(next_signals, y, x, 1, 0, Value::Int(res));
            }
            true
        }
        "*" => {
            // Blue Split: Delay (Time Dilation)
             if let Some(Value::Int(w)) = w_sig {
                 let res = if w == 0 { 1 } else { 0 };
                 set_sig(next_signals, y, x, 0, 1, Value::Int(res));
             }
             true
        }
        "/" => {
            // Blue Div: Filter (Pass W if W % E == 0)
            if let (Some(Value::Int(w)), Some(Value::Int(e))) = (w_sig, e_sig) {
                if e != 0 && w % e == 0 {
                     set_sig(next_signals, y, x, 1, 0, Value::Int(w));
                }
            }
            true
        }
        "%" => {
            // Blue Mod: Clock (W % 12)
            if let Some(Value::Int(w)) = w_sig {
                let res = w % 12;
                set_sig(next_signals, y, x, 1, 0, Value::Int(res));
            }
            true
        }
        "|" => {
            // Blue OR: NAND
             if let (Some(Value::Int(w)), Some(Value::Int(e))) = (w_sig, e_sig) {
                let res = if w != 0 && e != 0 { 0 } else { 1 };
                set_sig(next_signals, y, x, 1, 0, Value::Int(res));
            }
            true
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
