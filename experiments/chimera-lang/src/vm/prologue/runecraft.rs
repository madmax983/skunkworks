use super::normalize_coords;
use crate::ast::JunctionType;
use crate::vm::{ChimeraVM, Value};

pub fn apply_runecraft_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    // 1. Check for Definition Rune £
    if rune == "£" {
        apply_definition_rune(vm, y, x);
        return;
    }

    // 2. Check for The Anvil ⚒
    if rune == "⚒" {
        apply_anvil_rune(vm, y, x);
        return;
    }

    // 3. Check for Custom Rune Execution
    if let Some(&idx) = vm.prologue_state.custom_runes.get(rune) {
        execute_custom_rune(vm, rune, idx, y, x);
    }
}

fn apply_definition_rune(vm: &mut ChimeraVM, y: usize, x: usize) {
    // West: Rune Char (String)
    // North: Strand Index (Int) or Code (String)

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
                // Compile code string to new strand
                match crate::compiler::compile(&code, None) {
                    Ok(mut dna) => {
                        // Take strands from compiled DNA and append to Helix
                        let start_idx = vm.dna.helix.strands.len();
                        vm.dna.helix.strands.append(&mut dna.helix.strands);

                        // Register the FIRST new strand to the rune
                        // (Usually compilation produces 1 main strand, maybe more)
                        if vm.dna.helix.strands.len() > start_idx {
                            vm.prologue_state
                                .custom_runes
                                .insert(char_str.clone(), start_idx);
                            vm.output.push(format!(
                                "RUNECRAFT: Compiled & Defined Custom Rune '{}' -> Strand {}",
                                char_str, start_idx
                            ));
                            vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                        } else {
                            vm.output.push(format!(
                                "RUNECRAFT: Compilation produced no strands for '{}'",
                                char_str
                            ));
                        }
                    }
                    Err(e) => {
                        vm.output.push(format!(
                            "RUNECRAFT: Compilation Failed for '{}': {}",
                            char_str, e
                        ));
                    }
                }
            }
            _ => {}
        }
    }
}

fn execute_custom_rune(vm: &mut ChimeraVM, rune: &str, idx: usize, y: usize, x: usize) {
    // Execute the strand via interrupt
    if idx < vm.dna.helix.strands.len() {
        // We set context_loc so enzymes like `g_read` know where they were triggered from
        vm.context_loc = (y, x);
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

fn apply_anvil_rune(vm: &mut ChimeraVM, y: usize, x: usize) {
    // The Anvil ⚒
    // Input (West): Blueprint (Junction of Dish rows)
    // Action: Flatten to source, Compile, Append to DNA.
    // Output (South): New Strand Index.

    let w_sig = if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx].clone()
    } else {
        None
    };

    if let Some(Value::Junction(JunctionType::Dish, rows)) = w_sig {
        // Flatten Blueprint to Source String
        let mut source_parts = Vec::new();

        for row in rows {
            if let Value::Junction(JunctionType::Dish, cells) = row {
                for cell in cells {
                    match cell {
                        Value::Int(n) => source_parts.push(n.to_string()),
                        Value::Str(s) => {
                            if !s.is_empty() && s != "." {
                                source_parts.push(s);
                            }
                        }
                        _ => {}
                    }
                }
            }
        }

        if source_parts.is_empty() {
            vm.output.push("ANVIL: Empty blueprint".to_string());
            return;
        }

        let body = source_parts.join(" ");
        let strand_name = format!("forged_{}_{}_{}", vm.tick_counter, x, y);
        let source_code = format!("strand {} {{ {} }}", strand_name, body);

        match crate::compiler::compile(&source_code, None) {
            Ok(mut dna) => {
                let start_idx = vm.dna.helix.strands.len();
                if !dna.helix.strands.is_empty() {
                    // Append new strands
                    vm.dna.helix.strands.append(&mut dna.helix.strands);

                    // Emit signal South
                    if let Some((sy, sx)) = normalize_coords(y as i64 + 1, x as i64) {
                        vm.prologue_state.signal_grid[sy][sx] = Some(Value::Int(start_idx as i64));
                    }

                    vm.output.push(format!("ANVIL: Forged strand {}", start_idx));
                    // Self-activate to show success
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(1));
                } else {
                     vm.output.push("ANVIL: Compilation produced no strands".to_string());
                }
            }
            Err(e) => {
                vm.output.push(format!("ANVIL: Compilation Failed: {}", e));
            }
        }
    }
}
