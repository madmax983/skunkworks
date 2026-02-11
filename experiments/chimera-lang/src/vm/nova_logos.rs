#![cfg(feature = "nova")]

use crate::ast::JunctionType;
#[cfg(feature = "oracle")]
use crate::vm::oracle;
use crate::vm::{ChimeraVM, Value};
#[cfg(feature = "oracle")]
use std::collections::HashMap;

/// Processes Logic Chemistry (Logos) interactions.
///
/// Iterates over the grid. If two adjacent cells contain terms that satisfy a `reaction` rule
/// in the Oracle Knowledge Base, they react and transform.
///
/// Supported Rule formats:
/// 1. `reaction(Agent, Reagent, NewAgent, NewReagent)`
///    - Explicit transformation of both cells.
/// 2. `reaction(Agent, Reagent, Product)`
///    - `Agent` becomes `Product`, `Reagent` is consumed (becomes 0).
///    - If `Product` is `Junction(All, [A, B])`, `Agent` becomes `A`, `Reagent` becomes `B`.
pub fn process_logos(vm: &mut ChimeraVM) {
    #[cfg(feature = "oracle")]
    {
        let mut updates = Vec::new();
        let size = crate::vm::GRID_SIZE;

        // Immutable phase: Query reactions
        {
            let kb = &vm.knowledge_base;

            for y in 0..size {
                for x in 0..size {
                    let agent = &vm.grid[y][x];
                    if matches!(agent, Value::Int(0)) {
                        continue;
                    }

                    // Check neighbors (Right and Down only to avoid double processing)
                    let neighbors = [(0, 1), (1, 0)]; // dy, dx

                    for (dy, dx) in neighbors {
                        if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                            let reagent = &vm.grid[ny][nx];
                            if matches!(reagent, Value::Int(0)) {
                                continue;
                            }

                            // Try 4-arity first: reaction(A, B, ?NA, ?NB)
                            let query4 = Value::Junction(
                                JunctionType::Any,
                                vec![
                                    Value::Str("reaction".to_string()),
                                    agent.clone(),
                                    reagent.clone(),
                                    Value::Str("?NA".to_string()),
                                    Value::Str("?NB".to_string()),
                                ],
                            );

                            let mut solutions = Vec::new();
                            oracle::solve(&[query4], HashMap::new(), kb, vm, &mut solutions, 0);

                            if let Some(sol) = solutions.first() {
                                let na = sol.get("?NA").cloned().unwrap_or(agent.clone());
                                let nb = sol.get("?NB").cloned().unwrap_or(reagent.clone());
                                updates.push((y, x, na));
                                updates.push((ny, nx, nb));
                                continue; // Found 4-arity match
                            }

                            // Fallback to 3-arity: reaction(A, B, ?Result)
                            let query3 = Value::Junction(
                                JunctionType::Any,
                                vec![
                                    Value::Str("reaction".to_string()),
                                    agent.clone(),
                                    reagent.clone(),
                                    Value::Str("?Result".to_string()),
                                ],
                            );

                            solutions.clear();
                            oracle::solve(&[query3], HashMap::new(), kb, vm, &mut solutions, 0);

                            if let Some(sol) = solutions.first() {
                                if let Some(result) = sol.get("?Result") {
                                    // Determine outcome
                                    let (res_agent, res_reagent) = match result {
                                        Value::Junction(JunctionType::All, parts)
                                            if parts.len() == 2 =>
                                        {
                                            (parts[0].clone(), parts[1].clone())
                                        }
                                        val => (val.clone(), Value::Int(0)), // Default: Agent transforms, Reagent consumed
                                    };

                                    updates.push((y, x, res_agent));
                                    updates.push((ny, nx, res_reagent));
                                }
                            }
                        }
                    }
                }
            }
        }

        // Mutable phase: Apply updates
        let count = updates.len() / 2;
        if count > 0 {
            for (y, x, val) in updates {
                vm.grid[y][x] = val;
            }
            // Exothermic: Gain energy from reactions
            vm.energy = vm.energy.saturating_add(count as i64 * 2);
            vm.output
                .push(format!("LOGOS: {} reactions occurred", count));
        }
    }
}
