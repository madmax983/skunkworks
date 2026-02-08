use super::{ChimeraVM, Value, GRID_SIZE};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;

pub fn exec_elektra_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) {
    match op {
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
}

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
                    Value::Int(1) | Value::Int(2) | Value::Int(3) => 10.0, // Wire
                    Value::Int(_) => 0.01, // Other matter
                    _ => 0.0, // Air
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
                            Value::Int(1) | Value::Int(2) | Value::Int(3) => 10.0,
                            Value::Int(_) => 0.01,
                            _ => 0.0,
                        };

                        // Harmonic mean of conductivity? Or just avg?
                        // Simple averaging
                        let coupling = (conductivity + neighbor_cond) / 2.0;

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
