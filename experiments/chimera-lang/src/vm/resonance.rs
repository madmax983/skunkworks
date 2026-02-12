#![cfg(feature = "resonance")]
use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;
use resonance_audio::audio::AudioCommand;

pub fn exec_resonance_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) {
    if op == OpCode::Pluck {
        // stack: strength (top)
        if let Some(val) = vm.stack.pop() {
            if let Value::Int(s) = val {
                // Normalize 100 -> 1.0
                let strength = (s as f32) / 100.0;
                let (cy, cx) = vm.context_loc;

                if let Some(tx) = &vm.audio_tx {
                    // Ignore error if channel full/closed
                    let _ = tx.send(AudioCommand::Pluck {
                        x: cx,
                        y: cy,
                        strength,
                    });
                    vm.output
                        .push(format!("PLUCK: {},{} str={:.2}", cx, cy, strength));
                } else {
                    vm.output
                        .push("PLUCK: No audio channel connected".to_string());
                }
                vm.energy = vm.energy.saturating_sub(1);
            } else {
                vm.output.push("Error: Type mismatch for pluck".to_string());
            }
        } else {
            vm.output
                .push("Error: Stack underflow for pluck".to_string());
        }
    } else if op == OpCode::Oscillate {
        // stack: frequency, strength (top)
        if vm.stack.len() >= 2 {
            let s_val = vm.stack.pop().unwrap();
            let f_val = vm.stack.pop().unwrap();

            if let (Value::Int(s), Value::Int(f)) = (s_val, f_val) {
                let strength = (s as f32) / 100.0;
                let frequency = f as f32;
                let (cy, cx) = vm.context_loc;

                if let Some(tx) = &vm.audio_tx {
                    let _ = tx.send(AudioCommand::Oscillate {
                        x: cx,
                        y: cy,
                        frequency,
                        strength,
                    });
                    vm.output.push(format!(
                        "OSCILLATE: {},{} {}Hz str={:.2}",
                        cx, cy, frequency, strength
                    ));
                } else {
                    vm.output.push("OSCILLATE: No audio channel".to_string());
                }
                vm.energy = vm.energy.saturating_sub(5);
            } else {
                vm.output
                    .push("Error: Type mismatch for oscillate".to_string());
            }
        } else {
            vm.output
                .push("Error: Stack underflow for oscillate".to_string());
        }
    } else if op == OpCode::Hear {
        // stack: [ ... ] -> [ ..., amplitude ]
        let (cy, cx) = vm.context_loc;
        let width = 16; // GRID_SIZE
        let idx = cy * width + cx;
        let val = if idx < vm.audio_snapshot.pressure.len() {
            vm.audio_snapshot.pressure[idx]
        } else {
            0.0
        };
        // Scale f32 (-1.0 to 1.0) to Int (approx -100 to 100)
        let int_val = (val * 100.0) as i64;
        vm.stack.push(Value::Int(int_val));
        vm.energy = vm.energy.saturating_sub(1);
    } else if op == OpCode::Scream {
        // stack: duration, strength
        if vm.stack.len() >= 2 {
            let s_val = vm.stack.pop().unwrap();
            let d_val = vm.stack.pop().unwrap();

            if let (Value::Int(s), Value::Int(d)) = (s_val, d_val) {
                let strength = (s as f32) / 10.0; // High amplitude
                let duration = d as u64;
                let (cy, cx) = vm.context_loc;

                // Audio
                if let Some(tx) = &vm.audio_tx {
                    let _ = tx.send(AudioCommand::Tone {
                        x: cx,
                        y: cy,
                        frequency: 110.0, // Low rumble
                        strength,
                        duration_ms: duration * 10, // 10ms per tick approx
                    });
                    vm.output.push(format!(
                        "SCREAM: {},{} str={:.2} dur={}",
                        cx, cy, strength, duration
                    ));
                } else {
                    vm.output.push("SCREAM: No audio channel".to_string());
                }

                // Visual Shockwave (Manually hacking snapshot for TUI)
                // We want a ring of high values.
                let radius = (s as f32 / 20.0).max(1.0) as usize;
                let width = 16;
                for y in 0..width {
                    for x in 0..width {
                        let dx = (x as isize - cx as isize).abs();
                        let dy = (y as isize - cy as isize).abs();
                        let dist = ((dx * dx + dy * dy) as f32).sqrt();
                        if (dist - radius as f32).abs() < 1.5 {
                            let idx = y * width + x;
                            if idx < vm.audio_snapshot.pressure.len() {
                                vm.audio_snapshot.pressure[idx] = 5.0; // Super bright red
                            }
                        }
                    }
                }

                // Physics Push
                #[cfg(feature = "nova")]
                {
                    // Copy topology to avoid borrow conflict
                    let topology = vm.topology;
                    let width = 16; // GRID_SIZE
                    let height = 16;

                    for i in 0..vm.organelles.len() {
                        let (oy, ox) = vm.organelles[i].context_loc;
                        let dx = ox as f32 - cx as f32;
                        let dy = oy as f32 - cy as f32;
                        let dist = (dx * dx + dy * dy).sqrt();

                        if dist < (strength) && dist > 0.1 {
                            // Push away
                            let push_x = (dx / dist * 2.0).round() as i64;
                            let push_y = (dy / dist * 2.0).round() as i64;

                            if let Some((ny, nx)) =
                                topology.normalize(oy as i64 + push_y, ox as i64 + push_x, width, height)
                            {
                                vm.organelles[i].context_loc = (ny, nx);
                            }
                        }
                    }
                }

                vm.energy = vm.energy.saturating_sub(10);
            } else {
                vm.output
                    .push("Error: Type mismatch for scream".to_string());
            }
        } else {
            vm.output
                .push("Error: Stack underflow for scream".to_string());
        }
    }
}
