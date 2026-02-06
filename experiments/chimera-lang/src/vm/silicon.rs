#![cfg(feature = "silicon")]
use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;

/// Executes Silicon OpCodes (Wireworld and Circuits).
pub fn exec_silicon_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) {
    match op {
        OpCode::Conduct => {
            step_wireworld(vm);
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
        _ => {}
    }
}

/// Runs one step of the Wireworld Cellular Automaton on the grid.
///
/// **States:**
/// - `0`: Empty
/// - `1`: Conductor
/// - `2`: Electron Head
/// - `3`: Electron Tail
///
/// **Rules:**
/// - Empty -> Empty
/// - Head -> Tail
/// - Tail -> Conductor
/// - Conductor -> Head if 1 or 2 neighbors are Head.
pub fn step_wireworld(vm: &mut ChimeraVM) {
    let rows = vm.grid.len();
    if rows == 0 {
        return;
    }
    let cols = vm.grid[0].len();
    let mut next_grid = vm.grid.clone();

    for y in 0..rows {
        for x in 0..cols {
            let current_cell = &vm.grid[y][x];

            // Map values to Wireworld states (preserving other values as Empty/Static)
            let state = match current_cell {
                Value::Int(1) => 1, // Conductor
                Value::Int(2) => 2, // Head
                Value::Int(3) => 3, // Tail
                _ => 0, // Empty or other (treated as Empty for evolution, but we preserve them if not part of wireworld)
            };

            if state == 0 {
                continue; // Skip non-wireworld cells
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
                            // Using normalize_coords handles topology (Torus, etc)
                            if let Some((ny, nx)) =
                                vm.normalize_coords(y as i64 + dy, x as i64 + dx)
                            {
                                if let Value::Int(2) = vm.grid[ny][nx] {
                                    head_neighbors += 1;
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

    vm.grid = next_grid;
}
