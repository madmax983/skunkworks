use super::normalize_coords;
use crate::vm::{ChimeraVM, Value};
use std::collections::{HashMap, VecDeque};

fn val_to_idx(v: &Value) -> Option<usize> {
    match v {
        Value::Int(n) => Some(*n as usize),
        Value::Str(s) => s.parse::<usize>().ok(),
        _ => None,
    }
}

pub fn apply_chronos_runes(
    rune: &str,
    y: usize,
    x: usize,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    history: &mut HashMap<(usize, usize), VecDeque<Value>>,
) -> bool {
    let mut changes = false;
    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        current_signals[wy][wx].clone()
    } else {
        None
    };

    // Important: Only execute state-modifying actions if we haven't already outputted to Self in this tick.
    // This prevents multiple executions during the propagation loop iterations.
    if next_signals[y][x].is_some() {
        return false;
    }

    match rune {
        "s" => {
            // Sporulate (Save): Input West -> Self History. Output West -> Self.
            if let Some(val) = w_sig {
                let queue = history.entry((y, x)).or_insert_with(VecDeque::new);
                queue.push_back(val.clone());
                // Limit history size to prevent memory explosion
                if queue.len() > 50 {
                    queue.pop_front();
                }

                next_signals[y][x] = Some(val);
                changes = true;
            }
        }
        "g" => {
            // Germinate (Get/Pop Front): Read West Neighbor's History.
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(queue) = history.get_mut(&(wy, wx)) {
                    if let Some(val) = queue.pop_front() {
                        next_signals[y][x] = Some(val);
                        changes = true;
                    }
                }
            }
        }
        "r" => {
            // Retrograde (Reverse/Pop Back): Read West Neighbor's History.
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(queue) = history.get_mut(&(wy, wx)) {
                    if let Some(val) = queue.pop_back() {
                        next_signals[y][x] = Some(val);
                        changes = true;
                    }
                }
            }
        }
        _ => {}
    }
    changes
}

pub fn apply_chronos_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    match rune {
        "c" => {
            // Prophecy: West (Strand Index) -> Simulate -> South (1=Death, 0=Life)
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(sig) = &vm.prologue_state.signal_grid[wy][wx] {
                    if let Some(s_idx) = val_to_idx(sig) {
                        if s_idx < vm.dna.helix.strands.len() {
                            // Clone VM for simulation
                            let mut sim_vm = vm.clone();
                            sim_vm.output.clear();
                            sim_vm.halted = false;
                            sim_vm.ip = (s_idx, 0);

                            // Limit recursion
                            if sim_vm.recursion_depth < crate::vm::MAX_SIMULATION_DEPTH {
                                sim_vm.recursion_depth += 1;
                                let max_ticks = 100;
                                for _ in 0..max_ticks {
                                    sim_vm.step();
                                    if sim_vm.halted || sim_vm.energy <= 0 {
                                        break;
                                    }
                                }
                            }

                            let result = if sim_vm.halted || sim_vm.energy <= 0 {
                                1
                            } else {
                                0
                            };

                            if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                                vm.grid[sy][sx] = Value::Int(result);
                                vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                                // Light up
                            }
                        }
                    }
                }
            }
        }
        _ => {}
    }
}
