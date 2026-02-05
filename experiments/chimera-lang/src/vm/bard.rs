#![cfg(feature = "nova")]
use super::{ChimeraVM, Value};
use crate::ast::Nucleotide;
use crate::opcode::OpCode;

/// Represents a single musical event (Note or Rest).
#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    /// MIDI Pitch (0-127). 0 indicates a Rest.
    pub pitch: u8,
    /// Duration in 1/16th notes.
    pub duration: u8,
    /// Velocity (0-127). 0 is silent.
    pub velocity: u8,
}

impl Note {
    pub fn new(pitch: u8, duration: u8, velocity: u8) -> Self {
        Self {
            pitch,
            duration,
            velocity,
        }
    }
}

pub fn exec_bard_op(vm: &mut ChimeraVM, op: OpCode, _args: &[Nucleotide]) {
    match op {
        OpCode::Note => {
            // stack: velocity, duration, pitch (top)
            if vm.stack.len() >= 3 {
                let p_val = vm.stack.pop().unwrap();
                let d_val = vm.stack.pop().unwrap();
                let v_val = vm.stack.pop().unwrap();

                if let (Value::Int(p), Value::Int(d), Value::Int(v)) = (p_val, d_val, v_val) {
                    let pitch = p.clamp(0, 127) as u8;
                    let duration = d.clamp(1, 64) as u8; // Max duration 4 measures
                    let velocity = v.clamp(0, 127) as u8;

                    vm.score.push(Note::new(pitch, duration, velocity));
                    vm.energy = vm.energy.saturating_sub(1);
                    vm.output
                        .push(format!("NOTE: {} d={} v={}", pitch, duration, velocity));
                } else {
                    vm.output.push("Error: Type mismatch for note".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for note".to_string());
            }
        }
        OpCode::Rest => {
            // stack: duration (top)
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(d) = val {
                    let duration = d.clamp(1, 64) as u8;
                    vm.score.push(Note::new(0, duration, 0));
                    vm.energy = vm.energy.saturating_sub(1);
                    vm.output.push(format!("REST: d={}", duration));
                } else {
                    vm.output.push("Error: Type mismatch for rest".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for rest".to_string());
            }
        }
        OpCode::Tempo => {
            // stack: bpm (top)
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(bpm) = val {
                    vm.output.push(format!("TEMPO: Set to {} BPM", bpm));
                    // For now, we don't store tempo in the Note struct, but we could add a Tempo event.
                    // Since `score` is Vec<Note>, we can't easily add Tempo change unless Note has a variant.
                    // For MVP, just logging it is fine, or we can treat it as a metadata instruction.
                    // Let's just log it. The ABC exporter can just set a default or we can extend Note later.
                } else {
                    vm.output.push("Error: Type mismatch for tempo".to_string());
                }
            } else {
                vm.output
                    .push("Error: Stack underflow for tempo".to_string());
            }
        }
        OpCode::Perform => {
            let abc = score_to_abc(&vm.score);
            vm.stack.push(Value::Str(abc));
            vm.energy = vm.energy.saturating_sub(5);
            vm.output
                .push("PERFORM: Exported score to stack".to_string());
        }
        _ => {}
    }
}

pub fn score_to_abc(score: &[Note]) -> String {
    let mut s = String::from("X:1\nT:Chimera Composition\nM:4/4\nL:1/16\nK:C\n");
    let mut measure_dur = 0;

    for note in score {
        // Pitch mapping
        // 60 = C4 (Middle C) -> "C" in ABC
        // ABC: C, D, E, F, G, A, B, c, d, e, f...
        // Capital C is C3? No.
        // Standard: C = Middle C?
        // Let's use standard pitch notation:
        // C, is C2. C is C3. c is C4 (Middle C). c' is C5.
        // Wait, different standards.
        // Let's use:
        // 60 (Middle C) = "C"
        // 72 = "c"
        // 48 = "C,"

        // Actually, ABC standard:
        // C = middle C (60)
        // c = C5 (72)
        // C, = C3 (48)

        let pitch_str = if note.pitch == 0 {
            "z".to_string()
        } else {
            midi_to_abc(note.pitch)
        };

        let dur_str = if note.duration == 1 {
            String::new() // Default L:1/16
        } else {
            note.duration.to_string()
        };

        s.push_str(&format!("{}{}", pitch_str, dur_str));

        measure_dur += note.duration;
        if measure_dur >= 16 {
            s.push_str(" | ");
            measure_dur = 0;
        } else {
            s.push(' ');
        }
    }
    s
}

fn midi_to_abc(pitch: u8) -> String {
    let names = [
        "C", "^C", "D", "^D", "E", "F", "^F", "G", "^G", "A", "^A", "B",
    ];

    // Standard ABC:
    // C = C3 (48)
    // c = C4 (60, Middle C)
    // c' = C5 (72)
    // C, = C2 (36)

    let name_idx = (pitch % 12) as usize;
    let base_name = names[name_idx].to_string();

    // 60 / 12 = 5.
    // We want 60 -> "c" (lowercase, no prime).
    // So target "rel_octave" logic:
    // C3 (48) -> "C" (uppercase).
    // C4 (60) -> "c" (lowercase).

    let octave = (pitch / 12) as i32;
    // 36 -> 3. Target: "C," (Upper + comma)
    // 48 -> 4. Target: "C" (Upper)
    // 60 -> 5. Target: "c" (Lower)
    // 72 -> 6. Target: "c'" (Lower + prime)

    let mut res = base_name;

    if octave >= 5 {
        res = res.to_lowercase();
        // 60 -> 5. primes = 5 - 5 = 0.
        // 72 -> 6. primes = 6 - 5 = 1.
        for _ in 0..(octave - 5) {
            res.push('\'');
        }
    } else {
        // 48 -> 4. commas = 4 - 4 = 0.
        // 36 -> 3. commas = 4 - 3 = 1.
        for _ in 0..(4 - octave) {
            res.push(',');
        }
    }

    res
}
