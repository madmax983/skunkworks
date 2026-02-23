use super::normalize_coords;
use crate::vm::{ChimeraVM, Value};

pub fn apply_runecraft_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    // 1. Check for Definition Rune £
    if rune == "£" {
        apply_definition_rune(vm, y, x);
        return;
    }

    // 2. Check for Custom Rune Execution
    if let Some(&idx) = vm.prologue_state.custom_runes.get(rune) {
        execute_custom_rune(vm, rune, idx, y, x);
    }
}

fn apply_definition_rune(vm: &mut ChimeraVM, y: usize, x: usize) {
    // West: Rune Char (String)
    // North: Strand Index (Int)

    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx].clone()
    } else {
        None
    };

    let n_sig = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
        vm.prologue_state.signal_grid[ny][nx].clone()
    } else {
        None
    };

    if let (Some(Value::Str(char_str)), Some(val)) = (w_sig, n_sig) {
        // "Runes" are typically single char in Prologue, but strings are fine.
        // Let's stick to whatever String they give.

        match val {
            Value::Int(idx) => {
                if idx >= 0 {
                    vm.prologue_state
                        .custom_runes
                        .insert(char_str.clone(), idx as usize);
                    vm.output.push(format!(
                        "RUNECRAFT: Defined Custom Rune '{}' -> Strand {}",
                        char_str, idx
                    ));
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                }
            }
            Value::Str(code) => {
                // Future expansion: Compile code string to new strand
                vm.output.push(format!(
                    "RUNECRAFT: Define Code '{}' -> '{}' (Not implemented yet)",
                    char_str, code
                ));
            }
            _ => {}
        }
    }
}

fn execute_custom_rune(vm: &mut ChimeraVM, rune: &str, idx: usize, y: usize, x: usize) {
    // Execute the strand via interrupt
    if idx < vm.dna.helix.strands.len() {
        vm.interrupt(idx);
        vm.output
            .push(format!("RUNECRAFT: Executed Custom Rune '{}'", rune));
        vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1)); // Signal activation
    } else {
        vm.output.push(format!(
            "RUNECRAFT: Failed to execute '{}' (Invalid Strand {})",
            rune, idx
        ));
    }
}
