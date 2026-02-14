#![cfg(feature = "nova")]
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
/// - `Notate`: Compiles a DNA strand into a score.
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
        OpCode::Notate => {
            // stack: strand_idx (top)
            if let Some(val) = vm.stack.pop() {
                if let Value::Int(idx) = val {
                    let s_idx = idx as usize;
                    if s_idx < vm.dna.helix.strands.len() {
                        let strand = &vm.dna.helix.strands[s_idx];
                        for gene in &strand.genes {
                            if let Some(note) = gene_to_note(gene) {
                                vm.score.push(note);
                            }
                        }
                        vm.energy = vm.energy.saturating_sub(10);
                        vm.output.push(format!("NOTATE: Transcribed strand {}", s_idx));
                    } else {
                        vm.output.push("NOTATE: Invalid strand index".to_string());
                    }
                } else {
                    vm.output.push("NOTATE: Type mismatch".to_string());
                }
            } else {
                vm.output.push("NOTATE: Stack underflow".to_string());
            }
        }
        _ => {}
    }
}

/// Helper function for TUI to get OpCode name for a pitch.
pub fn get_opcode_name_for_pitch(pitch: u8) -> &'static str {
    if pitch == 0 {
        return "Rest";
    }
    let octave = pitch / 12;
    let class = pitch % 12;

    match (octave, class) {
        // Octave 3 (Arithmetic & Logic)
        (3, 0) => "Push",
        (3, 1) => "Pop/Drop",
        (3, 2) => "Add",
        (3, 3) => "Sub",
        (3, 4) => "Mul",
        (3, 5) => "Div",
        (3, 6) => "Mod", // Assuming Mod might be implemented or mapped
        (3, 7) => "Eq",  // Assuming Eq
        (3, 8) => "Gt",  // Assuming Gt
        (3, 9) => "Lt",  // Assuming Lt
        (3, 10) => "Not", // Assuming Not
        (3, 11) => "Swap",

        // Octave 4 (Control Flow & IO)
        (4, 0) => "Print",
        (4, 1) => "Scan", // Assuming Input/Scan
        (4, 2) => "Jump",
        (4, 3) => "Brz",
        (4, 4) => "Call",
        (4, 5) => "Ret",
        (4, 6) => "Signal",
        (4, 7) => "Receive",
        (4, 8) => "Broadcast",
        (4, 9) => "Tune",
        (4, 10) => "Sleep", // Assuming Wait/Sleep
        (4, 11) => "Halt",

        // Octave 5 (Biology)
        (5, 0) => "Mitosis",
        (5, 1) => "Apoptosis",
        (5, 2) => "Spawn",
        (5, 3) => "Differentiate",
        (5, 4) => "Photosynthesize",
        (5, 5) => "Consume",
        (5, 6) => "Secrete",
        (5, 7) => "Absorb",
        (5, 8) => "Detect",
        (5, 9) => "Chemotaxis",
        (5, 10) => "Migrate",
        (5, 11) => "Incubate",

        // Octave 6 (Grid & Physics)
        (6, 0) => "GRead",
        (6, 1) => "GWrite",
        (6, 2) => "Lumine",
        (6, 3) => "SenseLight",
        (6, 4) => "Gravitate",
        (6, 5) => "Rift",
        (6, 6) => "Seal",
        (6, 7) => "Membrane",
        (6, 8) => "Osmosis",
        (6, 9) => "TimeWarp",
        (6, 10) => "Chronostasis",
        (6, 11) => "Entropy",

        _ => "Unknown",
    }
}

/// Maps a musical Note to a genetic Instruction.
fn note_to_gene(note: &Note) -> Option<Gene> {
    if note.pitch == 0 {
        return None;
    }

    let octave = note.pitch / 12;
    let class = note.pitch % 12;
    let arg_val = note.duration as i64;

    let (op, args) = match (octave, class) {
        // Octave 3: Arithmetic & Logic
        (3, 0) => (OpCode::Push, vec![Nucleotide::Number(arg_val)]), // Use duration as value
        (3, 1) => (OpCode::Drop, vec![]),
        (3, 2) => (OpCode::Add, vec![]),
        (3, 3) => (OpCode::Sub, vec![]),
        (3, 4) => (OpCode::Mul, vec![]),
        (3, 5) => (OpCode::Div, vec![]),
        (3, 6) => (OpCode::Poly, vec![Nucleotide::String("mod".to_string()), Nucleotide::String("mod".to_string())]), // Hack for Mod
        (3, 7) => (OpCode::Poly, vec![Nucleotide::String("eq".to_string()), Nucleotide::String("eq".to_string())]), // Hack for Eq
        (3, 8) => (OpCode::Poly, vec![Nucleotide::String("gt".to_string()), Nucleotide::String("gt".to_string())]), // Hack for Gt
        (3, 9) => (OpCode::Poly, vec![Nucleotide::String("lt".to_string()), Nucleotide::String("lt".to_string())]), // Hack for Lt
        (3, 10) => (OpCode::Poly, vec![Nucleotide::String("not".to_string()), Nucleotide::String("not".to_string())]), // Hack for Not
        (3, 11) => (OpCode::Swap, vec![]),

        // Octave 4: Control Flow & IO
        (4, 0) => (OpCode::Print, vec![]),
        (4, 1) => (OpCode::Spirit, vec![]), // Input
        (4, 2) => (OpCode::Jump, vec![Nucleotide::Number(arg_val)]),
        (4, 3) => (OpCode::Brz, vec![Nucleotide::Number(arg_val)]),
        (4, 4) => (OpCode::Call, vec![Nucleotide::Number(arg_val)]),
        (4, 5) => (OpCode::Ret, vec![]),
        (4, 6) => (OpCode::Signal, vec![Nucleotide::Number(0), Nucleotide::Number(arg_val)]), // Channel 0 default
        (4, 7) => (OpCode::Receive, vec![Nucleotide::Number(0)]),
        (4, 8) => (OpCode::Broadcast, vec![Nucleotide::Number(0), Nucleotide::Number(arg_val)]),
        (4, 9) => (OpCode::Tune, vec![Nucleotide::Number(0)]),
        (4, 10) => (OpCode::Rest, vec![Nucleotide::Number(arg_val)]), // Sleep/Rest
        (4, 11) => (OpCode::Apoptosis, vec![Nucleotide::Number(0)]), // Halt/Die (Self)

        // Octave 5: Biology
        (5, 0) => (OpCode::Mitosis, vec![Nucleotide::Number(0)]), // Clone self
        (5, 1) => (OpCode::Apoptosis, vec![Nucleotide::Number(0)]),
        (5, 2) => (OpCode::Spawn, vec![Nucleotide::Number(0), Nucleotide::Number(1)]), // Default Worker
        (5, 3) => (OpCode::Differentiate, vec![Nucleotide::Number(1)]), // Default
        (5, 4) => (OpCode::Photosynthesize, vec![]),
        (5, 5) => (OpCode::Consume, vec![]),
        (5, 6) => (OpCode::Secrete, vec![Nucleotide::Number(0), Nucleotide::Number(arg_val)]),
        (5, 7) => (OpCode::Absorb, vec![Nucleotide::Number(0), Nucleotide::Number(arg_val)]),
        (5, 8) => (OpCode::Detect, vec![Nucleotide::Number(0)]),
        (5, 9) => (OpCode::Chemotaxis, vec![Nucleotide::Number(0)]),
        (5, 10) => (OpCode::Migrate, vec![Nucleotide::Number(1), Nucleotide::Number(0)]), // Default Move
        (5, 11) => (OpCode::Incubate, vec![Nucleotide::Number(arg_val), Nucleotide::Number(0), Nucleotide::Number(0)]),

        // Octave 6: Grid & Physics
        (6, 0) => (OpCode::GRead, vec![]),
        (6, 1) => (OpCode::GWrite, vec![]),
        (6, 2) => (OpCode::Lumine, vec![Nucleotide::Number(arg_val), Nucleotide::Number(5)]),
        (6, 3) => (OpCode::SenseLight, vec![]),
        (6, 4) => (OpCode::Gravitate, vec![Nucleotide::Number(arg_val)]),
        (6, 5) => (OpCode::Rift, vec![Nucleotide::Number(0), Nucleotide::Number(0), Nucleotide::Number(0), Nucleotide::Number(0)]),
        (6, 6) => (OpCode::Seal, vec![Nucleotide::Number(0), Nucleotide::Number(0)]),
        (6, 7) => (OpCode::Membrane, vec![Nucleotide::Number(15)]), // All directions
        (6, 8) => (OpCode::Osmosis, vec![Nucleotide::Number(1), Nucleotide::Number(0)]),
        (6, 9) => (OpCode::TimeWarp, vec![Nucleotide::Number(2), Nucleotide::Number(5)]),
        (6, 10) => (OpCode::Chronostasis, vec![Nucleotide::Number(arg_val)]),
        (6, 11) => (OpCode::Entropy, vec![]),

        _ => return None,
    };

    Some(Gene { op, args })
}

/// Maps a genetic Instruction to a musical Note.
fn gene_to_note(gene: &Gene) -> Option<Note> {
    let (octave, class, duration) = match gene.op {
        OpCode::Push => {
            let dur = if let Some(Nucleotide::Number(n)) = gene.args.first() {
                (*n).clamp(1, 64) as u8
            } else {
                4
            };
            (3, 0, dur)
        }
        OpCode::Drop => (3, 1, 4),
        OpCode::Add => (3, 2, 4),
        OpCode::Sub => (3, 3, 4),
        OpCode::Mul => (3, 4, 4),
        OpCode::Div => (3, 5, 4),
        OpCode::Swap => (3, 11, 4),

        OpCode::Print => (4, 0, 4),
        OpCode::Spirit => (4, 1, 4),
        OpCode::Jump => {
             let dur = if let Some(Nucleotide::Number(n)) = gene.args.first() {
                (*n).clamp(1, 64) as u8
            } else {
                4
            };
            (4, 2, dur)
        }
        OpCode::Brz => {
             let dur = if let Some(Nucleotide::Number(n)) = gene.args.first() {
                (*n).clamp(1, 64) as u8
            } else {
                4
            };
            (4, 3, dur)
        }
        OpCode::Call => {
             let dur = if let Some(Nucleotide::Number(n)) = gene.args.first() {
                (*n).clamp(1, 64) as u8
            } else {
                4
            };
            (4, 4, dur)
        }
        OpCode::Ret => (4, 5, 4),
        OpCode::Signal => (4, 6, 4),
        OpCode::Receive => (4, 7, 4),
        OpCode::Broadcast => (4, 8, 4),
        OpCode::Tune => (4, 9, 4),

        OpCode::Mitosis => (5, 0, 4),
        OpCode::Apoptosis => (5, 1, 4),
        OpCode::Spawn => (5, 2, 4),
        OpCode::Differentiate => (5, 3, 4),
        OpCode::Photosynthesize => (5, 4, 4),
        OpCode::Consume => (5, 5, 4),
        OpCode::Secrete => (5, 6, 4),
        OpCode::Absorb => (5, 7, 4),
        OpCode::Detect => (5, 8, 4),
        OpCode::Chemotaxis => (5, 9, 4),
        OpCode::Migrate => (5, 10, 4),
        OpCode::Incubate => (5, 11, 4),

        OpCode::GRead => (6, 0, 4),
        OpCode::GWrite => (6, 1, 4),
        OpCode::Lumine => (6, 2, 4),
        OpCode::SenseLight => (6, 3, 4),
        OpCode::Gravitate => (6, 4, 4),
        OpCode::Rift => (6, 5, 4),
        OpCode::Seal => (6, 6, 4),
        OpCode::Membrane => (6, 7, 4),
        OpCode::Osmosis => (6, 8, 4),
        OpCode::TimeWarp => (6, 9, 4),
        OpCode::Chronostasis => (6, 10, 4),
        OpCode::Entropy => (6, 11, 4),

        OpCode::Poly => {
            // Check args for specific operators
            if let Some(Nucleotide::String(s)) = gene.args.first() {
                match s.as_str() {
                    "mod" => (3, 6, 4),
                    "eq" => (3, 7, 4),
                    "gt" => (3, 8, 4),
                    "lt" => (3, 9, 4),
                    "not" => (3, 10, 4),
                    _ => return None,
                }
            } else {
                return None;
            }
        }

        _ => return None,
    };

    let pitch = (octave * 12 + class) as u8;
    Some(Note::new(pitch, duration, 100))
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
