use super::normalize_coords;
use crate::vm::Value;
use std::collections::HashMap;

pub struct ElektraArgs<'a> {
    pub rune: &'a str,
    pub y: usize,
    pub x: usize,
    pub tick: u64,
    pub current_signals: &'a [Vec<Option<Value>>],
    pub next_signals: &'a mut [Vec<Option<Value>>],
    pub voltage_grid: &'a mut [Vec<f32>],
    pub resistance_grid: &'a mut [Vec<f32>],
    pub capacitance_grid: &'a mut [Vec<f32>],
    pub energy: &'a mut i64,
    pub registers: &'a mut HashMap<(usize, usize), Value>,
}

#[cfg(feature = "elektra")]
pub fn apply_elektra_runes(args: ElektraArgs) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(args.y as i64, args.x as i64 - 1) {
        args.current_signals[wy][wx].clone()
    } else {
        None
    };

    match args.rune {
        "⚡" => {
            // Bolt: Source voltage. Reads West signal (Intensity) or defaults to 100V.
            let volts = if let Some(Value::Int(v)) = w_sig {
                v as f32
            } else {
                100.0
            };

            // Set resistance to Source (-1.0) and Voltage to v
            args.resistance_grid[args.y][args.x] = -1.0;
            args.voltage_grid[args.y][args.x] = volts;
        }
        "≡" => {
            // Ground: Sink voltage.
            // Always set resistance to Ground (-2.0)
            args.resistance_grid[args.y][args.x] = -2.0;
            args.voltage_grid[args.y][args.x] = 0.0;
        }
        "∿" => {
            // Sine: Sense Voltage.
            // Reads local voltage, outputs to South.
            let v = args.voltage_grid[args.y][args.x];
            if v.abs() > 0.1 {
                if let Some((sy, sx)) = normalize_coords(args.y as i64 + 1, args.x as i64) {
                    let new_val = Some(Value::Int(v as i64));
                    if args.next_signals[sy][sx] != new_val {
                        args.next_signals[sy][sx] = new_val;
                        changes = true;
                    }
                }
            }
        }
        "🔌" => {
            // Bio-Generator: Consumes 1 Energy -> Sets 100V
            // We use registers to ensure we only consume energy once per tick.
            let last_active = if let Some(Value::Int(t)) = args.registers.get(&(args.y, args.x)) {
                *t as u64
            } else {
                u64::MAX
            };

            if last_active != args.tick {
                if *args.energy >= 1 {
                    *args.energy -= 1;
                    args.registers
                        .insert((args.y, args.x), Value::Int(args.tick as i64));
                    args.voltage_grid[args.y][args.x] = 100.0;
                    args.resistance_grid[args.y][args.x] = -1.0; // Source
                } else {
                    // Not enough energy, acts as high resistance
                    args.resistance_grid[args.y][args.x] = 1000.0;
                }
            } else {
                // Already paid this tick
                args.voltage_grid[args.y][args.x] = 100.0;
                args.resistance_grid[args.y][args.x] = -1.0;
            }
        }
        "💡" => {
            // Bio-Light: Consumes Voltage -> Adds 5 Energy
            let last_active = if let Some(Value::Int(t)) = args.registers.get(&(args.y, args.x)) {
                *t as u64
            } else {
                u64::MAX
            };

            if last_active != args.tick {
                let v = args.voltage_grid[args.y][args.x];
                if v > 50.0 {
                    *args.energy += 5;
                    args.registers
                        .insert((args.y, args.x), Value::Int(args.tick as i64));
                }
            }
            args.resistance_grid[args.y][args.x] = 100.0; // Load
        }
        "🔋" => {
            // Capacitor: Set Capacitance to 100.0
            args.capacitance_grid[args.y][args.x] = 100.0;
        }
        "♒" => {
            // Memristor: Set Resistance to 50.0 (initial)
            let initialized = args.registers.contains_key(&(args.y, args.x));
            if !initialized {
                args.resistance_grid[args.y][args.x] = 50.0;
                args.registers.insert((args.y, args.x), Value::Int(1));
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
            args.resistance_grid[args.y][args.x] = (100.0 - sig).max(1.0);
        }
        "⏧" => {
            // Switch / Transistor:
            // Gate (North): Controls flow.
            // Source (West): Input Signal.
            // Drain (East): Output Signal.
            let n_sig = if let Some((ny, nx)) = normalize_coords(args.y as i64 - 1, args.x as i64) {
                args.current_signals[ny][nx].clone()
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
                    if let Some((ey, ex)) = normalize_coords(args.y as i64, args.x as i64 + 1) {
                        let new_val = Some(val);
                        if args.next_signals[ey][ex] != new_val {
                            args.next_signals[ey][ex] = new_val;
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
pub fn apply_elektra_runes(_args: ElektraArgs) -> bool {
    false
}
