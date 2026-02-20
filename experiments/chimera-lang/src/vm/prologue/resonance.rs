use crate::vm::{ChimeraVM, Value};
#[cfg(feature = "resonance")]
use resonance_audio::audio::AudioCommand;

/// Applies resonance logic for sink runes (♪, ♫, 🥁).
pub fn apply_resonance_sinks(vm: &mut ChimeraVM, rune: &str, y: usize, x: usize) {
    // Only proceed if there is a signal from the West
    let w_sig = if let Some((wy, wx)) = super::normalize_coords(y as i64, x as i64 - 1) {
        vm.prologue_state.signal_grid[wy][wx].clone()
    } else {
        None
    };

    match rune {
        "♪" => {
            if let Some(Value::Int(note)) = w_sig {
                vm.prologue_state.signal_grid[y][x] = Some(Value::Int(note)); // Light up

                #[cfg(feature = "resonance")]
                if let Some(tx) = &vm.audio_tx {
                    // MIDI to Frequency: f = 440 * 2^((d - 69) / 12)
                    // Clamp note to reasonable range 0-127
                    let n = note.clamp(0, 127) as f32;
                    let freq = 440.0 * 2.0f32.powf((n - 69.0) / 12.0);

                    let _ = tx.send(AudioCommand::Tone {
                        x,
                        y,
                        frequency: freq,
                        strength: 0.8,
                        duration_ms: 200,
                    });

                    vm.output.push(format!("RESONANCE: Note {} ({:.1}Hz) at {},{}", note, freq, x, y));
                } else {
                    vm.output.push(format!("RESONANCE: Note {} (No Audio) at {},{}", note, x, y));
                }

                #[cfg(not(feature = "resonance"))]
                {
                    vm.output.push(format!("RESONANCE: Note {} (No Audio Feature) at {},{}", note, x, y));
                }
            }
        }
        "♫" => {
            // Chord: Root (West), Type (North)
            if let Some(Value::Int(root)) = w_sig {
                 let type_sig = if let Some((ny, nx)) = super::normalize_coords(y as i64 - 1, x as i64) {
                    vm.prologue_state.signal_grid[ny][nx].clone()
                } else {
                    None
                };

                let chord_type = if let Some(Value::Int(t)) = type_sig { t } else { 0 }; // Default Major

                vm.prologue_state.signal_grid[y][x] = Some(Value::Int(root));

                #[cfg(feature = "resonance")]
                if let Some(tx) = &vm.audio_tx {
                    let offsets = match chord_type {
                        1 => vec![0, 3, 7], // Minor
                        2 => vec![0, 4, 7, 11], // Maj7
                        3 => vec![0, 3, 7, 10], // Min7
                        _ => vec![0, 4, 7], // Major
                    };

                    for off in offsets {
                        let n = (root + off).clamp(0, 127) as f32;
                        let freq = 440.0 * 2.0f32.powf((n - 69.0) / 12.0);
                        let _ = tx.send(AudioCommand::Tone {
                            x,
                            y,
                            frequency: freq,
                            strength: 0.5,
                            duration_ms: 500,
                        });
                    }
                    vm.output.push(format!("RESONANCE: Chord {} Type {} at {},{}", root, chord_type, x, y));
                } else {
                     vm.output.push(format!("RESONANCE: Chord {} Type {} (No Audio) at {},{}", root, chord_type, x, y));
                }

                #[cfg(not(feature = "resonance"))]
                {
                    vm.output.push(format!("RESONANCE: Chord {} Type {} (No Audio Feature) at {},{}", root, chord_type, x, y));
                }
            }
        }
        "🥁" => {
             if let Some(Value::Int(trig)) = w_sig {
                if trig > 0 {
                    vm.prologue_state.signal_grid[y][x] = Some(Value::Int(trig));

                    #[cfg(feature = "resonance")]
                    if let Some(tx) = &vm.audio_tx {
                         // Simple pluck
                         let _ = tx.send(AudioCommand::Pluck {
                            x,
                            y,
                            strength: 1.0,
                        });
                        vm.output.push(format!("RESONANCE: Drum at {},{}", x, y));
                    } else {
                        vm.output.push(format!("RESONANCE: Drum (No Audio) at {},{}", x, y));
                    }

                    #[cfg(not(feature = "resonance"))]
                    {
                        vm.output.push(format!("RESONANCE: Drum (No Audio Feature) at {},{}", x, y));
                    }
                }
             }
        }
        _ => {}
    }
}
