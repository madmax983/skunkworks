use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value, GRID_SIZE, MAX_PLANES};

pub fn exec_planes_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) {
    match op {
        OpCode::Dimension => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(target_id) = val {
                    if target_id == vm.current_plane {
                        // No op
                        return;
                    }

                    // Check if we can allocate a new plane if target doesn't exist
                    if !vm.planes.contains_key(&target_id) && vm.planes.len() >= MAX_PLANES {
                        vm.output.push("Error: Plane limit exceeded".to_string());
                        return;
                    }

                    // Save current grid
                    let current_grid = std::mem::replace(
                        &mut vm.grid,
                        vec![vec![Value::Int(0); GRID_SIZE]; GRID_SIZE],
                    );
                    vm.planes.insert(vm.current_plane, current_grid);

                    // Load target grid
                    if let Some(target_grid) = vm.planes.remove(&target_id) {
                        vm.grid = target_grid;
                    } else {
                        // Create new grid (already default 0s from replace, so we just keep it)
                        vm.output
                            .push(format!("DIMENSION: Created new plane {}", target_id));
                    }

                    vm.output
                        .push(format!("DIMENSION: Shifted to plane {}", target_id));
                    vm.current_plane = target_id;
                } else {
                    vm.output
                        .push("Error: Dimension requires Int ID".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for Dimension".to_string());
            }
        }
        OpCode::DView => {
            vm.stack.push(Value::Int(vm.current_plane));
        }
        OpCode::DRead => {
            // Stack: y, x, plane_id
            if vm.stack.len() >= 3 {
                let plane_val = vm.stack.pop().unwrap();
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();

                if let (Value::Int(y), Value::Int(x), Value::Int(id)) = (y_val, x_val, plane_val) {
                    if id == vm.current_plane {
                        // Read from current
                        if vm.is_valid_coord(y, x) {
                            vm.stack.push(vm.grid[y as usize][x as usize].clone());
                        } else {
                            vm.stack.push(Value::Int(0));
                        }
                    } else {
                        // Read from stored
                        if let Some(plane) = vm.planes.get(&id) {
                            if vm.is_valid_coord(y, x) {
                                vm.stack.push(plane[y as usize][x as usize].clone());
                            } else {
                                vm.stack.push(Value::Int(0));
                            }
                        } else {
                            // Plane empty/doesn't exist -> 0
                            vm.stack.push(Value::Int(0));
                        }
                    }
                } else {
                    vm.output.push("Error: Type mismatch for DRead".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for DRead".to_string());
            }
        }
        OpCode::DWrite => {
            // Stack: val, y, x, plane_id
            if vm.stack.len() >= 4 {
                let plane_val = vm.stack.pop().unwrap();
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let val = vm.stack.pop().unwrap();

                if let (Value::Int(y), Value::Int(x), Value::Int(id)) = (y_val, x_val, plane_val) {
                    if !vm.is_valid_coord(y, x) {
                        return;
                    }

                    if id == vm.current_plane {
                        vm.grid[y as usize][x as usize] = val;
                    } else {
                        // Access stored plane or create it
                        let has_plane = vm.planes.contains_key(&id);
                        if has_plane {
                            if let Some(plane) = vm.planes.get_mut(&id) {
                                plane[y as usize][x as usize] = val;
                            }
                        } else if vm.planes.len() >= MAX_PLANES {
                            vm.output.push("Error: Plane limit exceeded".to_string());
                        } else {
                            let plane =
                                vm.planes
                                    .entry(id)
                                    .or_insert(vec![vec![Value::Int(0); GRID_SIZE]; GRID_SIZE]);
                            plane[y as usize][x as usize] = val;
                        }
                    }
                } else {
                    vm.output
                        .push("Error: Type mismatch for DWrite".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for DWrite".to_string());
            }
        }
        OpCode::DMerge => {
            // Stack: method, plane_id
            if vm.stack.len() >= 2 {
                let id_val = vm.stack.pop().unwrap();
                let method_val = vm.stack.pop().unwrap();

                if let (Value::Int(method), Value::Int(id)) = (method_val, id_val) {
                    if id == vm.current_plane {
                        // Merge self? No-op.
                        return;
                    }

                    // Clone the target plane to iterate over it while mutating self.grid
                    let target_plane = if let Some(p) = vm.planes.get(&id) {
                        p.clone()
                    } else {
                        return;
                    };

                    for y in 0..GRID_SIZE {
                        for x in 0..GRID_SIZE {
                            let src = &target_plane[y][x];
                            let dst = &mut vm.grid[y][x];

                            match method {
                                0 => {
                                    // Add
                                    if let Value::Int(b_val) = src {
                                        if let Value::Int(ref mut a_val) = dst {
                                            *a_val = (*a_val).wrapping_add(*b_val);
                                        }
                                    }
                                }
                                1 => {
                                    // Max
                                    if let Value::Int(b_val) = src {
                                        if let Value::Int(ref mut a_val) = dst {
                                            *a_val = (*a_val).max(*b_val);
                                        }
                                    }
                                }
                                2 => {
                                    // Overwrite if not 0
                                    if !matches!(src, Value::Int(0)) {
                                        *dst = src.clone();
                                    }
                                }
                                _ => {}
                            }
                        }
                    }
                    vm.output
                        .push(format!("DMERGE: Merged plane {} (method {})", id, method));
                } else {
                    vm.output
                        .push("Error: Type mismatch for DMerge".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for DMerge".to_string());
            }
        }
        _ => {}
    }
}
