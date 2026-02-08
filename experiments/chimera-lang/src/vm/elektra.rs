#![cfg(feature = "elektra")]
use super::{ChimeraVM, Value, GRID_SIZE};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use std::collections::{HashSet, VecDeque};

pub fn exec_elektra_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) {
    match op {
        OpCode::Battery => {
            if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let v_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x), Value::Int(v)) = (y_val, x_val, v_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        vm.grid[ny][nx] = Value::Str(format!("BAT:{}", v));
                        vm.output.push(format!("BATTERY: {}V at {},{}", v, nx, ny));
                    }
                } else {
                    vm.output.push("Error: Type mismatch for battery".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for battery".to_string());
            }
        }
        OpCode::Ground => {
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x)) = (y_val, x_val) {
                    if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                        vm.grid[ny][nx] = Value::Str("GND".to_string());
                        vm.output.push(format!("GROUND at {},{}", nx, ny));
                    }
                } else {
                    vm.output.push("Error: Type mismatch for ground".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for ground".to_string());
            }
        }
        OpCode::SenseVolt => {
            let (y, x) = vm.context_loc;
            let v = vm.voltage_grid[y][x];
            vm.stack.push(Value::Int(v as i64));
        }
        OpCode::Shock => {
             if let Some(Value::Int(dmg)) = vm.stack.pop() {
                let (cy, cx) = vm.context_loc;
                let current = vm.current_grid[cy][cx];
                if current > 0.0 {
                    vm.output.push(format!("SHOCK: Discharged {} damage!", dmg));
                    // Logic to damage neighbors could go here
                } else {
                    vm.output.push("SHOCK: Fizzled (No Current)".to_string());
                }
             } else {
                 vm.output.push("Error: Stack underflow for shock".to_string());
             }
        }
        OpCode::Lightning => {
             if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let p_val = vm.stack.pop().unwrap();
                if let (Value::Int(y), Value::Int(x), Value::Int(p)) = (y_val, x_val, p_val) {
                     vm.output.push(format!("LIGHTNING: Strike at {},{} Power {}", x, y, p));
                     if let Some((ny, nx)) = vm.normalize_coords(y, x) {
                         vm.grid[ny][nx] = Value::Int(0); // Vaporize
                     }
                } else {
                    vm.output.push("Error: Type mismatch for lightning".to_string());
                }
             } else {
                 vm.output.push("Error: Stack underflow for lightning".to_string());
             }
        }
        _ => {}
    }
}

pub fn update_circuitry(vm: &mut ChimeraVM) {
    // Reset grids
    for row in vm.voltage_grid.iter_mut() {
        for cell in row.iter_mut() {
            *cell = 0.0;
        }
    }
    for row in vm.current_grid.iter_mut() {
        for cell in row.iter_mut() {
            *cell = 0.0;
        }
    }

    let mut visited = HashSet::new();

    for y in 0..GRID_SIZE {
        for x in 0..GRID_SIZE {
            if visited.contains(&(y, x)) {
                continue;
            }

            if is_conductive(vm, y, x) {
                // BFS to find Net
                let mut net = Vec::new();
                let mut queue = VecDeque::new();
                queue.push_back((y, x));
                visited.insert((y, x));

                let mut max_voltage: f32 = 0.0;
                let mut has_ground = false;

                while let Some((cy, cx)) = queue.pop_front() {
                    net.push((cy, cx));

                    // Check Source/Sink properties
                    match &vm.grid[cy][cx] {
                        Value::Str(s) if s.starts_with("BAT:") => {
                            if let Ok(v) = s.trim_start_matches("BAT:").parse::<f32>() {
                                max_voltage = max_voltage.max(v);
                            }
                        }
                        Value::Str(s) if s == "GND" => {
                            has_ground = true;
                        }
                        _ => {}
                    }

                    // Neighbors
                    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                    for (dy, dx) in neighbors {
                        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                            if !visited.contains(&(ny, nx)) && is_conductive(vm, ny, nx) {
                                visited.insert((ny, nx));
                                queue.push_back((ny, nx));
                            }
                        }
                    }
                }

                // Apply State
                if max_voltage > 0.0 {
                    for (ny, nx) in &net {
                        vm.voltage_grid[*ny][*nx] = max_voltage;
                        if has_ground {
                            vm.current_grid[*ny][*nx] = max_voltage;
                        }
                    }
                }
            }
        }
    }
}

fn is_conductive(vm: &ChimeraVM, y: usize, x: usize) -> bool {
    match &vm.grid[y][x] {
        Value::Int(1) => true, // Wire
        Value::Str(s) => {
            s.starts_with("BAT:") ||
            s == "GND" ||
            s == "WR" ||
            s == "PIN:IN" ||
            s == "PIN:OUT" ||
            s == "+" || s == "-" || s == "|" ||
            s == "iron" || s == "copper" || s == "water"
        }
        _ => false
    }
}
