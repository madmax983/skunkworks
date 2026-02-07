use crate::ast::{JunctionType, Nucleotide};
use crate::opcode::OpCode;
use crate::vm::{ChimeraVM, Value};
use rand::Rng;

pub fn exec_crystal_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Nucleate => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(type_id) = val {
                    let (cy, cx) = vm.context_loc;
                    let crystal = Value::Junction(
                        JunctionType::All,
                        vec![Value::Str("Crystal".to_string()), Value::Int(type_id)],
                    );
                    vm.grid[cy][cx] = crystal;
                    vm.output.push(format!(
                        "CRYSTAL: Nucleated type {} at {},{}",
                        type_id, cx, cy
                    ));
                } else {
                    vm.output
                        .push("Error: Invalid type for Nucleate".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for Nucleate".to_string());
            }
        }
        OpCode::Accrete => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(rate) = val {
                    let mut new_grid = vm.grid.clone();
                    let mut growth_count = 0;

                    for y in 0..16 {
                        for x in 0..16 {
                            if let Value::Junction(_, vals) = &vm.grid[y][x] {
                                if vals.len() >= 2 && vals[0] == Value::Str("Crystal".to_string()) {
                                    // It's a crystal. Grow to neighbors.
                                    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                                    for (dy, dx) in neighbors {
                                        // Check bounds
                                        let ny = y as i64 + dy;
                                        let nx = x as i64 + dx;
                                        if let Some((ny, nx)) = vm.normalize_coords(ny, nx) {
                                            // If empty (0) or simple int, take over
                                            if matches!(vm.grid[ny][nx], Value::Int(_)) {
                                                let mut rng = rand::thread_rng();
                                                if rng.gen_range(0..100) < rate {
                                                    // Don't overwrite if we already grew into it in this step
                                                    // But logic here copies from OLD grid, so conflicts are resolved by "last write wins" or explicit check.
                                                    // Since we write to new_grid, we should check if new_grid already has a crystal there to avoid overwriting.
                                                    // But simplistic is fine.
                                                    new_grid[ny][nx] = vm.grid[y][x].clone();
                                                    growth_count += 1;
                                                }
                                            }
                                        }
                                    }
                                }
                            }
                        }
                    }
                    vm.grid = new_grid;
                    vm.output
                        .push(format!("CRYSTAL: Accreted {} cells", growth_count));
                } else {
                    vm.output
                        .push("Error: Invalid rate for Accrete".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for Accrete".to_string());
            }
        }
        OpCode::Shatter => {
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(force) = val {
                    let mut shattered = 0;

                    // Re-iterate with indices to avoid complexity
                    for y in 0..16 {
                        for x in 0..16 {
                            let mut smash = false;
                            if let Value::Junction(_, vals) = &vm.grid[y][x] {
                                if vals.len() >= 2 && vals[0] == Value::Str("Crystal".to_string()) {
                                    let mut rng = rand::thread_rng();
                                    if rng.gen_range(0..100) < force {
                                        smash = true;
                                    }
                                }
                            }
                            if smash {
                                let mut rng = rand::thread_rng();
                                vm.grid[y][x] = Value::Int(rng.gen_range(0..100));
                                shattered += 1;
                            }
                        }
                    }

                    vm.output
                        .push(format!("CRYSTAL: Shattered {} cells", shattered));
                } else {
                    vm.output
                        .push("Error: Invalid force for Shatter".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for Shatter".to_string());
            }
        }
        OpCode::Anneal => {
            if vm.stack.len() >= 2 {
                let radius_val = vm.stack.pop().unwrap();
                let iter_val = vm.stack.pop().unwrap();

                if let (Value::Int(radius), Value::Int(_iters)) = (radius_val, iter_val) {
                    let (cy, cx) = vm.context_loc;
                    // Gather values
                    let coords = vm.get_circular_coords(cx as i64, cy as i64, radius);
                    let mut values = Vec::new();
                    for &(x, y) in &coords {
                        values.push(vm.grid[y][x].clone());
                    }

                    // Sort (Anneal)
                    values.sort_by(|a, b| match (a, b) {
                        (Value::Int(ia), Value::Int(ib)) => ia.cmp(ib),
                        (Value::Str(sa), Value::Str(sb)) => sa.cmp(sb),
                        // Put Crystals at the end
                        (Value::Junction(_, _), Value::Int(_)) => std::cmp::Ordering::Greater,
                        (Value::Int(_), Value::Junction(_, _)) => std::cmp::Ordering::Less,
                        _ => std::cmp::Ordering::Equal,
                    });

                    // Put back
                    for (i, &(x, y)) in coords.iter().enumerate() {
                        if i < values.len() {
                            vm.grid[y][x] = values[i].clone();
                        }
                    }
                    vm.output
                        .push(format!("CRYSTAL: Annealed {} cells", values.len()));
                } else {
                    vm.output.push("Error: Invalid args for Anneal".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for Anneal".to_string());
            }
        }
        _ => {}
    }
    None
}
