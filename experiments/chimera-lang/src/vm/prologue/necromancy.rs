use super::normalize_coords;
use crate::vm::ChimeraVM;
use crate::vm::Value;

pub fn apply_necromancy_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
) -> bool {
    // Only light up if triggered by West signal
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "†" | "‡" | "Ψ" => {
            if w_sig.is_some() && next_signals[y][x].is_none() {
                next_signals[y][x] = Some(Value::Int(1));
                return true;
            }
        }
        _ => {}
    }
    false
}

pub fn apply_necromancy_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx].clone()
    } else {
        None
    };

    if w_sig.is_none() {
        return;
    }

    match rune {
        "†" => {
            // Bury: Moves strand/value to Graveyard.
            if let Some(Value::Int(idx)) = w_sig {
                let idx = idx as usize;
                if idx < vm.dna.helix.strands.len() {
                    let strand = vm.dna.helix.strands[idx].clone();
                    if vm.graveyard.len() >= crate::vm::MAX_GRAVEYARD_SIZE {
                        vm.graveyard.remove(0); // FIFO
                    }
                    vm.graveyard.push(strand);

                    // Clear the original strand to remove it from Helix logic without breaking indices
                    vm.dna.helix.strands[idx].genes.clear();

                    vm.output.push(format!("NECROMANCY: Buried strand {}", idx));
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1)); // Light up
                } else {
                    vm.output
                        .push(format!("NECROMANCY: Invalid strand index {}", idx));
                }
            }
        }
        "‡" => {
            // Exhume: Restores last buried strand to Helix.
            if let Some(strand) = vm.graveyard.pop() {
                vm.dna.helix.strands.push(strand);
                let new_idx = vm.dna.helix.strands.len() - 1;

                // Output index to South
                if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                    vm.grid[sy][sx] = Value::Int(new_idx as i64);
                }
                vm.output
                    .push(format!("NECROMANCY: Exhumed strand to {}", new_idx));
                vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
            } else {
                vm.output.push("NECROMANCY: Graveyard empty".to_string());
            }
        }
        "Ψ" => {
            // Seance: Executes last buried strand as Ghost.
            if let Some(strand) = vm.graveyard.last() {
                if vm.dna.helix.strands.len() >= crate::vm::MAX_STRANDS {
                    vm.output
                        .push("NECROMANCY: Helix full, cannot Seance".to_string());
                    return;
                }

                // Temporarily append to Helix
                let ghost_strand = strand.clone();
                vm.dna.helix.strands.push(ghost_strand);
                let ghost_idx = vm.dna.helix.strands.len() - 1;

                // Trigger interrupt
                vm.interrupt(ghost_idx);

                vm.output
                    .push(format!("NECROMANCY: Seance for ghost {}", ghost_idx));
                vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
            } else {
                vm.output.push("NECROMANCY: Graveyard empty".to_string());
            }
        }
        _ => {}
    }
}
