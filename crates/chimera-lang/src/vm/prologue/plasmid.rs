//! # Plasmid Agent 🧬
//!
//! The Plasmid (`P`) is a Mobile Genetic Element that roams the grid, acting as a vector for Horizontal Gene Transfer.
//! It absorbs genetic material (Signals) from Sources and conjugates with other Agents to inject this material.
//!
//! ## Behavior
//!
//! 1.  **Absorption**: If a Plasmid is adjacent to a Source (`!`), it absorbs the signal value into its internal payload.
//! 2.  **Conjugation**: If a Plasmid bumps into another Agent (e.g., `C`, `@`), it injects its payload into them.
//!     *   **Critters (`C`)**: The payload is appended to their genome as a new Gene.
//!     *   **Simple Agents**: The payload overwrites their state.
//! 3.  **Movement**: Plasmids drift randomly (Brownian motion) or follow wires (`~`).
//!
//! ## Symbiosis
//!
//! Plasmids are the "Bees" of the Prologue ecosystem, cross-pollinating logic and code across the grid.

use super::{normalize_coords, PrologueAgent};
use crate::vm::{ChimeraVM, Value};
use rand::Rng;

/// Executes the logic for the Plasmid Agent.
///
/// Returns the updated Agent state and its new position (if it moved).
pub fn process_plasmid_logic(
    vm: &mut ChimeraVM,
    agent: &PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(PrologueAgent, Option<(usize, usize)>)> {
    let mut current_agent = agent.clone();
    let (y, x) = (agent.y, agent.x);
    let mut rng = rand::thread_rng();

    // 1. Absorption: Check West for Source (!)
    if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        if let Value::Str(s) = &grid_snapshot[wy][wx] {
            if s == "!" {
                // Check signal grid at (wy, wx)
                if let Some(sig) = &vm.prologue_state.signal_grid[wy][wx] {
                     // Update Agent State with Payload
                    current_agent.state = sig.clone();
                    vm.output.push(format!("PLASMID: Absorbed payload {:?}", sig));
                }
            }
        }
    }

    // 2. Conjugation: Check neighbors for Agents
    let neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];

    // Shuffle neighbors
    let mut shuffled_neighbors = neighbors.to_vec();
    use rand::seq::SliceRandom;
    shuffled_neighbors.shuffle(&mut rng);

    for (dy, dx) in shuffled_neighbors {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            if let Value::Str(s) = &grid_snapshot[ny][nx] {
                if s == "@" || s == "K" || s == "H" || s == "C" {
                    // Inject Payload from updated state
                    let payload = current_agent.state.clone();
                    if let Value::Int(0) = payload {
                        // Empty payload, do nothing
                        continue;
                    }

                    if s == "C" {
                         // Inject into Critter
                        let target_state = vm.prologue_state.registers.get(&(ny, nx)).cloned();

                        if let Some(Value::Str(state_str)) = target_state {
                            if let Ok(mut critter) = state_str.parse::<super::critter::CritterState>() {
                                let gene = match &payload {
                                    Value::Str(g) => g.clone(),
                                    Value::Int(i) => format!("{}", i),
                                    _ => "M".to_string(),
                                };
                                critter.genes.push_str(&gene);
                                let new_state = critter.to_value();
                                vm.prologue_state.registers.insert((ny, nx), new_state);
                                vm.output.push(format!("PLASMID: Conjugated gene '{}' into Critter at {},{}", gene, nx, ny));
                            }
                        }
                    } else {
                        // Overwrite simple agent state
                        vm.prologue_state.registers.insert((ny, nx), payload.clone());
                         vm.output.push(format!("PLASMID: Conjugated payload into {} at {},{}", s, nx, ny));
                    }
                }
            }
        }
    }

    // 3. Movement (Random Walk)
    let mut possible_moves = Vec::new();
    let move_neighbors = [(-1, 0), (1, 0), (0, -1), (0, 1)];
    for (dy, dx) in move_neighbors {
        if let Some((ny, nx)) = normalize_coords(y as i64 + dy, x as i64 + dx) {
            // Can only move to empty space or Wire
            if let Value::Int(0) = &grid_snapshot[ny][nx] {
                 possible_moves.push((ny, nx));
            } else if let Value::Str(s) = &grid_snapshot[ny][nx] {
                if s == "~" {
                    possible_moves.push((ny, nx));
                }
            }
        }
    }

    let target = if !possible_moves.is_empty() {
        let idx = rng.gen_range(0..possible_moves.len());
        Some(possible_moves[idx])
    } else {
        None
    };

    Some((current_agent, target))
}
