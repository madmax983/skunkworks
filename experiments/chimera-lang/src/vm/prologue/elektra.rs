use super::normalize_coords;
use crate::vm::Value;
use std::collections::HashMap;

#[cfg(feature = "elektra")]
pub fn apply_elektra_runes(
    rune: &str,
    y: usize,
    x: usize,
    tick: u64,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    voltage_grid: &mut Vec<Vec<f32>>,
    resistance_grid: &mut Vec<Vec<f32>>,
    capacitance_grid: &mut Vec<Vec<f32>>,
    energy: &mut i64,
    registers: &mut HashMap<(usize, usize), Value>,
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "⚡" => {
            // Bolt: Source voltage. Reads West signal (Intensity) or defaults to 100V.
            let volts = if let Some(Value::Int(v)) = w_sig {
                v as f32
            } else {
                100.0
            };

            // Set resistance to Source (-1.0) and Voltage to v
            resistance_grid[y][x] = -1.0;
            voltage_grid[y][x] = volts;
        }
        "≡" => {
            // Ground: Sink voltage.
            // Always set resistance to Ground (-2.0)
            resistance_grid[y][x] = -2.0;
            voltage_grid[y][x] = 0.0;
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
        "🔌" => {
            // Bio-Generator: Consumes 1 Energy -> Sets 100V
            // We use registers to ensure we only consume energy once per tick.
            let last_active = if let Some(Value::Int(t)) = registers.get(&(y, x)) {
                *t as u64
            } else {
                u64::MAX
            };

            if last_active != tick {
                if *energy >= 1 {
                    *energy -= 1;
                    registers.insert((y, x), Value::Int(tick as i64));
                    voltage_grid[y][x] = 100.0;
                    resistance_grid[y][x] = -1.0; // Source
                } else {
                    // Not enough energy, acts as high resistance
                    resistance_grid[y][x] = 1000.0;
                }
            } else {
                // Already paid this tick
                voltage_grid[y][x] = 100.0;
                resistance_grid[y][x] = -1.0;
            }
        }
        "💡" => {
            // Bio-Light: Consumes Voltage -> Adds 5 Energy
            // Also need to limit to once per tick?
            // Yes, otherwise we generate infinite energy in the loop.
            let last_active = if let Some(Value::Int(t)) = registers.get(&(y, x)) {
                *t as u64
            } else {
                u64::MAX
            };

            if last_active != tick {
                let v = voltage_grid[y][x];
                if v > 50.0 {
                    *energy += 5;
                    registers.insert((y, x), Value::Int(tick as i64));
                }
            }
            resistance_grid[y][x] = 100.0; // Load
        }
        "🔋" => {
            // Capacitor: Set Capacitance to 100.0
            capacitance_grid[y][x] = 100.0;
        }
        "♒" => {
            // Memristor: Set Resistance to 50.0 (initial)
            // But if it's dynamic, we shouldn't overwrite it every tick if it drifts.
            // However, scan_grid happens every tick.
            // If we set it here, we reset it.
            // We should only set it if it's not already set? Or let update_circuit handle persistence.
            // update_circuit handles Memristor logic if cell string starts with R:.
            // But here the cell string is "♒".
            // So we need to map "♒" to Memristor behavior.
            // We can set resistance here, but we need registers to store state if we want persistence across ticks,
            // OR we rely on resistance_grid preserving value (which it does).
            // But if we write resistance_grid[y][x] = 50.0 here every tick, we clamp it.
            // Solution: Check if we initialized it this tick? No.
            // Check if value is default?

            // Actually, `prepare_signals` clears `signal_grid`, but `resistance_grid` persists in `vm`.
            // But `scan_grid_rules` runs every tick. `apply_elektra_runes` runs every tick.

            // If I write `resistance_grid[y][x] = 50.0`, I reset any training.
            // I should use `registers` to track if initialized.
            // Or just check if resistance is exactly default (1.0)? But what if it trained to 1.0?

            // Better approach: Use registers.
            let initialized = registers.contains_key(&(y, x));
            if !initialized {
                resistance_grid[y][x] = 50.0;
                registers.insert((y, x), Value::Int(1));
            }
            // If initialized, do nothing (let update_circuit modify resistance_grid).
        }
        "⇝" => {
            // Variable Resistor: Reads West signal.
            // Resistance = 100.0 - Signal. (Min 1.0)
            let sig = if let Some(Value::Int(v)) = w_sig {
                v.clamp(0, 99) as f32
            } else {
                0.0
            };
            resistance_grid[y][x] = (100.0 - sig).max(1.0);
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
    _tick: u64,
    _current_signals: &[Vec<Option<Value>>],
    _next_signals: &mut Vec<Vec<Option<Value>>>,
    _voltage_grid: &mut Vec<Vec<f32>>,
    _resistance_grid: &mut Vec<Vec<f32>>,
    _capacitance_grid: &mut Vec<Vec<f32>>,
    _energy: &mut i64,
    _registers: &mut HashMap<(usize, usize), Value>,
) -> bool {
    false
}
