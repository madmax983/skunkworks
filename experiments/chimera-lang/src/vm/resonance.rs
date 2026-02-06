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
        let val = if idx < vm.audio_snapshot.len() {
            vm.audio_snapshot[idx]
        } else {
            0.0
        };
        // Scale f32 (-1.0 to 1.0) to Int (approx -100 to 100)
        let int_val = (val * 100.0) as i64;
        vm.stack.push(Value::Int(int_val));
        vm.energy = vm.energy.saturating_sub(1);
    }
}
