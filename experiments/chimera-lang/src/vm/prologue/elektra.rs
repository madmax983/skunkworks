use super::normalize_coords;
use crate::vm::Value;

#[cfg(feature = "elektra")]
pub fn apply_elektra_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    voltage_grid: &mut Vec<Vec<f32>>,
    resistance_grid: &mut Vec<Vec<f32>>,
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "⚡" => {
            // Bolt: Source voltage. Reads West signal (Intensity).
            if let Some(Value::Int(v)) = w_sig {
                if v > 0 {
                    // Set resistance to Source (-1.0) and Voltage to v
                    resistance_grid[y][x] = -1.0;
                    voltage_grid[y][x] = v as f32;
                    // No output signal, effect is on voltage grid
                } else {
                    // Turn off source
                    if resistance_grid[y][x] == -1.0 {
                        resistance_grid[y][x] = 1.0; // Reset to default air?
                    }
                }
            }
        }
        "≡" => {
            // Ground: Sink voltage.
            // Always set resistance to Ground (-2.0)
            resistance_grid[y][x] = -2.0;
            voltage_grid[y][x] = 0.0;

            // If there is voltage nearby, emit signal South?
            // "Reads Voltage at Self"
            // Since we ground it, V at self is 0. But we can read current?
            // Or maybe it reads voltage BEFORE grounding?
            // The voltage grid simulation step happens separately.

            // Let's say Ground rune just establishes the Ground node.
            // Use ∿ for sensing.
        }
        "∿" => {
            // Sine: Sense Voltage.
            // Reads local voltage, outputs to South.
            let v = voltage_grid[y][x];
            if v.abs() > 0.1 {
                if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                    if next_signals[sy][sx].is_none() {
                        next_signals[sy][sx] = Some(Value::Int(v as i64));
                        changes = true;
                    }
                }
            }
        }
        _ => {}
    }
    changes
}

#[cfg(not(feature = "elektra"))]
pub fn apply_elektra_runes(
    _rune: &str,
    _y: usize,
    _x: usize,
    _current_signals: &[Vec<Option<Value>>],
    _next_signals: &mut Vec<Vec<Option<Value>>>,
    _voltage_grid: &mut Vec<Vec<f32>>,
    _resistance_grid: &mut Vec<Vec<f32>>,
) -> bool {
    false
}
