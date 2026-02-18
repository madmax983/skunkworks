#![cfg(feature = "nova")]

use crate::vm::{ChimeraVM, Value};
use super::normalize_coords;
use crate::vm::nova_chronos;

pub fn apply_chronos_sink(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    match rune {
        "s" => {
            // Sporulate: Signal West -> Save State, Output ID South
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if vm.prologue_state.signal_grid[wy][wx].is_some() {
                    let spore = nova_chronos::create_spore(vm);
                    let id = vm.spores.len();
                    vm.spores.push(spore);
                    vm.output.push(format!("PROLOGUE: Sporulated ID {}", id));

                    // Write ID to South
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        vm.grid[sy][sx] = Value::Int(id as i64);
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                    }
                }
            }
        }
        "g" => {
            // Germinate: West (Spore ID) -> Restore State
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(Value::Int(id)) = &vm.prologue_state.signal_grid[wy][wx] {
                    let idx = *id as usize;
                    if idx < vm.spores.len() {
                        let spore = vm.spores[idx].clone();
                        nova_chronos::restore_state(vm, &spore);
                        vm.output.push(format!("PROLOGUE: Germinated Spore {}", idx));
                        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                    } else {
                        vm.output.push(format!("PROLOGUE: Invalid Spore ID {}", idx));
                    }
                }
            }
        }
        "r" => {
            // Retrograde: West (Ticks) -> Revert Grid History
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(Value::Int(ticks)) = &vm.prologue_state.signal_grid[wy][wx] {
                    if *ticks > 0 {
                        let history_len = vm.grid_history.len();
                        let t_idx = *ticks as usize;
                        if t_idx < history_len {
                            let idx = history_len.saturating_sub(1).saturating_sub(t_idx);
                            let past_grid = vm.grid_history[idx].clone();
                            vm.grid = past_grid;
                            vm.output.push(format!("PROLOGUE: Retrograde {} ticks", ticks));
                            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                        }
                    }
                }
            }
        }
        _ => {}
    }
}
