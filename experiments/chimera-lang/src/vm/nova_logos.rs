#![cfg(feature = "nova")]

use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
#[cfg(feature = "oracle")]
use crate::ast::JunctionType;
#[cfg(feature = "oracle")]
use crate::vm::oracle;
use crate::opcode::OpCode;
#[cfg(feature = "oracle")]
use std::collections::HashMap;

/// Toggles Logic Chemistry mode.
pub fn exec_logos_op(vm: &mut ChimeraVM, _op: OpCode, _args: &[Nucleotide]) -> Option<(usize, usize)> {
    vm.logos_mode = !vm.logos_mode;
    let status = if vm.logos_mode { "ON" } else { "OFF" };
    vm.output.push(format!("LOGOS: Logic Chemistry {}", status));
    None
}

/// Processes one step of Logic Chemistry.
///
/// Iterates through the grid and triggers reactions based on the Knowledge Base.
/// Rule format: `reaction(Agent, Reagent, Product)`.
/// Effect: Agent + Reagent -> Product + Empty.
pub fn process_logos(vm: &mut ChimeraVM) {
    #[cfg(feature = "oracle")]
    {
        if !vm.logos_mode {
            return;
        }

        let mut updates = Vec::new();
        let grid_size = crate::vm::GRID_SIZE;

        // Phase 1: Scan for reactions
        // We only check for reactions where the current cell is the "Agent" (first arg).
        // To avoid double processing (A reacts with B, then B reacts with A), we could use a strategy.
        // For now, let's just process. Race conditions in cellular automata are feature, not bug.

        for y in 0..grid_size {
            for x in 0..grid_size {
                let cell_val = &vm.grid[y][x];

                // Skip empty cells as Agents
                if let Value::Int(0) = cell_val {
                    continue;
                }

                // Check neighbors
                let neighbors = [(-1, 0), (0, 1), (1, 0), (0, -1)];
                for (dy, dx) in neighbors {
                    if let Some((ny, nx)) = vm.normalize_coords(y as i64 + dy, x as i64 + dx) {
                        let neighbor_val = &vm.grid[ny][nx];

                        // Skip empty reagents
                        if let Value::Int(0) = neighbor_val {
                            continue;
                        }

                        // Query: reaction(Cell, Neighbor, ?Product)
                        // We construct the term: any("reaction", Cell, Neighbor, "?Product")
                        let query = Value::Junction(
                            JunctionType::Any,
                            vec![
                                Value::Str("reaction".to_string()),
                                cell_val.clone(),
                                neighbor_val.clone(),
                                Value::Str("?Product".to_string())
                            ]
                        );

                        let mut solutions = Vec::new();
                        oracle::solve(
                            &[query],
                            HashMap::new(),
                            &vm.knowledge_base,
                            vm,
                            &mut solutions,
                            0
                        );

                        if let Some(sol) = solutions.first() {
                            if let Some(product) = sol.get("?Product") {
                                // Found a reaction!
                                // Schedule update: (y, x) -> Product, (ny, nx) -> 0
                                // Conflict resolution: First come first served in this list.
                                updates.push(((y, x), product.clone()));
                                updates.push(((ny, nx), Value::Int(0)));

                                // Optimization: Break neighbor loop to avoid reacting with multiple neighbors in one tick?
                                // Let's break to keep it 1 reaction per agent per tick.
                                break;
                            }
                        }
                    }
                }
            }
        }

        // Phase 2: Apply updates
        // We need to be careful not to overwrite a cell multiple times in a conflicting way if possible,
        // or just let last write win.
        for ((y, x), val) in updates {
            vm.grid[y][x] = val;
        }
    }
}
