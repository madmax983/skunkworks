#![cfg(feature = "nova")]

#[cfg(feature = "oracle")]
use crate::ast::JunctionType;
#[cfg(feature = "oracle")]
use crate::value::Value;
#[cfg(feature = "oracle")]
use crate::vm::oracle;
use crate::vm::ChimeraVM;
#[cfg(feature = "oracle")]
use std::collections::HashMap;

/// Processes Logic Chemistry (Logos) interactions.
///
/// Iterates over the grid. If two adjacent cells contain terms that satisfy a `reaction/3` rule
/// in the Oracle Knowledge Base, they react and transform.
///
/// Rule format: `reaction(Agent, Reagent, Product)`
/// - `Agent`: The cell at (y, x)
/// - `Reagent`: The neighbor cell
/// - `Product`: The result.
///   - If `Product` is a single value, `Agent` becomes `Product` and `Reagent` becomes 0 (consumed).
///   - If `Product` is `Junction(All, [A, B])`, `Agent` becomes `A` and `Reagent` becomes `B`.
pub fn process_logos(vm: &mut ChimeraVM) {
    #[cfg(not(feature = "oracle"))]
    let _ = vm;
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
                    // Actually, we should check all directions if we want directional reactions,
                    // but standard chemistry usually implies random mixing.
                    // To prevent order bias, checking all is better, but expensive.
                    // Let's check East and South for now to ensure pair uniqueness.
                    // (x,y) interacts with (x+1, y) and (x, y+1).

                    let neighbors = [(0, 1), (1, 0)]; // dy, dx

                    for (dy, dx) in neighbors {
                        if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                            let reagent = &vm.grid[ny][nx];
                            if matches!(reagent, Value::Int(0)) {
                                continue;
                            }

                            // Query: reaction(Agent, Reagent, ?Result)
                            let query = Value::Junction(
                                JunctionType::Any,
                                vec![
                                    Value::Str("reaction".to_string()),
                                    agent.clone(),
                                    reagent.clone(),
                                    Value::Str("?Result".to_string()),
                                ],
                            );

                            let mut solutions = Vec::new();
                            oracle::solve(&[query], HashMap::new(), kb, vm, &mut solutions, 0);

                            if let Some(sol) = solutions.first() {
                                if let Some(result) = sol.get("?Result") {
                                    // Found a reaction!
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
            vm.energy = vm.energy.saturating_add(count as i64 * 2); // Exothermic
            vm.output
                .push(format!("LOGOS: {} reactions occurred", count));
        }
    }
}
