#[cfg(feature = "nova")]
use crate::vm::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use crate::opcode::OpCode;
#[cfg(feature = "nova")]
use crate::ast::Nucleotide;
#[cfg(feature = "nova")]
use super::nova_biome::Biome;

#[cfg(feature = "nova")]
pub fn exec_physics_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    match op {
        OpCode::Aeolus => exec_aeolus(vm),
        OpCode::SenseWind => exec_sense_wind(vm),
        OpCode::Storm => exec_storm(vm),
        OpCode::SenseMoisture => exec_sense_moisture(vm),
        OpCode::Terraform => exec_terraform(vm),
        OpCode::SenseBiome => exec_sense_biome(vm),
        OpCode::Gravitate => exec_gravitate(vm),
        OpCode::Lumine => exec_lumine(vm),
        OpCode::SenseLight => exec_sense_light(vm),
        OpCode::Irradiate => exec_irradiate(vm),
        OpCode::SenseMutagen => exec_sense_mutagen(vm),
        OpCode::Detox => exec_detox(vm),
        _ => None,
    }
}

#[cfg(feature = "nova")]
fn exec_aeolus(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., angle, strength ]
    if vm.stack.len() >= 2 {
        let str_val = vm.stack.pop().unwrap();
        let ang_val = vm.stack.pop().unwrap();

        if let (Value::Int(ang), Value::Int(str)) = (ang_val, str_val) {
            let strength = str.clamp(0, 10) as i8;
            let (dy, dx) = match ang.rem_euclid(8) {
                0 => (-1, 0),  // N
                1 => (-1, 1),  // NE
                2 => (0, 1),   // E
                3 => (1, 1),   // SE
                4 => (1, 0),   // S
                5 => (1, -1),  // SW
                6 => (0, -1),  // W
                7 => (-1, -1), // NW
                _ => (0, 0),
            };

            let vec = (dy * strength, dx * strength);
            let (cy, cx) = vm.context_loc;
            vm.wind_grid[cy][cx] = vec;

            vm.energy = vm.energy.saturating_sub(5);
            vm.output
                .push(format!("AEOLUS: Wind set to {:?} at {},{}", vec, cx, cy));
        } else {
            vm.output
                .push("Error: Type mismatch for Aeolus".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for Aeolus".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_sense_wind(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let (dy, dx) = vm.wind_grid[cy][cx];
    vm.stack.push(Value::Int(dy as i64));
    vm.stack.push(Value::Int(dx as i64));
    None
}

#[cfg(feature = "nova")]
fn exec_storm(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., intensity, radius ]
    if vm.stack.len() >= 2 {
        let rad_val = vm.stack.pop().unwrap();
        let int_val = vm.stack.pop().unwrap();

        if let (Value::Int(int), Value::Int(rad)) = (int_val, rad_val) {
            if rad > 0 && int > 0 {
                let (cy, cx) = vm.context_loc;
                let coords = vm.get_circular_coords(cx as i64, cy as i64, rad);
                for (tx, ty) in coords {
                    vm.moisture_grid[ty][tx] = vm.moisture_grid[ty][tx].saturating_add(int);
                }
                vm.energy = vm.energy.saturating_sub(int / 2 + rad);
                vm.output
                    .push(format!("STORM: Rain intensity {} at {},{}", int, cx, cy));
            }
        } else {
            vm.output.push("Error: Type mismatch for Storm".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for Storm".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_sense_moisture(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    vm.stack.push(Value::Int(vm.moisture_grid[cy][cx]));
    None
}

#[cfg(feature = "nova")]
fn exec_terraform(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: biome_id, radius (top)
    if vm.stack.len() >= 2 {
        let radius_val = vm.stack.pop().unwrap();
        let id_val = vm.stack.pop().unwrap();

        if let (Value::Int(r), Value::Int(id)) = (radius_val, id_val) {
            if r > 0 {
                let biome = match id {
                    0 => Biome::Plains,
                    1 => Biome::Swamp,
                    2 => Biome::Desert,
                    3 => Biome::Tundra,
                    4 => Biome::Volcanic,
                    _ => Biome::Plains,
                };

                let (cy, cx) = vm.context_loc;
                let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
                let count = coords.len();

                for (tx, ty) in coords {
                    vm.biome_grid[ty][tx] = biome;
                }

                // Terraforming is expensive
                vm.energy = vm.energy.saturating_sub(count as i64 * 5);
                vm.output.push(format!(
                    "TERRAFORM: Changed {} cells to {:?} at {},{}",
                    count, biome, cx, cy
                ));
            }
        } else {
            vm.output
                .push("Error: Type mismatch for terraform".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for terraform".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_sense_biome(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let biome = vm.biome_grid[cy][cx];
    let id = match biome {
        Biome::Plains => 0,
        Biome::Swamp => 1,
        Biome::Desert => 2,
        Biome::Tundra => 3,
        Biome::Volcanic => 4,
    };
    vm.stack.push(Value::Int(id));
    None
}

#[cfg(feature = "nova")]
fn exec_gravitate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: radius
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(r) = val {
            if r > 0 {
                let (cy, cx) = vm.context_loc;
                // Get coordinates within radius
                // Note: get_circular_coords uses Euclidean distance on Plane.
                let coords = vm.get_circular_coords(cx as i64, cy as i64, r);

                // Calculate distances and sort
                let mut coords_with_dist: Vec<((usize, usize), i64)> = coords
                    .into_iter()
                    .map(|(x, y)| {
                        let dx = x as i64 - cx as i64;
                        let dy = y as i64 - cy as i64;
                        // Squared distance is sufficient for sorting
                        ((x, y), dx * dx + dy * dy)
                    })
                    .collect();

                // Sort by distance (ascending)
                coords_with_dist.sort_by_key(|&(_, d)| d);

                let mut moved_count = 0;

                for ((tx, ty), dist_sq) in coords_with_dist {
                    if dist_sq == 0 {
                        continue; // Skip center
                    }

                    // If empty, skip
                    if matches!(vm.grid[ty][tx], Value::Int(0)) {
                        continue;
                    }

                    // Calculate target (one step closer to center)
                    let dx = cx as i64 - tx as i64;
                    let dy = cy as i64 - ty as i64;

                    let sx = if dx > 0 {
                        1
                    } else if dx < 0 {
                        -1
                    } else {
                        0
                    };
                    let sy = if dy > 0 {
                        1
                    } else if dy < 0 {
                        -1
                    } else {
                        0
                    };

                    // Use normalize_coords to find valid target
                    if let Some((target_y, target_x)) =
                        vm.normalize_coords(ty as i64 + sy, tx as i64 + sx)
                    {
                        // Check if target is empty
                        if matches!(vm.grid[target_y][target_x], Value::Int(0)) {
                            // Move
                            vm.grid[target_y][target_x] = vm.grid[ty][tx].clone();
                            vm.grid[ty][tx] = Value::Int(0);
                            moved_count += 1;
                        }
                    }
                }

                vm.energy = vm.energy.saturating_sub(moved_count + 5); // Base cost + variable
                vm.output.push(format!(
                    "GRAVITATE: Pulled {} items towards {},{}",
                    moved_count, cx, cy
                ));
            } else {
                // Negative or zero radius is no-op
            }
        } else {
            vm.output
                .push("Error: Type mismatch for gravitate".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for gravitate".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_lumine(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: intensity, radius (bottom)
    if vm.stack.len() >= 2 {
        let intensity_val = vm.stack.pop().unwrap();
        let radius_val = vm.stack.pop().unwrap();
        if let (Value::Int(r), Value::Int(intensity)) = (radius_val, intensity_val) {
            if r > 0 && intensity > 0 {
                let (cy, cx) = vm.context_loc;
                let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
                let count = coords.len();
                for (tx, ty) in coords {
                    vm.light_grid[ty][tx] = vm.light_grid[ty][tx].saturating_add(intensity);
                }
                vm.energy = vm.energy.saturating_sub((count / 2) as i64);
                vm.output.push(format!(
                    "LUMINE: Emitted {} light at {},{} r={}",
                    intensity, cx, cy, r
                ));
            }
        } else {
            vm.output
                .push("Error: Type mismatch for lumine".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for lumine".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_sense_light(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let intensity = vm.light_grid[cy][cx];
    vm.stack.push(Value::Int(intensity));
    None
}

#[cfg(feature = "nova")]
fn exec_irradiate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: amount, radius (top)
    if vm.stack.len() >= 2 {
        let radius_val = vm.stack.pop().unwrap();
        let amount_val = vm.stack.pop().unwrap();
        if let (Value::Int(r), Value::Int(amount)) = (radius_val, amount_val) {
            if r > 0 && amount > 0 {
                let (cy, cx) = vm.context_loc;
                let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
                for (tx, ty) in coords {
                    vm.mutagen_grid[ty][tx] =
                        vm.mutagen_grid[ty][tx].saturating_add(amount);
                }
                // Cost is proportional to amount and area
                vm.energy = vm
                    .energy
                    .saturating_sub((r * r + 1).clamp(5, 50) + amount / 10);
                vm.output.push(format!(
                    "IRRADIATE: Added {} mutagen at {},{} r={}",
                    amount, cx, cy, r
                ));
            }
        } else {
            vm.output
                .push("Error: Type mismatch for irradiate".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for irradiate".to_string());
    }
    None
}

#[cfg(feature = "nova")]
fn exec_sense_mutagen(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let level = vm.mutagen_grid[cy][cx];
    vm.stack.push(Value::Int(level));
    None
}

#[cfg(feature = "nova")]
fn exec_detox(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: radius (top)
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(r) = val {
            let (cy, cx) = vm.context_loc;
            let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
            for (tx, ty) in coords {
                vm.waste_grid[ty][tx] = 0;
            }
            vm.energy = vm.energy.saturating_sub((r * r + 1).clamp(5, 50)); // Cost proportional to area
            vm.output
                .push(format!("DETOX: Cleansed radius {} at {},{}", r, cx, cy));
        } else {
            vm.output.push("Error: Type mismatch for detox".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for detox".to_string());
    }
    None
}
