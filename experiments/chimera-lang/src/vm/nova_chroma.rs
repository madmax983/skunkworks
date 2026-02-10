#![cfg(feature = "nova")]

use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use rand::Rng;

/// Processes interactions based on the color of the current grid cell.
///
/// This function is called every tick if `chroma_shift_mode` is enabled.
/// It reads the `chroma_grid` at `vm.context_loc` and applies effects.
pub fn process_chroma_interaction(vm: &mut ChimeraVM) {
    let (cy, cx) = vm.context_loc;
    let chroma = &vm.chroma_grid[cy][cx];

    if let Some((r, g, b)) = chroma.fg {
        // Classify Color
        // Simple thresholding for primary/secondary colors
        let high = 200;
        let low = 100;

        // Red: Fury (High Cost, Double Execution/Damage)
        if r > high && g < low && b < low {
            vm.energy = vm.energy.saturating_sub(1); // Extra cost
            vm.output.push("CHROMA: Fury (Red)".to_string());
            // Double execution is tricky here as we are in `step`.
            // Instead, we can push a duplicate of the *next* instruction or similar.
            // Or just apply "Fury" state which might affect subsequent calls.
            // For now, let's just burn energy and maybe apply a buff?
            vm.buffs.insert("Fury".to_string(), 1);
        }
        // Green: Growth (Energy Gain, Mutation)
        else if g > high && r < low && b < low {
            vm.energy = vm.energy.saturating_add(1);
            vm.output.push("CHROMA: Growth (Green)".to_string());
            if rand::thread_rng().gen_bool(0.05) {
                vm.mutate();
            }
        }
        // Blue: Stasis (Slow Time, Preservation)
        else if b > high && r < low && g < low {
            // Restore Telomeres
            if vm.ip.0 < vm.telomeres.len() {
                vm.telomeres[vm.ip.0] = vm.telomeres[vm.ip.0].saturating_add(1).min(100);
            }
            vm.output.push("CHROMA: Stasis (Blue)".to_string());
        }
        // Yellow: Haste (Speed)
        else if r > high && g > high && b < low {
            vm.buffs.insert("Haste".to_string(), 1);
            vm.output.push("CHROMA: Haste (Yellow)".to_string());
        }
        // Cyan: Reflection (Reflex)
        else if g > high && b > high && r < low {
            // Trigger a random reflex?
            // Or trigger a specific reflex event based on some condition?
            // Let's trigger reflex 0 (Generic)
            if vm.trigger_reflex(0) {
                vm.output.push("CHROMA: Reflection (Cyan)".to_string());
            }
        }
        // Magenta: Magic (Chaos/Random Op)
        else if r > high && b > high && g < low {
            vm.output.push("CHROMA: Magic (Magenta)".to_string());
            if rand::thread_rng().gen_bool(0.1) {
                // Execute random op
                let op = match rand::thread_rng().gen_range(0..5) {
                    0 => OpCode::Push,
                    1 => OpCode::Add,
                    2 => OpCode::Swap,
                    3 => OpCode::GRead,
                    4 => OpCode::Lumine,
                    _ => OpCode::Nop,
                };
                let _ = vm.execute_gene_inner(op, &[]);
            }
        }
    }
}

pub fn exec_chroma_op(
    vm: &mut ChimeraVM,
    op: OpCode,
    _args: &[Nucleotide],
) -> Option<(usize, usize)> {
    match op {
        OpCode::ChromaShift => {
            vm.chroma_shift_mode = !vm.chroma_shift_mode;
            let status = if vm.chroma_shift_mode { "ON" } else { "OFF" };
            vm.output
                .push(format!("CHROMA: Physics Shift {}", status));
        }
        OpCode::Refract => {
            // Change direction based on current color Hue
            let (cy, cx) = vm.context_loc;
            if let Some((r, g, b)) = vm.chroma_grid[cy][cx].fg {
                // Simple hue mapping to direction
                // R -> North (-1, 0)
                // G -> East (0, 1)
                // B -> South (1, 0)
                // Y -> West (0, -1)
                if r > g && r > b {
                    vm.direction = -1; // "North"? direction is usually +/- 1 in stack processing?
                                       // Wait, vm.direction is isize (1 or -1 usually).
                                       // But for spatial movement (Organelles), it's (dy, dx).
                                       // For main execution, it's just instruction pointer direction?
                                       // Let's assume we affect vm.direction (isize).
                    vm.direction = -1; // Reverse
                } else if g > r && g > b {
                    vm.direction = 1; // Forward
                } else if b > r && b > g {
                    // Skip next?
                    vm.ip.1 = vm.ip.1.saturating_add(1);
                } else {
                    // Random
                    vm.direction = if rand::thread_rng().gen_bool(0.5) {
                        1
                    } else {
                        -1
                    };
                }
                vm.output.push(format!(
                    "REFRACT: Direction set to {} based on color",
                    vm.direction
                ));
            } else {
                vm.output.push("REFRACT: No color to refract".to_string());
            }
        }
        _ => {}
    }
    None
}
