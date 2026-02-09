use super::{ChimeraVM, Value, GRID_SIZE};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;

pub fn exec_elektra_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    match op {
        OpCode::Electrogenesis => {
            // [amount]
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(amount) = val {
                    let cost = amount.abs().max(1);
                    if vm.energy >= cost {
                        vm.energy -= cost;
                        let (y, x) = vm.context_loc;
                        vm.voltage_grid[y][x] += amount as f32;
                        vm.output.push(format!("ELECTROGENESIS: +{}V at {},{}", amount, x, y));
                    } else {
                        vm.output.push("ELECTROGENESIS: Not enough energy".to_string());
                    }
                }
            }
        }
        OpCode::Induction => {
            // [] -> [amount]
            let (y, x) = vm.context_loc;
            let v = vm.voltage_grid[y][x];
            let energy_gain = v.abs() as i64;
            if energy_gain > 0 {
                vm.energy = vm.energy.saturating_add(energy_gain);
                vm.voltage_grid[y][x] = 0.0; // Absorb charge
                vm.stack.push(Value::Int(energy_gain));
                vm.output.push(format!("INDUCTION: Absorbed {}V -> {} Energy", v, energy_gain));
            } else {
                vm.stack.push(Value::Int(0));
            }
        }
        OpCode::WireGrowth => {
            // [direction]
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(dir) = val {
                    if vm.energy >= 5 {
                        let (cy, cx) = vm.context_loc;
                        // 0=N, 1=E, 2=S, 3=W
                        let (dy, dx) = match dir % 4 {
                            0 => (-1, 0),
                            1 => (0, 1),
                            2 => (1, 0),
                            3 => (0, -1),
                            _ => (0, 0),
                        };

                        // Check bounds manually or use helper?
                        // vm.normalize_coords might wrap if torus, but here let's assume valid grid.
                        // Actually, better to check bounds.
                        let ny = cy as i64 + dy;
                        let nx = cx as i64 + dx;

                        if ny >= 0 && ny < GRID_SIZE as i64 && nx >= 0 && nx < GRID_SIZE as i64 {
                            let ny = ny as usize;
                            let nx = nx as usize;
                            vm.grid[ny][nx] = Value::Int(1); // Wire
                            vm.energy -= 5;
                            vm.output.push(format!("WIREGROWTH: Wire at {},{}", nx, ny));
                        } else {
                             vm.output.push("WIREGROWTH: Out of bounds".to_string());
                        }
                    } else {
                        vm.output.push("WIREGROWTH: Not enough energy".to_string());
                    }
                }
            }
        }
        OpCode::CircuitBreaker => {
            // [threshold, strand_idx]
            if vm.stack.len() >= 2 {
                let s_val = vm.stack.pop().unwrap();
                let t_val = vm.stack.pop().unwrap();

                if let (Value::Int(strand_idx), Value::Int(threshold)) = (s_val, t_val) {
                    let (y, x) = vm.context_loc;
                    let v = vm.voltage_grid[y][x];
                    if v > threshold as f32 {
                        if strand_idx >= 0 {
                             vm.output.push(format!("CIRCUITBREAKER: Tripped at {}V > {}", v, threshold));
                             return Some((strand_idx as usize, 0));
                        }
                    }
                }
            }
        }
        OpCode::Battery => {
            // [voltage, y, x]
            if vm.stack.len() >= 3 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                let v_val = vm.stack.pop().unwrap();
                if let (Value::Int(x), Value::Int(y), Value::Int(v)) = (x_val, y_val, v_val) {
                    if x >= 0 && x < GRID_SIZE as i64 && y >= 0 && y < GRID_SIZE as i64 {
                        let ux = x as usize;
                        let uy = y as usize;
                        vm.voltage_grid[uy][ux] = v as f32;
                        vm.resistance_grid[uy][ux] = -1.0; // Mark as Battery
                        vm.output.push(format!("BATTERY: {}V at {},{}", v, x, y));
                    }
                }
            }
        }
        OpCode::Ground => {
            // [y, x]
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
                    if x >= 0 && x < GRID_SIZE as i64 && y >= 0 && y < GRID_SIZE as i64 {
                        let ux = x as usize;
                        let uy = y as usize;
                        vm.voltage_grid[uy][ux] = 0.0;
                        vm.resistance_grid[uy][ux] = -2.0; // Mark as Ground
                        vm.output.push(format!("GROUND: 0V at {},{}", x, y));
                    }
                }
            }
        }
        OpCode::SenseVolt => {
            // [y, x] -> [volts]
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
                    if x >= 0 && x < GRID_SIZE as i64 && y >= 0 && y < GRID_SIZE as i64 {
                        let v = vm.voltage_grid[y as usize][x as usize];
                        vm.stack.push(Value::Int(v as i64));
                    } else {
                        vm.stack.push(Value::Int(0));
                    }
                }
            }
        }
        OpCode::Shock => {
            // [power, radius] (Centered on context_loc)
            if vm.stack.len() >= 2 {
                let r_val = vm.stack.pop().unwrap();
                let p_val = vm.stack.pop().unwrap();
                if let (Value::Int(r), Value::Int(p)) = (r_val, p_val) {
                    let (cy, cx) = vm.context_loc;
                    let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
                    for (x, y) in coords {
                        // Reset voltage/resistance? Or just damage?
                        // Let's reset component status (blow fuse)
                        if vm.resistance_grid[y][x] < 0.0 {
                            vm.resistance_grid[y][x] = 1.0; // Reset to air
                            vm.output.push(format!("SHOCK: Blown fuse at {},{}", x, y));
                        }
                    }
                    vm.output.push(format!("SHOCK: Discharged {} power", p));
                }
            }
        }
        OpCode::Lightning => {
            // [y, x] - Visual + Sound?
            if vm.stack.len() >= 2 {
                let x_val = vm.stack.pop().unwrap();
                let y_val = vm.stack.pop().unwrap();
                if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
                    vm.output.push(format!("LIGHTNING: Strike at {},{}", x, y));
                    // Could trigger neighbors
                }
            }
        }
        _ => {}
    }
    None
}

#[allow(clippy::needless_range_loop)]
pub fn update_circuit(vm: &mut ChimeraVM) {
    // Iterative solver for potential
    // V[new] = avg(V[neighbors])
    // Resistance affects coupling.
    // If R is high (Air), coupling is low.
    // If R is low (Wire), coupling is high.
    // Sources (R < 0) are fixed.

    let iterations = 10;
    let grid_size = GRID_SIZE;

    // Temporary grid for next step
    let mut next_voltage = vm.voltage_grid.clone();

    for _ in 0..iterations {
        for y in 0..grid_size {
            for x in 0..grid_size {
                let r_self = vm.resistance_grid[y][x];

                // Fixed nodes
                if r_self < 0.0 {
                    next_voltage[y][x] = vm.voltage_grid[y][x];
                    continue;
                }

                // Determine effective resistance based on grid content
                // 1 (Wire) -> 0.1, Else -> 100.0 (Air)
                // If resistance_grid is set to something else (e.g. by future resistor op), use it?
                // For now, resistance_grid > 0 is just "initialized".
                // We trust grid content more for wiring.
                let cell_val = &vm.grid[y][x];
                let conductivity = match cell_val {
                    Value::Int(0) => 0.0,                                  // Air
                    Value::Int(1) | Value::Int(2) | Value::Int(3) => 10.0, // Wire
                    Value::Int(_) => 0.01,                                 // Other matter
                    _ => 0.0,                                              // Air
                };

                if conductivity <= 0.001 {
                    // Insulator, V decays to 0
                    next_voltage[y][x] = vm.voltage_grid[y][x] * 0.9;
                    continue;
                }

                let mut v_sum = 0.0;
                let mut weight_sum = 0.0;

                let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                for (dy, dx) in neighbors {
                    let ny = y as i64 + dy;
                    let nx = x as i64 + dx;
                    if ny >= 0 && ny < grid_size as i64 && nx >= 0 && nx < grid_size as i64 {
                        let ny = ny as usize;
                        let nx = nx as usize;

                        let neighbor_val = &vm.grid[ny][nx];
                        let neighbor_cond = match neighbor_val {
                            Value::Int(0) => 0.0,
                            Value::Int(1) | Value::Int(2) | Value::Int(3) => 10.0,
                            Value::Int(_) => 0.01,
                            _ => 0.0,
                        };

                        // Harmonic mean of conductivity (Series conductance)
                        // If either is 0 (Air), no coupling.
                        let coupling = if conductivity == 0.0 || neighbor_cond == 0.0 {
                            0.0
                        } else {
                            (conductivity * neighbor_cond) / (conductivity + neighbor_cond)
                        };

                        v_sum += vm.voltage_grid[ny][nx] * coupling;
                        weight_sum += coupling;
                    }
                }

                if weight_sum > 0.0 {
                    next_voltage[y][x] = v_sum / weight_sum;
                } else {
                    next_voltage[y][x] = vm.voltage_grid[y][x] * 0.95; // Decay
                }
            }
        }
        vm.voltage_grid = next_voltage.clone();
    }

    // Calculate Current (I = dV * Conductance)
    // We'll store magnitude of current flow
    for y in 0..grid_size {
        for x in 0..grid_size {
            let cell_val = &vm.grid[y][x];
            let conductivity = match cell_val {
                Value::Int(0) => 0.0,
                Value::Int(1) | Value::Int(2) | Value::Int(3) => 10.0,
                Value::Int(_) => 0.01,
                _ => 0.0,
            };

            let mut max_diff = 0.0;
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dy, dx) in neighbors {
                let ny = y as i64 + dy;
                let nx = x as i64 + dx;
                if ny >= 0 && ny < grid_size as i64 && nx >= 0 && nx < grid_size as i64 {
                    let ny = ny as usize;
                    let nx = nx as usize;
                    let diff = (vm.voltage_grid[y][x] - vm.voltage_grid[ny][nx]).abs();
                    if diff > max_diff {
                        max_diff = diff;
                    }
                }
            }
            vm.current_grid[y][x] = max_diff * conductivity;
        }
    }
}
