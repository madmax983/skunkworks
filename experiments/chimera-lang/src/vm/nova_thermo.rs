#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use crate::opcode::OpCode;

#[cfg(feature = "nova")]
pub fn diffuse_heat(vm: &mut ChimeraVM) {
    let size = crate::vm::GRID_SIZE;
    let mut buffer = vec![vec![20i64; size]; size];

    // Ambient temperature
    let ambient = 20;

    for y in 0..size {
        for x in 0..size {
            let inertia = vm.biome_grid[y][x].diffusion_inertia();
            // Higher inertia = holds temperature better (insulation)
            let weight_center = inertia as i128 * 2;
            let mut sum = (vm.temperature_grid[y][x] as i128) * weight_center;
            let mut total_weight = weight_center;

            // Neighbors
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dy, dx) in neighbors {
                if let Some(mask) = crate::vm::nova::get_direction_mask(dy, dx) {
                    if (vm.membranes[y][x] & mask) != 0 {
                        continue; // Blocked by membrane
                    }
                }

                if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                    let (w_dy, w_dx) = vm.wind_grid[ny][nx];
                    // Wind flow from neighbor to here
                    let flow = -(w_dy as i128 * dy as i128 + w_dx as i128 * dx as i128);
                    let weight = (10 + flow).max(0);

                    sum += (vm.temperature_grid[ny][nx] as i128) * weight;
                    total_weight += weight;
                }
            }

            // Pull towards ambient
            sum += ambient as i128 * 5;
            total_weight += 5;

            if total_weight > 0 {
                buffer[y][x] = (sum / total_weight) as i64;
            }
        }
    }

    vm.temperature_grid = buffer;
}

#[cfg(feature = "nova")]
pub fn process_phase_changes(vm: &mut ChimeraVM) {
    let size = crate::vm::GRID_SIZE;

    for y in 0..size {
        for x in 0..size {
            let temp = vm.temperature_grid[y][x];

            if temp > 100 {
                // Boiling
                if vm.moisture_grid[y][x] > 0 {
                    // Evaporate moisture: reduces local moisture, increases humidity elsewhere?
                    // For simplicity: convert moisture to "Steam" cloud value if cell is empty
                    if matches!(vm.grid[y][x], Value::Int(0)) {
                        vm.grid[y][x] = Value::Str("Steam".to_string());
                    }
                    vm.moisture_grid[y][x] = vm.moisture_grid[y][x].saturating_sub(10);
                }
            } else if temp < 0 {
                // Freezing
                if vm.moisture_grid[y][x] > 10 {
                    // Freeze into Ice
                    if matches!(vm.grid[y][x], Value::Int(0)) || matches!(vm.grid[y][x], Value::Str(ref s) if s == "Water") {
                        vm.grid[y][x] = Value::Str("Ice".to_string());
                    }
                }
            } else if temp > 500 {
                // Fire / Plasma
                if !matches!(vm.grid[y][x], Value::Int(0)) {
                    // Chance to burn
                    if rand::random::<f64>() < 0.1 {
                        vm.grid[y][x] = Value::Str("Ash".to_string());
                        vm.output.push(format!("THERMO: Burned cell at {},{}", x, y));
                    }
                }
            }
        }
    }
}

#[cfg(feature = "nova")]
pub fn exec_thermo_op(vm: &mut ChimeraVM, op: OpCode, _args: &[crate::ast::Nucleotide]) -> Option<(usize, usize)> {
    match op {
        OpCode::Exothermic => {
            // Stack: [ ..., amount, radius ]
            if vm.stack.len() >= 2 {
                let r_val = vm.stack.pop().unwrap();
                let amt_val = vm.stack.pop().unwrap();

                if let (Value::Int(amount), Value::Int(r)) = (amt_val, r_val) {
                    if r > 0 && amount > 0 {
                        let (cy, cx) = vm.context_loc;
                        let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
                        for (tx, ty) in coords {
                            vm.temperature_grid[ty][tx] = vm.temperature_grid[ty][tx].saturating_add(amount);
                        }

                        vm.energy = vm.energy.saturating_sub(amount / 2);
                        vm.output.push(format!("EXOTHERMIC: Heat +{} radius {} at {},{}", amount, r, cx, cy));
                    }
                } else {
                    vm.output.push("Error: Type mismatch for exothermic".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for exothermic".to_string());
            }
            None
        }
        OpCode::Endothermic => {
            // Stack: [ ..., amount, radius ]
            if vm.stack.len() >= 2 {
                let r_val = vm.stack.pop().unwrap();
                let amt_val = vm.stack.pop().unwrap();

                if let (Value::Int(amount), Value::Int(r)) = (amt_val, r_val) {
                    if r > 0 && amount > 0 {
                        let (cy, cx) = vm.context_loc;
                        let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
                        for (tx, ty) in coords {
                            vm.temperature_grid[ty][tx] = vm.temperature_grid[ty][tx].saturating_sub(amount);
                        }

                        vm.energy = vm.energy.saturating_sub(amount / 2);
                        vm.output.push(format!("ENDOTHERMIC: Heat -{} radius {} at {},{}", amount, r, cx, cy));
                    }
                } else {
                    vm.output.push("Error: Type mismatch for endothermic".to_string());
                }
            } else {
                vm.output.push("Error: Stack underflow for endothermic".to_string());
            }
            None
        }
        OpCode::Thermometer => {
            let (cy, cx) = vm.context_loc;
            let temp = vm.temperature_grid[cy][cx];
            vm.stack.push(Value::Int(temp));
            None
        }
        OpCode::State => {
            let (cy, cx) = vm.context_loc;
            let temp = vm.temperature_grid[cy][cx];

            let state = if temp < 0 {
                0 // Solid (Ice)
            } else if temp <= 100 {
                1 // Liquid (Water)
            } else if temp <= 500 {
                2 // Gas (Steam)
            } else {
                3 // Plasma
            };

            vm.stack.push(Value::Int(state));
            None
        }
        _ => None,
    }
}
