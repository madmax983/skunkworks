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
    next_signals: &mut [Vec<Option<Value>>],
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
                    let new_val = Some(Value::Int(v as i64));
                    if next_signals[sy][sx] != new_val {
                        next_signals[sy][sx] = new_val;
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
            let initialized = registers.contains_key(&(y, x));
            if !initialized {
                resistance_grid[y][x] = 50.0;
                registers.insert((y, x), Value::Int(1));
            }
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
        "⏧" => {
            // Switch / Transistor:
            // Gate (North): Controls flow.
            // Source (West): Input Signal.
            // Drain (East): Output Signal.
            let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                current_signals[ny][nx].clone()
            } else {
                None
            };

            let gate_open = match n_sig {
                Some(Value::Int(n)) => n != 0,
                Some(Value::Str(s)) => !s.is_empty(),
                _ => false,
            };

            if gate_open {
                // Pass West to East
                if let Some(val) = w_sig {
                    if let Some((ey, ex)) = normalize_coords(y as i64, x as i64 + 1) {
                        let new_val = Some(val);
                        if next_signals[ey][ex] != new_val {
                            next_signals[ey][ex] = new_val;
                            changes = true;
                        }
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
    _tick: u64,
    _current_signals: &[Vec<Option<Value>>],
    _next_signals: &mut [Vec<Option<Value>>],
    _voltage_grid: &mut Vec<Vec<f32>>,
    _resistance_grid: &mut Vec<Vec<f32>>,
    _capacitance_grid: &mut Vec<Vec<f32>>,
    _energy: &mut i64,
    _registers: &mut HashMap<(usize, usize), Value>,
) -> bool {
    false
}
