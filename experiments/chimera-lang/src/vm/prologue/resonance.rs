use crate::vm::ChimeraVM;
use crate::vm::Value;
use super::normalize_coords;

#[cfg(feature = "resonance")]
use resonance_audio::audio::AudioCommand;

/// Applies resonance runes (Audio Sinks).
///
/// * `♪` (Note): Reads West (Value) -> Plays Frequency (MIDI).
/// * `♫` (Chord): Reads West (Root), North (Type) -> Plays Chord.
/// * `🥁` (Drum): Reads West (Trigger) -> Plays Drum.
pub fn apply_resonance_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    match rune {
        "♪" => {
            // Note: West (Value) -> Frequency
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(val) = &vm.prologue_state.signal_grid[wy][wx] {
                    if let Value::Int(n) = val {
                        let note = *n;
                        let freq = midi_to_freq(note);

                        #[cfg(feature = "resonance")]
                        if let Some(tx) = &vm.audio_tx {
                            let _ = tx.send(AudioCommand::Oscillate {
                                x,
                                y,
                                frequency: freq,
                                strength: 0.5,
                            });
                        }

                        vm.output.push(format!("RESONANCE: Note {} ({:.2}Hz) at {},{}", note, freq, x, y));
                        vm.prologue_state.signal_grid[y][x] = Some(val.clone()); // Light up
                    }
                }
            }
        }
        "♫" => {
            // Chord: West (Root), North (Type: 0=Maj, 1=Min)
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(root_val) = &vm.prologue_state.signal_grid[wy][wx] {
                    if let Value::Int(root) = root_val {
                         let chord_type = if let Some((ny, nx)) = normalize_coords(y as i64 - 1, x as i64) {
                            if let Some(Value::Int(t)) = &vm.prologue_state.signal_grid[ny][nx] {
                                *t
                            } else {
                                0 // Default Major
                            }
                        } else {
                            0
                        };

                        let offsets = if chord_type == 1 {
                            vec![0, 3, 7] // Minor
                        } else {
                            vec![0, 4, 7] // Major
                        };

                        for offset in offsets {
                            let note = root + offset;
                            let _freq = midi_to_freq(note);

                            #[cfg(feature = "resonance")]
                            if let Some(tx) = &vm.audio_tx {
                                let _ = tx.send(AudioCommand::Oscillate {
                                    x,
                                    y,
                                    frequency: _freq,
                                    strength: 0.3, // Lower volume for chord
                                });
                            }
                        }

                        vm.output.push(format!("RESONANCE: Chord {} (Type {}) at {},{}", root, chord_type, x, y));
                        vm.prologue_state.signal_grid[y][x] = Some(root_val.clone());
                    }
                }
            }
        }
        "🥁" => {
            // Drum: West (Trigger)
            if let Some((wy, wx)) = normalize_coords(y as i64, x as i64 - 1) {
                if let Some(val) = &vm.prologue_state.signal_grid[wy][wx] {
                    // Trigger sound
                    #[cfg(feature = "resonance")]
                    if let Some(tx) = &vm.audio_tx {
                         // Simple noise burst for now via Pluck (high tension)
                        let _ = tx.send(AudioCommand::Pluck {
                            x,
                            y,
                            strength: 0.8,
                        });
                    }
                    vm.output.push(format!("RESONANCE: Drum at {},{}", x, y));
                    vm.prologue_state.signal_grid[y][x] = Some(val.clone());
                }
            }
        }
        _ => {}
    }
}

fn midi_to_freq(note: i64) -> f32 {
    440.0 * 2.0f32.powf((note as f32 - 69.0) / 12.0)
}
