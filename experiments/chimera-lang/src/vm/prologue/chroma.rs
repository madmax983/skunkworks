use super::normalize_coords;
use crate::vm::Value;

/// Applies signal propagation rules for Chromatics.
///
/// Handles `🎨` (Palette), `🔴`/`🟢`/`🔵` (Extractors), and `👁` (Eye).
pub fn apply_chroma_runes(
    rune: &str,
    y: usize,
    x: usize,
    _current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    chroma_grid: &[Vec<crate::vm::ChromaCell>],
) -> bool {
    let mut changes = false;

    // Helper to read signal from a neighbor
    let read_sig = |dy: i64, dx: i64| -> Option<Value> {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            _current_signals[ny][nx].clone()
        } else {
            None
        }
    };

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
