use super::{ChimeraVM, Value};
use crate::vm::nova::{get_direction_mask, Phase};
use crate::vm::nova_biome::Biome;

pub fn exec_gravitate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: radius
    let r = vm.pop_int("gravitate")?;

    if r > 0 {
        let (cy, cx) = vm.context_loc;
        // Get coordinates within radius
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
            if let Some((target_y, target_x)) = vm.normalize_coords(ty as i64 + sy, tx as i64 + sx)
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
    None
}

pub fn exec_migrate(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: dy, dx (top)
    let mut dx = vm.pop_int("migrate")?;
    let mut dy = vm.pop_int("migrate")?;

    if vm.chirality == crate::vm::Chirality::Right {
        dy = -dy;
        dx = -dx;
    }

    if vm.phase == Phase::Crystalline {
        vm.output
            .push("Error: Crystalline phase is immobile".to_string());
        return None;
    }

    let (cy, cx) = vm.context_loc;

    let mut blocked = false;
    if vm.phase != Phase::Ethereal {
        if let Some(mask) = get_direction_mask(dy, dx) {
            if (vm.membranes[cy][cx] & mask) != 0 {
                blocked = true;
            }
        }
    }

    if blocked {
        // Blocked by membrane
        vm.energy = vm.energy.saturating_sub(2);
        vm.output.push("MIGRATE: Blocked by membrane".to_string());
        if vm.trigger_reflex(0) {
            return Some(vm.ip);
        }
        return None;
    }

    if let Some((mut new_y, mut new_x)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
        // Check for portal
        if let Some(&(py, px)) = vm.portals.get(&(new_y, new_x)) {
            vm.output.push(format!(
                "PORTAL: Teleported from {},{} to {},{}",
                new_x, new_y, px, py
            ));
            new_y = py;
            new_x = px;
        }

        vm.context_loc = (new_y, new_x);
        vm.energy = vm.energy.saturating_sub(5);
        vm.output
            .push(format!("MIGRATE: moved to {},{}", new_x, new_y));

        if let Some(target) = crate::vm::nova_ward::check_ward_trigger(vm) {
            return Some(target);
        }
    } else {
        // Hit boundary
        vm.energy = vm.energy.saturating_sub(2);
        vm.output.push("MIGRATE: Blocked by boundary".to_string());
        if vm.trigger_reflex(0) {
            return Some(vm.ip);
        }
    }

    None
}

pub fn exec_relativity(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    vm.relativity_mode = !vm.relativity_mode;
    let status = if vm.relativity_mode { "ON" } else { "OFF" };
    vm.output
        .push(format!("RELATIVITY: Physics engine {}", status));
    None
}

pub fn exec_graviton(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    vm.gravity_grid[cy][cx] = vm.gravity_grid[cy][cx].saturating_add(50);
    vm.energy = vm.energy.saturating_sub(10);
    vm.output
        .push(format!("GRAVITON: Emitted at {},{}", cx, cy));
    None
}

pub fn exec_event_horizon(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let g = vm.gravity_grid[cy][cx];
    vm.stack.push(Value::Int(g));
    None
}

pub fn exec_sense_wind(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let (dy, dx) = vm.wind_grid[cy][cx];
    vm.stack.push(Value::Int(dy as i64));
    vm.stack.push(Value::Int(dx as i64));
    None
}

pub fn exec_sense_moisture(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    vm.stack.push(Value::Int(vm.moisture_grid[cy][cx]));
    None
}

pub fn exec_terraform(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
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
                    5 => Biome::Glitch,
                    6 => Biome::Aether,
                    7 => Biome::Silicon,
                    8 => Biome::Garden,
                    _ => Biome::Plains,
                };

                let (cy, cx) = vm.context_loc;
                let mut count = 0;
                crate::vm::iterate_circle(
                    #[cfg(feature = "nova")]
                    vm.topology,
                    cx as i64,
                    cy as i64,
                    r,
                    |tx, ty| {
                        vm.biome_grid[ty][tx] = biome;
                        count += 1;
                    },
                );

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

pub fn exec_sense_biome(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let biome = vm.biome_grid[cy][cx];
    let id = match biome {
        Biome::Plains => 0,
        Biome::Swamp => 1,
        Biome::Desert => 2,
        Biome::Tundra => 3,
        Biome::Volcanic => 4,
        Biome::Glitch => 5,
        Biome::Aether => 6,
        Biome::Silicon => 7,
        Biome::Garden => 8,
    };
    vm.stack.push(Value::Int(id));
    None
}

pub fn exec_shape(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(t) = val {
            let new_topology = match t {
                0 => Some(crate::vm::Topology::Plane),
                1 => Some(crate::vm::Topology::Torus),
                2 => Some(crate::vm::Topology::CylinderH),
                3 => Some(crate::vm::Topology::CylinderV),
                4 => Some(crate::vm::Topology::Klein),
                5 => Some(crate::vm::Topology::Mobius),
                6 => Some(crate::vm::Topology::Hyperbolic),
                7 => Some(crate::vm::Topology::Sphere),
                8 => Some(crate::vm::Topology::Projective),
                _ => None,
            };

            if let Some(topo) = new_topology {
                vm.topology = topo;
                vm.output
                    .push(format!("SHAPE: Changed topology to {:?}", topo));
                vm.energy = vm.energy.saturating_sub(100);
            } else {
                vm.output
                    .push(format!("Error: Invalid topology index {}", t));
            }
        } else {
            vm.output.push("Error: Type mismatch for shape".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for shape".to_string());
    }
    None
}

pub fn exec_rift(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 4 {
        let x2_val = vm.stack.pop().unwrap();
        let y2_val = vm.stack.pop().unwrap();
        let x1_val = vm.stack.pop().unwrap();
        let y1_val = vm.stack.pop().unwrap();

        if let (Value::Int(x1), Value::Int(y1), Value::Int(x2), Value::Int(y2)) =
            (x1_val, y1_val, x2_val, y2_val)
        {
            if (0..16).contains(&x1)
                && (0..16).contains(&y1)
                && (0..16).contains(&x2)
                && (0..16).contains(&y2)
            {
                vm.portals
                    .insert((y1 as usize, x1 as usize), (y2 as usize, x2 as usize));
                vm.energy = vm.energy.saturating_sub(50);
                vm.output.push(format!(
                    "RIFT: Opened portal from {},{} to {},{}",
                    x1, y1, x2, y2
                ));
            } else {
                vm.output
                    .push("Error: Coordinates out of bounds for rift".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for rift".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for rift".to_string());
    }
    None
}

pub fn exec_seal(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
            if (0..16).contains(&x) && (0..16).contains(&y) {
                if vm.portals.remove(&(y as usize, x as usize)).is_some() {
                    vm.energy = vm.energy.saturating_sub(10);
                    vm.output
                        .push(format!("SEAL: Closed portal at {},{}", x, y));
                } else {
                    vm.output
                        .push(format!("SEAL: No portal found at {},{}", x, y));
                }
            } else {
                vm.output
                    .push("Error: Coordinates out of bounds for seal".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for seal".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for seal".to_string());
    }
    None
}

pub fn exec_isomerize(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    vm.chirality = match vm.chirality {
        crate::vm::Chirality::Left => crate::vm::Chirality::Right,
        crate::vm::Chirality::Right => crate::vm::Chirality::Left,
    };
    vm.output
        .push(format!("ISOMERIZE: Switched to {:?}", vm.chirality));
    None
}

pub fn exec_phase_shift(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(id) = val {
            let phase = match id {
                1 => Phase::Ethereal,
                2 => Phase::Crystalline,
                3 => Phase::Flux,
                _ => Phase::Corporeal,
            };
            vm.phase = phase;
            vm.energy = vm.energy.saturating_sub(50);
            vm.output
                .push(format!("PHASE_SHIFT: Transformed to {:?}", phase));
        } else {
            vm.output
                .push("Error: Type mismatch for phase_shift".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for phase_shift".to_string());
    }
    None
}

pub fn exec_membrane(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let mask_val = vm.pop_int("membrane")?;

    let mask = mask_val as u8;
    let (cy, cx) = vm.context_loc;

    vm.membranes[cy][cx] ^= mask;

    if (mask & 1) != 0 {
        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 - 1, cx as i64) {
            vm.membranes[ny][nx] ^= 2;
        }
    }
    if (mask & 2) != 0 {
        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + 1, cx as i64) {
            vm.membranes[ny][nx] ^= 1;
        }
    }
    if (mask & 4) != 0 {
        if let Some((ny, nx)) = vm.normalize_coords(cy as i64, cx as i64 + 1) {
            vm.membranes[ny][nx] ^= 8;
        }
    }
    if (mask & 8) != 0 {
        if let Some((ny, nx)) = vm.normalize_coords(cy as i64, cx as i64 - 1) {
            vm.membranes[ny][nx] ^= 4;
        }
    }

    vm.energy = vm.energy.saturating_sub(10);
    vm.output
        .push(format!("MEMBRANE: Toggled mask {} at {},{}", mask, cx, cy));

    None
}

pub fn exec_osmosis(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let dx = vm.pop_int("osmosis")?;
    let dy = vm.pop_int("osmosis")?;

    let (cy, cx) = vm.context_loc;
    if let Some((mut new_y, mut new_x)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
        if let Some(&(py, px)) = vm.portals.get(&(new_y, new_x)) {
            vm.output.push(format!(
                "PORTAL: Teleported from {},{} to {},{}",
                new_x, new_y, px, py
            ));
            new_y = py;
            new_x = px;
        }

        vm.context_loc = (new_y, new_x);
        vm.energy = vm.energy.saturating_sub(20);
        vm.output
            .push(format!("OSMOSIS: Moved to {},{}", new_x, new_y));

        if let Some(target) = crate::vm::nova_ward::check_ward_trigger(vm) {
            return Some(target);
        }
    } else {
        vm.energy = vm.energy.saturating_sub(5);
        vm.output.push("OSMOSIS: Blocked by boundary".to_string());
    }
    None
}
