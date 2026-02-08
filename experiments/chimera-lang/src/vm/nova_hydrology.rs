#![cfg(feature = "nova")]

use super::{ChimeraVM, Value, GRID_SIZE};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use rand::Rng;

pub fn exec_hydrology_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::Rain => exec_rain(vm),
        OpCode::Flow => exec_flow(vm),
        OpCode::River => exec_river(vm),
        OpCode::SenseFlow => exec_sense_flow(vm),
        OpCode::Aquifer => exec_aquifer(vm),
        _ => None,
    }
}

fn exec_rain(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: intensity, radius (top)
    if vm.stack.len() >= 2 {
        let rad_val = vm.stack.pop().unwrap();
        let int_val = vm.stack.pop().unwrap();

        if let (Value::Int(r), Value::Int(i)) = (rad_val, int_val) {
            if r > 0 && i > 0 {
                let (cy, cx) = vm.context_loc;
                let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
                let count = coords.len();

                for (tx, ty) in coords {
                    vm.moisture_grid[ty][tx] = vm.moisture_grid[ty][tx].saturating_add(i);
                }

                vm.energy = vm.energy.saturating_sub((count as i64 * i) / 10 + 5);
                vm.output.push(format!(
                    "RAIN: Added {} moisture to {} cells",
                    i, count
                ));
            }
        } else {
            vm.output.push("Error: Type mismatch for rain".to_string());
        }
    } else {
        vm.output.push("Error: Stack underflow for rain".to_string());
    }
    None
}

fn exec_flow(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: iterations (top)
    let iterations = if let Some(Value::Int(i)) = vm.stack.pop() {
        i.clamp(1, 10) as usize
    } else {
        1
    };

    let mut moved_total = 0;

    for _ in 0..iterations {
        // Double buffering for moisture to avoid immediate cascading bias
        let mut next_moisture = vm.moisture_grid.clone();
        // We modify grid (terrain) in place for erosion/deposition

        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let moisture = vm.moisture_grid[y][x];
                if moisture <= 0 {
                    continue;
                }

                // Get current height
                let height = match &vm.grid[y][x] {
                    Value::Int(h) => *h,
                    _ => 0,
                };

                // Find lowest neighbor
                let mut min_h = height;
                let mut target = None;

                // Check neighbors
                let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                for (dy, dx) in neighbors {
                    if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                        let n_h = match &vm.grid[ny][nx] {
                            Value::Int(h) => *h,
                            _ => 0,
                        };
                        // Water + Height determines head, but simple height diff is easier
                        if n_h < min_h {
                            min_h = n_h;
                            target = Some((ny, nx));
                        }
                    }
                }

                if let Some((ty, tx)) = target {
                    let diff = height - min_h;
                    // Move amount proportional to slope and available moisture
                    let flow = (moisture / 2).min(diff * 5).max(1); // Ensure at least 1 flows if diff exists

                    // Move moisture
                    next_moisture[y][x] -= flow;
                    next_moisture[ty][tx] += flow;
                    moved_total += flow;

                    // Erosion/Deposition
                    // Takes sediment from source (erosion)
                    if let Value::Int(h) = &mut vm.grid[y][x] {
                        if *h > -100 { // Bedrock limit
                             *h -= 1;
                        }
                    }
                    // Deposits at target
                    if let Value::Int(h) = &mut vm.grid[ty][tx] {
                        if *h < 100 { // Sky limit
                            *h += 1;
                        }
                    }
                }
            }
        }
        vm.moisture_grid = next_moisture;
    }

    vm.energy = vm.energy.saturating_sub((iterations * 10) as i64);
    vm.output.push(format!(
        "FLOW: Simulated {} iterations, moved {} units",
        iterations, moved_total
    ));
    None
}

fn exec_river(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: width, length (top)
    if vm.stack.len() >= 2 {
        let len_val = vm.stack.pop().unwrap();
        let width_val = vm.stack.pop().unwrap();

        if let (Value::Int(len), Value::Int(width)) = (len_val, width_val) {
            if len > 0 {
                let mut cy = vm.context_loc.0;
                let mut cx = vm.context_loc.1;
                let mut segments = 0;

                for _ in 0..len {
                    // Decide next move BEFORE carving
                    let current_h = match vm.grid[cy][cx] {
                        Value::Int(h) => h,
                        _ => 0,
                    };

                    let mut best_next = None;
                    let min_h = current_h;

                    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
                    let mut valid_neighbors = Vec::new();
                    let mut rng = rand::thread_rng();

                    for (dy, dx) in neighbors {
                        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                             valid_neighbors.push(((ny, nx), match vm.grid[ny][nx] { Value::Int(h) => h, _ => 0 }));
                        }
                    }

                    // Sort by height ascending
                    valid_neighbors.sort_by_key(|&(_, h)| h);

                    if let Some(&((ny, nx), h)) = valid_neighbors.first() {
                        if h < min_h {
                            best_next = Some((ny, nx));
                        } else if h == min_h && !valid_neighbors.is_empty() {
                             // Flat terrain: Random walk
                             let idx = rng.gen_range(0..valid_neighbors.len());
                             best_next = Some(valid_neighbors[idx].0);
                        }
                    }

                    // Carve at current location
                    if let Value::Int(h) = &mut vm.grid[cy][cx] {
                        *h = h.saturating_sub(2);
                    }
                    vm.moisture_grid[cy][cx] = vm.moisture_grid[cy][cx].saturating_add(50);

                    // Carve width
                    if width > 1 {
                        for dy in -1..=1 {
                            for dx in -1..=1 {
                                if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
                                    if let Value::Int(h) = &mut vm.grid[ny][nx] {
                                        *h = h.saturating_sub(1);
                                    }
                                    vm.moisture_grid[ny][nx] = vm.moisture_grid[ny][nx].saturating_add(25);
                                }
                            }
                        }
                    }

                    if let Some((ny, nx)) = best_next {
                        cy = ny;
                        cx = nx;
                        segments += 1;
                    } else {
                        // Pit/Lake formed
                        vm.moisture_grid[cy][cx] += 100;
                        break;
                    }
                }

                vm.energy = vm.energy.saturating_sub(segments * 2);
                vm.output.push(format!("RIVER: Carved {} segments", segments));
            }
        } else {
            vm.output.push("Error: Type mismatch for river".to_string());
        }
    } else {
        vm.output.push("Error: Stack underflow for river".to_string());
    }
    None
}

fn exec_sense_flow(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    let h = match vm.grid[cy][cx] {
        Value::Int(v) => v,
        _ => 0,
    };

    let mut flow_dy = 0;
    let mut flow_dx = 0;
    let mut min_h = h;

    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dy, dx) in neighbors {
        if let Some((ny, nx)) = vm.normalize_coords(cy as i64 + dy, cx as i64 + dx) {
            let nh = match vm.grid[ny][nx] {
                Value::Int(v) => v,
                _ => 0,
            };
            if nh < min_h {
                min_h = nh;
                flow_dy = dy;
                flow_dx = dx;
            }
        }
    }

    vm.stack.push(Value::Int(flow_dy as i64));
    vm.stack.push(Value::Int(flow_dx as i64));
    None
}

fn exec_aquifer(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // stack: amount (top)
    if let Some(Value::Int(amount)) = vm.stack.pop() {
        let (cy, cx) = vm.context_loc;
        if amount > 0 {
            // Pump up (Create moisture)
            vm.moisture_grid[cy][cx] = vm.moisture_grid[cy][cx].saturating_add(amount);
            vm.energy = vm.energy.saturating_sub(amount / 5);
            vm.output.push(format!("AQUIFER: Pumped {} moisture", amount));
        } else if amount < 0 {
            // Drain down (Remove moisture)
            let drain = amount.abs();
            let current = vm.moisture_grid[cy][cx];
            let removed = current.min(drain);
            vm.moisture_grid[cy][cx] -= removed;
            vm.stack.push(Value::Int(removed)); // Return extracted amount
            vm.energy = vm.energy.saturating_sub(removed / 10);
            vm.output.push(format!("AQUIFER: Drained {} moisture", removed));
        }
    } else {
        vm.output.push("Error: Type mismatch for aquifer".to_string());
    }
    None
}
