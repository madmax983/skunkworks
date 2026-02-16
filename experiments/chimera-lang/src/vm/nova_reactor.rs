#[cfg(feature = "nova")]
use super::{ChimeraVM, Value};
#[cfg(feature = "nova")]
use crate::ast::JunctionType;
#[cfg(feature = "nova")]
use crate::vm::oracle;
#[cfg(feature = "nova")]
use std::collections::HashMap;

#[cfg(feature = "nova")]
pub fn process_reactor(vm: &mut ChimeraVM) {
    if !vm.reactor_mode {
        return;
    }

    let size = super::GRID_SIZE;
    let mut changes = Vec::new();

    // Iterate over grid to find reactions
    // Optimization: Skip empty cells? No, empty cells might react with neighbors (e.g. Spreading).
    for y in 0..size {
        for x in 0..size {
            let val = &vm.grid[y][x];

            // Check neighbors (Von Neumann neighborhood for simplicity)
            let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];

            for (dy, dx) in neighbors {
                if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                    let neighbor_val = &vm.grid[ny][nx];

                    // Skip if both are 0 (Empty space doesn't react with empty space)
                    if matches!(val, Value::Int(0)) && matches!(neighbor_val, Value::Int(0)) {
                        continue;
                    }

                    // Check Cache
                    let key = (val.clone(), neighbor_val.clone());
                    let result_opt: Option<Value> = if let Some(cached) = vm.reactor_cache.get(&key)
                    {
                        let v: Option<Value> = cached.clone();
                        v
                    } else {
                        // Query Oracle: reaction(A, B, ?Result)
                        let query = Value::Junction(
                            JunctionType::Any,
                            vec![
                                Value::Str("reaction".to_string()),
                                val.clone(),
                                neighbor_val.clone(),
                                Value::Str("?Result".to_string()),
                            ],
                        );

                        let mut solutions = Vec::new();
                        // Limit depth to avoid stalls
                        #[cfg(feature = "oracle")]
                        oracle::solve(
                            &[query],
                            HashMap::new(),
                            &vm.knowledge_base,
                            vm,
                            &mut solutions,
                            0,
                        );

                        let res = if let Some(sol) = solutions.first() {
                            if let Some(r) = sol.get("?Result") {
                                Some(r.clone())
                            } else {
                                None
                            }
                        } else {
                            None
                        };

                        // Update Cache
                        vm.reactor_cache.insert(key, res.clone());
                        res
                    };

                    if let Some(new_val) = result_opt {
                        // We found a reaction!
                        // Reaction logic: A + B -> A' (Result)
                        // This updates the CURRENT cell based on neighbor.
                        // Does it consume the neighbor? Usually CA rules just update self based on neighbors.
                        // Let's assume the rule defines the new state of A* when adjacent to B.

                        // Avoid overwriting if multiple neighbors cause reaction?
                        // First match wins? Or sum?
                        // Let's just take the first reaction and break.
                        changes.push((y, x, new_val));

                        // Mark flash
                        vm.reactor_flash[y][x] = 255;

                        // Cost energy?
                        vm.energy = vm.energy.saturating_sub(1);
                        break;
                    }
                }
            }
        }
    }

    // Apply changes
    for (y, x, val) in changes {
        vm.grid[y][x] = val;
    }
}

#[cfg(feature = "nova")]
pub fn exec_reactor(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    vm.reactor_mode = !vm.reactor_mode;
    let status = if vm.reactor_mode { "ON" } else { "OFF" };
    vm.output
        .push(format!("REACTOR: Logic Automata {}", status));
    None
}

#[cfg(feature = "nova")]
pub fn exec_reaction(vm: &mut ChimeraVM) -> Option<(usize, usize)> {
    // Stack: [ ..., InputA, InputB, Output ]
    if vm.stack.len() >= 3 {
        let output = vm.stack.pop().unwrap();
        let input_b = vm.stack.pop().unwrap();
        let input_a = vm.stack.pop().unwrap();

        // Actually, if it's unconditional, we can just assert the fact reaction(A, B, Out).
        // But using "rule" format allows future expansion.
        // Let's just assert the fact for simplicity if body is empty.
        // Wait, `oracle.rs` expects `rule` wrapper for rules, or raw fact.
        // `reaction(A, B, Out)` is a fact.

        let fact = Value::Junction(
            JunctionType::Any,
            vec![Value::Str("reaction".to_string()), input_a, input_b, output],
        );

        if !vm.knowledge_base.contains(&fact) {
            vm.knowledge_base.push(fact.clone());
            // Clear cache because rules changed
            vm.reactor_cache.clear();
            vm.output.push(format!("REACTION: Added {}", fact));
        } else {
            vm.output.push("REACTION: Rule already exists".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for reaction".to_string());
    }
    None
}
