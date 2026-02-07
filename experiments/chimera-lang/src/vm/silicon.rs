#![cfg(feature = "silicon")]
use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;

/// Executes Silicon OpCodes (Wireworld and Circuits).
pub fn exec_silicon_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) {
    match op {
        OpCode::Conduct => {
            step_circuit(vm);
            vm.output.push("SILICON: Conducted one step".to_string());
        }
        OpCode::Wire => {
            // stack: y, x (top)
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        vm.grid[ny][nx] = Value::Int(1); // Conductor
                    }
                } else {
                    vm.output.push("Error: Type mismatch for wire".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for wire".to_string());
            }
        }
        OpCode::Pulse => {
            // stack: y, x (top)
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        vm.grid[ny][nx] = Value::Int(2); // Electron Head
                    }
                } else {
                    vm.output.push("Error: Type mismatch for pulse".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for pulse".to_string());
            }
        }
        OpCode::Silicon => {
            vm.silicon_mode = !vm.silicon_mode;
            let status = if vm.silicon_mode { "ON" } else { "OFF" };
            vm.output
                .push(format!("SILICON: Auto-conduction {}", status));
        }
        OpCode::Construct => {
            // stack: type, dir, y, x (top)
            if vm.stack.len() >= 4 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let dir_val = vm.stack.pop().unwrap();
                let type_val = vm.stack.pop().unwrap();

                if let (Value::Int(t), Value::Int(d), Value::Int(y), Value::Int(x)) =
                    (type_val, dir_val, y_val, x_val)
                {
                    let type_str = match t {
                        0 => "AND",
                        1 => "OR",
                        2 => "XOR",
                        3 => "NAND",
                        4 => "NOT",
                        _ => "AND",
                    };
                    let dir_idx = d.rem_euclid(4);
                    let s = format!("G:{}:{}", type_str, dir_idx);
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        vm.grid[ny][nx] = Value::Str(s);
                        vm.output.push(format!(
                            "CONSTRUCT: {} gate facing {} at {},{}",
                            type_str, dir_idx, nx, ny
                        ));
                    } else {
                        vm.output
                            .push("Error: Coordinates out of bounds for construct".to_string());
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for construct".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for construct".to_string());
            }
        }
        OpCode::LogicGate => {
            // Manual placement logic if needed, or introspection
            // For now, no-op or placeholder
        }
        OpCode::PinIn => {
            // stack: y, x (top)
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        vm.grid[ny][nx] = Value::Str("PIN:IN".to_string());
                        vm.output.push(format!("PIN_IN: Created at {},{}", nx, ny));
                    }
                } else {
                    vm.output.push("Error: Type mismatch for pin_in".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for pin_in".to_string());
            }
        }
        OpCode::PinOut => {
            // stack: y, x (top)
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        vm.grid[ny][nx] = Value::Str("PIN:OUT".to_string());
                        vm.output.push(format!("PIN_OUT: Created at {},{}", nx, ny));
                    }
                } else {
                    vm.output.push("Error: Type mismatch for pin_out".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for pin_out".to_string());
            }
        }
        _ => {}
    }
}

/// Runs one step of the Circuit (Wireworld + Gates + Pins) on the grid.
pub fn step_circuit(vm: &mut ChimeraVM) {
    let rows = vm.grid.len();
    if rows == 0 {
        return;
    }
    let cols = vm.grid[0].len();
    let mut next_grid = vm.grid.clone();

    // Helper to get neighbor coords
    let get_neighbor =
        |vm: &ChimeraVM, y: usize, x: usize, dy: i64, dx: i64| -> Option<(usize, usize)> {
            vm.normalize_coords(y as i64 + dy, x as i64 + dx)
        };

    // Pass 1: Wireworld Automata
    for y in 0..rows {
        for x in 0..cols {
            let current_cell = &vm.grid[y][x];

            // Map values to Wireworld states
            let state = match current_cell {
                Value::Int(1) => 1, // Conductor
                Value::Int(2) => 2, // Head
                Value::Int(3) => 3, // Tail
                Value::Str(s) if s == "PIN:IN" => 4, // Input Pin (Acts as Conductor/Head)
                _ => 0,
            };

            if state == 0 {
                continue; // Skip
            }

            if state == 4 {
                // PIN:IN logic: Check stack. If peek > 0, become Head (fire).
                // But wait, the pin itself is a static component.
                // We shouldn't change the PIN:IN string to Int(2) because then it loses identity.
                // Instead, we should emit electrons to neighbors if active.
                // OR: PIN:IN *temporarily* becomes Head? No, that overwrites it.
                // Solution: PIN:IN remains PIN:IN. But neighbors see it as Head if active.
                continue;
            }

            let next_state = match state {
                2 => 3, // Head -> Tail
                3 => 1, // Tail -> Conductor
                1 => {
                    // Conductor -> Head if 1 or 2 heads nearby
                    let mut head_neighbors = 0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dy == 0 && dx == 0 {
                                continue;
                            }
                            if let Some((ny, nx)) =
                                vm.normalize_coords(y as i64 + dy, x as i64 + dx)
                            {
                                match &vm.grid[ny][nx] {
                                    Value::Int(2) => head_neighbors += 1,
                                    Value::Str(s) if s == "PIN:IN" => {
                                        // Check if PIN:IN is firing
                                        // It fires if stack has value > 0
                                        if let Some(Value::Int(val)) = vm.stack.last() {
                                            if *val > 0 {
                                                head_neighbors += 1;
                                            }
                                        }
                                    },
                                    _ => {}
                                }
                            }
                        }
                    }
                    if head_neighbors == 1 || head_neighbors == 2 {
                        2
                    } else {
                        1
                    }
                }
                _ => state,
            };

            if next_state != state {
                next_grid[y][x] = Value::Int(next_state);
            }
        }
    }

    // Pass 2: Gate Logic & Pin Output
    for y in 0..rows {
        for x in 0..cols {
            if let Value::Str(s) = &vm.grid[y][x] {
                // Pin Output Logic
                if s == "PIN:OUT" {
                    // Check neighbors for Electron Head (2)
                    let mut triggered = false;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dy == 0 && dx == 0 { continue; }
                            if let Some((ny, nx)) = get_neighbor(vm, y, x, dy, dx) {
                                if let Value::Int(2) = vm.grid[ny][nx] {
                                    triggered = true;
                                }
                            }
                        }
                    }
                    if triggered {
                        vm.stack.push(Value::Int(1));
                    }
                }
                // Logic Gate
                else if s.starts_with("G:") {
                    let parts: Vec<&str> = s.split(':').collect();
                    if parts.len() == 3 {
                        let gate_type = parts[1];
                        let dir_idx = parts[2].parse::<usize>().unwrap_or(0) % 4;
                        let (out_dy, out_dx) = match dir_idx {
                            0 => (-1, 0),
                            1 => (0, 1),
                            2 => (1, 0),
                            3 => (0, -1),
                            _ => (0, 0),
                        };

                        let mut active_inputs = 0;
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                if dy == 0 && dx == 0 {
                                    continue;
                                }
                                if dy == out_dy && dx == out_dx {
                                    continue;
                                }
                                if let Some((ny, nx)) = get_neighbor(vm, y, x, dy, dx) {
                                    if let Value::Int(2) = vm.grid[ny][nx] {
                                        active_inputs += 1;
                                    }
                                }
                            }
                        }

                        let output_high = match gate_type {
                            "AND" => active_inputs >= 2,
                            "OR" => active_inputs >= 1,
                            "XOR" => active_inputs % 2 == 1,
                            "NAND" => active_inputs < 2,
                            "NOT" => active_inputs == 0,
                            _ => false,
                        };

                        if output_high {
                            if let Some((ny, nx)) = get_neighbor(vm, y, x, out_dy, out_dx) {
                                if let Value::Int(1) = vm.grid[ny][nx] {
                                    next_grid[ny][nx] = Value::Int(2);
                                }
                            }
                        }
                    }
                }
            }
        }
    }

    vm.grid = next_grid;
}
