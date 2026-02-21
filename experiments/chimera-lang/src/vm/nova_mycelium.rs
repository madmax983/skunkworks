use serde::{Deserialize, Serialize};
use crate::vm::{ChimeraVM, Value};
use crate::vm::nova::get_direction_mask;
use std::collections::{VecDeque, HashSet, HashMap};
use rand::Rng;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct MyceliumNode {
    pub connections: Vec<(usize, usize)>,
    pub resources: i64,
    pub age: u64,
}

pub fn exec_hyphae(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    let (cy, cx) = vm.context_loc;
    if let std::collections::hash_map::Entry::Vacant(e) = vm.mycelium.entry((cy, cx)) {
        e.insert(MyceliumNode {
            connections: Vec::new(),
            resources: 50, // Initial resources
            age: 0,
        });
        vm.energy = vm.energy.saturating_sub(20);
        vm.output.push(format!("HYPHAE: Sprouted at {},{}", cx, cy));
    } else {
        vm.output
            .push(format!("HYPHAE: Node already exists at {},{}", cx, cy));
    }
    None
}

pub fn exec_connect(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
            if let Some((ty, tx)) = vm.normalize_coords(y, x) {
                let (cy, cx) = vm.context_loc;
                let has_source = vm.mycelium.contains_key(&(cy, cx));
                let has_target = vm.mycelium.contains_key(&(ty, tx));

                if has_source && has_target {
                    // Add target to source
                    if let Some(node) = vm.mycelium.get_mut(&(cy, cx)) {
                        if !node.connections.contains(&(ty, tx)) {
                            node.connections.push((ty, tx));
                        }
                    }

                    // Add source to target
                    if let Some(node) = vm.mycelium.get_mut(&(ty, tx)) {
                        if !node.connections.contains(&(cy, cx)) {
                            node.connections.push((cy, cx));
                        }
                    }

                    vm.energy = vm.energy.saturating_sub(10);
                    vm.output.push(format!(
                        "CONNECT: Mycelium linked {},{} <-> {},{}",
                        cx, cy, tx, ty
                    ));
                } else {
                    vm.output
                        .push("CONNECT: Both ends must be Hyphae".to_string());
                }
            } else {
                vm.output
                    .push("Error: Coordinates out of bounds for connect".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for connect".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for connect".to_string());
    }
    None
}

pub fn exec_transport(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 3 {
        let x_val = vm.stack.pop().unwrap();
        let y_val = vm.stack.pop().unwrap();
        let val = vm.stack.pop().unwrap();

        if let (Value::Int(x), Value::Int(y)) = (x_val, y_val) {
            if let Some((ty, tx)) = vm.normalize_coords(y, x) {
                let (cy, cx) = vm.context_loc;

                // Pathfinding BFS
                let mut queue = VecDeque::new();
                let mut visited = HashSet::new();
                queue.push_back((cy, cx));
                visited.insert((cy, cx));

                let mut found = false;
                while let Some(curr) = queue.pop_front() {
                    if curr == (ty, tx) {
                        found = true;
                        break;
                    }
                    if let Some(node) = vm.mycelium.get(&curr) {
                        for &next in &node.connections {
                            if !visited.contains(&next) {
                                visited.insert(next);
                                queue.push_back(next);
                            }
                        }
                    }
                }

                if found {
                    vm.grid[ty][tx] = val;
                    vm.energy = vm.energy.saturating_sub(5);
                    vm.output
                        .push(format!("TRANSPORT: Sent value to {},{}", tx, ty));
                } else {
                    vm.output
                        .push("TRANSPORT: No mycelial path found".to_string());
                }
            } else {
                vm.output
                    .push("Error: Coordinates out of bounds for transport".to_string());
            }
        } else {
            vm.output
                .push("Error: Type mismatch for transport".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for transport".to_string());
    }
    None
}

pub fn exec_spore_cloud(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    if vm.stack.len() >= 2 {
        let dens_val = vm.stack.pop().unwrap();
        let rad_val = vm.stack.pop().unwrap();
        if let (Value::Int(r), Value::Int(d)) = (rad_val, dens_val) {
            let (cy, cx) = vm.context_loc;
            let mut rng = rand::thread_rng();

            let mut count = 0;
            let mut targets = Vec::new();

            crate::vm::iterate_circle(
                #[cfg(feature = "nova")]
                vm.topology,
                cx as i64,
                cy as i64,
                r,
                |tx, ty| {
                    if rng.gen_range(0..100) < d {
                        targets.push((ty, tx));
                    }
                },
            );

            for (ty, tx) in targets {
                if let std::collections::hash_map::Entry::Vacant(e) =
                    vm.mycelium.entry((ty, tx))
                {
                    e.insert(MyceliumNode {
                        connections: Vec::new(),
                        resources: 20,
                        age: 0
                    });
                    count += 1;
                }
            }

            vm.energy = vm.energy.saturating_sub(count * 5);
            vm.output
                .push(format!("SPORE_CLOUD: Sprouted {} hyphae", count));
        } else {
            vm.output
                .push("Error: Type mismatch for spore_cloud".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for spore_cloud".to_string());
    }
    None
}

/// Simulates the living fungal network.
/// Handles metabolism, foraging, resource sharing, and growth.
pub fn process_mycelium(vm: &mut ChimeraVM) {
    let keys: Vec<(usize, usize)> = vm.mycelium.keys().cloned().collect();
    if keys.is_empty() { return; }

    let mut death_row = Vec::new();
    let mut growth_sprouts = Vec::new();
    let mut resource_deltas: HashMap<(usize, usize), i64> = HashMap::new();

    // 1. Forage & Metabolism
    for (y, x) in &keys {
        let mut absorbed = 0;

        // Forage from grid
        if let Some(val) = vm.grid.get_mut(*y).and_then(|row| row.get_mut(*x)) {
            if let Value::Int(n) = val {
                if *n > 0 {
                    absorbed = *n;
                    *val = Value::Int(0); // Consume
                }
            }
        }

        // Emit Pheromone (Scent)
        // Add scent "Hyphae" at (y,x) with intensity 50
        vm.pheromones.push(crate::vm::nova_scent::Scent {
            x: *x as f64,
            y: *y as f64,
            intensity: 50.0,
            signature: "Hyphae".to_string(),
            age: 0,
        });

        // Mutate Node
        if let Some(node) = vm.mycelium.get_mut(&(*y, *x)) {
            node.resources = node.resources.saturating_add(absorbed);
            node.resources = node.resources.saturating_sub(1); // Metabolism
            node.age += 1;

            if node.resources <= 0 {
                death_row.push((*y, *x));
            } else if node.resources > 100 {
                // Growth Candidates
                growth_sprouts.push((*y, *x));
            }
        }
    }

    // 2. Transport (Osmosis)
    // We calculate deltas without applying them to avoid order dependency in this tick
    for (y, x) in &keys {
        if let Some(node) = vm.mycelium.get(&(*y, *x)) {
            for target in &node.connections {
                if let Some(neighbor) = vm.mycelium.get(target) {
                    // Flow from High to Low
                    if node.resources > neighbor.resources + 5 {
                        *resource_deltas.entry((*y, *x)).or_insert(0) -= 1;
                        *resource_deltas.entry(*target).or_insert(0) += 1;
                    }
                }
            }
        }
    }

    // Apply Deltas
    for (coords, delta) in resource_deltas {
        if let Some(node) = vm.mycelium.get_mut(&coords) {
            if delta > 0 {
                node.resources = node.resources.saturating_add(delta.abs() as i64);
            } else {
                node.resources = node.resources.saturating_sub(delta.abs() as i64);
            }
        }
    }

    // 3. Growth
    let mut rng = rand::thread_rng();
    for (py, px) in growth_sprouts {
        // Check if parent still has resources (might have lost them in transport)
        let parent_res = vm.mycelium.get(&(py, px)).map(|n| n.resources).unwrap_or(0);
        if parent_res > 100 {
            // Find empty neighbor
            let mut candidates = Vec::new();
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = vm.normalize_coords(py as i64 + dy, px as i64 + dx) {
                    if !vm.mycelium.contains_key(&(ny, nx)) {
                        // Check if blocked by wall/membrane?
                        // Assuming Hyphae can grow through standard space.
                        candidates.push((ny, nx));
                    }
                }
            }

            if !candidates.is_empty() {
                let idx = rng.gen_range(0..candidates.len());
                let (ny, nx) = candidates[idx];

                // Pay cost
                if let Some(node) = vm.mycelium.get_mut(&(py, px)) {
                    node.resources -= 50;
                    node.connections.push((ny, nx));
                }

                // Spawn new
                vm.mycelium.insert((ny, nx), MyceliumNode {
                    connections: vec![(py, px)],
                    resources: 20,
                    age: 0,
                });

                vm.output.push(format!("HYPHAE: Grown at {},{}", nx, ny));
            }
        }
    }

    // 4. Death
    for coords in death_row {
        if let Some(node) = vm.mycelium.remove(&coords) {
            // Remove connections in neighbors
            for neighbor_coords in node.connections {
                if let Some(neighbor) = vm.mycelium.get_mut(&neighbor_coords) {
                    neighbor.connections.retain(|&c| c != coords);
                }
            }
            vm.output.push(format!("HYPHAE: Withered at {},{}", coords.1, coords.0));
        }
    }
}
