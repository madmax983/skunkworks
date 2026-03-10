//! # Bard Extension 🎻
//!
//! The `Bard` module enables the Chimera VM to compose music.
//! It treats the execution trace as a musical score, recording notes, rests, and tempo changes.
//!
//! The resulting score can be exported as [ABC Notation](https://abcnotation.com/), allowing
//! the organism's "song" to be played by external tools.
//!
//! With the **Composer** feature (`OpCode::Compose`), the organism can also turn its song back into DNA,
//! creating a feedback loop between music and genetics.
//!
//! ## Example
//!
//! ```
//! use chimera_lang::vm::bard::{Note, score_to_abc};
//!
//! // Simulate a simple melody (Twinkle Twinkle Little Star)
//! let score = vec![
//!     Note::new(60, 4, 100), // C4
//!     Note::new(60, 4, 100), // C4
//!     Note::new(67, 4, 100), // G4
//!     Note::new(67, 4, 100), // G4
//!     Note::new(69, 4, 100), // A4
//!     Note::new(69, 4, 100), // A4
//!     Note::new(67, 8, 100), // G4 (Half note)
//! ];
//!
//! let abc = score_to_abc(&score);
//! assert!(abc.contains("c4 c4 g4 g4 | a4 a4 g8"));
//! ```

use super::{ChimeraVM, Value};
use crate::ast::{Gene, Nucleotide, Strand};
use crate::opcode::OpCode;
#[cfg(feature = "resonance")]
use resonance_audio::audio::AudioCommand;

/// Represents a single musical event (Note or Rest).
#[derive(Debug, Clone, PartialEq)]
pub struct Note {
    /// MIDI Pitch (0-127). 0 indicates a Rest.
    ///
    /// - 60 = Middle C (C4)
    /// - 69 = A4 (440Hz)
    pub pitch: u8,
    /// Duration in 1/16th notes.
    ///
    /// - 1 = 16th note
    /// - 4 = Quarter note
    /// - 16 = Whole note
    pub duration: u8,
    /// Velocity (0-127). 0 is silent (Rest).
    ///
    /// Used for dynamic expression (pianissimo to fortissimo).
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

/// Executes a Bard-specific OpCode.
///
/// Handles musical instructions to build up the internal `vm.score`.
///
/// # Supported Ops
///
/// - `Note`: Adds a pitched note.
/// - `Rest`: Adds a silence.
/// - `Tempo`: Logs tempo change (metadata).
/// - `Perform`: Compiles the score to ABC notation string on the stack.
/// - `Compose`: Compiles the score into a DNA strand.
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

                    #[cfg(feature = "resonance")]
                    {
                        if let Some(tx) = &vm.audio_tx {
                            let freq = 440.0 * 2.0f32.powf((pitch as f32 - 69.0) / 12.0);
                            let strength = (velocity as f32) / 127.0;
                            // Duration in ms.
                            // Assuming 120 BPM.
                            // Quarter note = 60000 / 120 = 500ms.
                            // Note duration is 1/16th.
                            // Quarter note = 4 * 16th.
                            // So 1 unit = 500 / 4 = 125ms.
                            let duration_ms = duration as u64 * 125;
                            let (cy, cx) = vm.context_loc;

                            let _ = tx.send(AudioCommand::Tone {
                                x: cx,
                                y: cy,
                                frequency: freq,
                                strength,
                                duration_ms,
                            });
                        }
                    }

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
        OpCode::Compose => {
            if vm.score.is_empty() {
                vm.stack.push(Value::Int(-1));
                vm.output.push("COMPOSE: Score empty".to_string());
                return;
            }

            let mut genes = Vec::new();
            for note in &vm.score {
                if let Some(gene) = note_to_gene(note) {
                    genes.push(gene);
                }
            }

            if !genes.is_empty() {
                vm.dna.helix.strands.push(Strand { genes });
                vm.telomeres.push(50);
                #[cfg(feature = "cortex")]
                {
                    vm.activation_levels.push(0);
                    vm.synapse_map.push(Vec::new());
                }
                let new_idx = vm.dna.helix.strands.len() - 1;
                vm.stack.push(Value::Int(new_idx as i64));
                vm.output
                    .push(format!("COMPOSE: Created strand {} from song", new_idx));

                // Clear score after composition (consuming the inspiration)
                vm.score.clear();

                vm.energy = vm.energy.saturating_sub(20);
            } else {
                vm.stack.push(Value::Int(-1));
                vm.output
                    .push("COMPOSE: No valid genes produced".to_string());
            }
        }
        _ => {}
    }
}

/// Maps a musical Note to a genetic Instruction.
///
/// The mapping is based on the pitch class (Note name) relative to C.
/// Arguments (Nucleotides) are derived from Velocity and Duration.
fn note_to_gene(note: &Note) -> Option<Gene> {
    if note.pitch == 0 {
        return None; // Rest -> No Op (or maybe Nop?)
    }

    // Chromatic Scale (C = 0)
    let class = note.pitch % 12;
    // Duration used as numeric argument
    let arg_val = note.duration as i64;
    // Velocity used as secondary argument (if needed) or alternative
    let _vel_val = note.velocity as i64;

    let (op, args) = match class {
        0 => (OpCode::Push, vec![Nucleotide::Number(arg_val)]), // C
        1 => (OpCode::Dup, vec![]),                             // C#
        2 => (OpCode::Add, vec![]),                             // D
        3 => (OpCode::Sub, vec![]),                             // D#
        4 => (OpCode::Mul, vec![]),                             // E
        5 => (OpCode::Div, vec![]),                             // F
        6 => (OpCode::GRead, vec![]),                           // F#
        7 => (OpCode::GWrite, vec![]),                          // G
        8 => (OpCode::Print, vec![]),                           // G#
        9 => (OpCode::Jump, vec![Nucleotide::Number(arg_val)]), // A
        10 => (OpCode::Brz, vec![Nucleotide::Number(arg_val)]), // A#
        11 => (OpCode::Call, vec![Nucleotide::Number(arg_val)]), // B
        _ => return None,
    };

    Some(Gene { op, args })
}

/// Converts the recorded score into an ABC Notation string.
pub fn score_to_abc(score: &[Note]) -> String {
    let mut s = String::from("X:1\nT:Chimera Composition\nM:4/4\nL:1/16\nK:C\n");
    let mut measure_dur = 0;

    for note in score {
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

    let name_idx = (pitch % 12) as usize;
    let base_name = names[name_idx].to_string();

    let octave = (pitch / 12) as i32;

    let mut res = base_name;

    if octave >= 5 {
        res = res.to_lowercase();
        for _ in 0..(octave - 5) {
            res.push('\'');
        }
    } else {
        for _ in 0..(4 - octave) {
            res.push(',');
        }
    }

    res
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_midi_to_abc_conversion() {
        let cases = vec![
            (60, "c", "Middle C (C4)"),
            (48, "C", "C3"),
            (36, "C,", "C2"),
            (72, "c'", "C5"),
            (61, "^c", "C#4"),
            (59, "B", "B3"),
        ];

        for (pitch, expected_note, desc) in cases {
            let score = vec![Note::new(pitch, 1, 100)];
            let abc = score_to_abc(&score);
            assert!(
                abc.contains(expected_note),
                "Failed on {}: expected '{}' in output, got '{}'",
                desc,
                expected_note,
                abc
            );
        }
    }

    #[test]
    fn test_score_to_abc_rhythm() {
        let score = vec![
            Note::new(60, 4, 100), // C4, quarter
            Note::new(0, 4, 0),    // Rest, quarter
            Note::new(67, 8, 100), // G4, half
        ];
        let abc = score_to_abc(&score);
        assert!(abc.contains("c4 z4 g8"));
    }
}
