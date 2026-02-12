#![cfg(all(feature = "nova", feature = "resonance"))]

use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;

pub fn exec_sift(vm: &mut ChimeraVM, _op: OpCode, _args: &[Nucleotide]) {
    // Stack: [ ..., radius ]
    if let Some(val) = vm.stack.pop() {
        if let Value::Int(r) = val {
            if r > 0 {
                let (cy, cx) = vm.context_loc;
                let coords = vm.get_circular_coords(cx as i64, cy as i64, r);
                let width = 16; // GRID_SIZE

                // We need audio snapshot.
                // vm.audio_snapshot.pressure is Vec<f32>, length GRID_SIZE*GRID_SIZE (256).

                if vm.audio_snapshot.pressure.len() != width * width {
                    vm.output
                        .push("SIFT ERROR: Audio snapshot invalid size".to_string());
                    return;
                }

                let mut moved_count: i64 = 0;

                // Process coordinates
                for (tx, ty) in coords {
                    // Skip empty cells
                    if matches!(vm.grid[ty][tx], Value::Int(0)) {
                        continue;
                    }

                    let curr_idx = ty * width + tx;
                    let curr_amp = vm.audio_snapshot.pressure[curr_idx].abs();

                    let mut best_amp = curr_amp;
                    let mut best_pos = None;

                    // Check neighbors
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dy == 0 && dx == 0 {
                                continue;
                            }
                            if let Some((ny, nx)) =
                                vm.normalize_coords(ty as i64 + dy, tx as i64 + dx)
                            {
                                // Must be empty
                                if matches!(vm.grid[ny][nx], Value::Int(0)) {
                                    let n_idx = ny * width + nx;
                                    let n_amp = vm.audio_snapshot.pressure[n_idx].abs();
                                    // Move towards LOWER amplitude (node)
                                    if n_amp < best_amp {
                                        best_amp = n_amp;
                                        best_pos = Some((ny, nx));
                                    }
                                }
                            }
                        }
                    }

                    if let Some((ny, nx)) = best_pos {
                        vm.grid[ny][nx] = vm.grid[ty][tx].clone();
                        vm.grid[ty][tx] = Value::Int(0);
                        moved_count += 1;
                    }
                }

                vm.energy = vm.energy.saturating_sub(moved_count / 2 + 5);
                vm.output
                    .push(format!("SIFT: Moved {} particles to nodes", moved_count));
            } else {
                vm.output.push("Error: Invalid radius for sift".to_string());
            }
        } else {
            vm.output.push("Error: Type mismatch for sift".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for sift".to_string());
    }
}

pub fn exec_reshape(vm: &mut ChimeraVM, _op: OpCode, _args: &[Nucleotide]) {
    // Stack: [ ..., threshold, mode ]
    if vm.stack.len() >= 2 {
        let mode_val = vm.stack.pop().unwrap();
        let thresh_val = vm.stack.pop().unwrap();

        if let (Value::Int(threshold_int), Value::Int(mode)) = (thresh_val, mode_val) {
            let threshold = (threshold_int as f32) / 100.0;
            let width = 16;

            if vm.audio_snapshot.pressure.len() != width * width {
                vm.output
                    .push("RESHAPE ERROR: Audio snapshot invalid".to_string());
                return;
            }

            let mut count: i64 = 0;

            for y in 0..width {
                for x in 0..width {
                    let idx = y * width + x;
                    let amp = vm.audio_snapshot.pressure[idx].abs();

                    if amp > threshold {
                        match mode {
                            0 => {
                                // Solidify: Create walls (Membrane 15 = Full Block)
                                if vm.membranes[y][x] != 15 {
                                    vm.membranes[y][x] = 15;
                                    count += 1;
                                }
                            }
                            1 => {
                                // Liquefy: Remove walls
                                if vm.membranes[y][x] != 0 {
                                    vm.membranes[y][x] = 0;
                                    count += 1;
                                }
                            }
                            _ => {}
                        }
                    }
                }
            }

            vm.energy = vm.energy.saturating_sub(count + 10);
            let mode_str = if mode == 0 { "Solidified" } else { "Liquefied" };
            vm.output.push(format!(
                "RESHAPE: {} {} cells (Thresh {:.2})",
                mode_str, count, threshold
            ));
        } else {
            vm.output
                .push("Error: Type mismatch for reshape".to_string());
        }
    } else {
        vm.output
            .push("Error: Stack underflow for reshape".to_string());
    }
}
