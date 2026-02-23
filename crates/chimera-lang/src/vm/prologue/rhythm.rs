use crate::vm::Value;
#[cfg(feature = "resonance")]
use crossbeam_channel::Sender;
#[cfg(feature = "resonance")]
use resonance_audio::audio::AudioCommand;
use serde::{Deserialize, Serialize};

#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct RhythmState {
    pub bpm: u64,
    pub ticks: u64,
    pub beat_interval: u64,
    pub last_beat_tick: u64,
}

impl Default for RhythmState {
    fn default() -> Self {
        Self {
            bpm: 120,
            ticks: 0,
            beat_interval: 5, // Default ~120 BPM at 10Hz tick rate
            last_beat_tick: 0,
        }
    }
}

impl RhythmState {
    pub fn new() -> Self {
        Self::default()
    }
}

#[allow(clippy::too_many_arguments)]
pub fn apply_rhythm_runes(
    rune: &str,
    y: usize,
    x: usize,
    tick: u64,
    current_signals: &[Vec<Option<Value>>],
    next_signals: &mut Vec<Vec<Option<Value>>>,
    rhythm_state: &mut RhythmState,
    #[cfg(feature = "resonance")] audio_tx: &Option<Sender<AudioCommand>>,
    #[cfg(not(feature = "resonance"))] _audio_tx: &Option<()>,
    output: &mut Vec<String>,
) -> bool {
    let mut change = false;
    match rune {
        "⏱️" => {
            // Clock
            // Emits signal on beat
            let interval = rhythm_state.beat_interval;
            if interval > 0 && tick % interval == 0 {
                next_signals[y][x] = Some(Value::Int(1));
                change = true;
            }
        }
        "🥁" => {
            // Drum
            // Reads West. If signal, Play Drum.
            if let Some((wy, wx)) = super::normalize_coords(y as i64, x as i64 - 1) {
                if current_signals[wy][wx].is_some() {
                    #[cfg(feature = "resonance")]
                    if let Some(tx) = audio_tx {
                        let _ = tx.send(AudioCommand::Tone {
                            x,
                            y,
                            frequency: 60.0, // Low freq for kick
                            strength: 1.0,
                            duration_ms: 100,
                        });
                    }
                    output.push("RHYTHM: BOOM!".to_string());
                    next_signals[y][x] = Some(Value::Int(1)); // Light up
                    change = true;
                }
            }
        }
        "🎹" => {
            // Keys
            // Reads West (Note)
            if let Some((wy, wx)) = super::normalize_coords(y as i64, x as i64 - 1) {
                if let Some(val) = &current_signals[wy][wx] {
                    // Parse note
                    let freq = match val {
                        Value::Int(n) => *n as f32,
                        _ => 60.0, // Middle C
                    };
                    #[cfg(feature = "resonance")]
                    if let Some(tx) = audio_tx {
                        let hz = 440.0 * 2.0f32.powf((freq - 69.0) / 12.0);
                        let _ = tx.send(AudioCommand::Tone {
                            x,
                            y,
                            frequency: hz,
                            strength: 0.8,
                            duration_ms: 200,
                        });
                    }
                    next_signals[y][x] = Some(val.clone());
                    change = true;
                }
            }
        }
        "🎚️" => {
            // Fader
            // Reads West (Value) -> Sets BPM
            if let Some((wy, wx)) = super::normalize_coords(y as i64, x as i64 - 1) {
                if let Some(Value::Int(n)) = &current_signals[wy][wx] {
                    if *n > 0 {
                        rhythm_state.bpm = *n as u64;
                        // Approx conversion: Tick = 100ms (10Hz).
                        // BPM 60 = 1 beat/sec = 10 ticks.
                        // Interval = (600 / BPM)
                        rhythm_state.beat_interval = (600 / *n).max(1) as u64;
                    }
                }
            }
        }
        _ => {}
    }
    change
}
