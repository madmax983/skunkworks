use super::normalize_coords;
use crate::vm::{ChimeraVM, Value};
use rand::Rng;
#[cfg(feature = "resonance")]
use resonance_audio::audio::AudioCommand;
use std::fmt;
use std::str::FromStr;

/// Represents the state of a Siren (Musical Agent).
///
/// Format: "♬:BPM:Octave:Velocity:Waveform:Direction"
/// Example: "♬:120:0:100:0:1" (BPM 120, Octave 0, Vel 100, Sine Wave, East)
#[derive(Debug, Clone)]
pub struct SirenState {
    pub bpm: u64,
    pub octave: i64,
    pub velocity: u8,
    pub waveform: u8,     // 0=Sine, 1=Square, 2=Saw, 3=Triangle, 4=Noise
    pub direction: usize, // 0=N, 1=E, 2=S, 3=W
}

impl SirenState {
    pub fn new(bpm: u64, octave: i64, velocity: u8, waveform: u8, direction: usize) -> Self {
        Self {
            bpm,
            octave,
            velocity,
            waveform,
            direction,
        }
    }

    pub fn default() -> Self {
        Self {
            bpm: 120,
            octave: 0,
            velocity: 100,
            waveform: 0,
            direction: 1, // Start moving East
        }
    }

    pub fn to_value(&self) -> Value {
        Value::Str(self.to_string())
    }
}

impl fmt::Display for SirenState {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        write!(
            f,
            "♬:{}:{}:{}:{}:{}",
            self.bpm, self.octave, self.velocity, self.waveform, self.direction
        )
    }
}

impl FromStr for SirenState {
    type Err = ();

    fn from_str(s: &str) -> Result<Self, Self::Err> {
        let parts: Vec<&str> = s.split(':').collect();
        if parts.len() >= 6 && parts[0] == "♬" {
            let bpm = parts[1].parse().unwrap_or(120);
            let octave = parts[2].parse().unwrap_or(0);
            let velocity = parts[3].parse().unwrap_or(100);
            let waveform = parts[4].parse().unwrap_or(0);
            let direction = parts[5].parse().unwrap_or(1);
            Ok(Self {
                bpm,
                octave,
                velocity,
                waveform,
                direction,
            })
        } else {
            // Try fallback if parsing fails or old format (if any)
            Ok(Self::default())
        }
    }
}

/// Processes the logic for a Siren agent.
///
/// Sirens act as Look-Ahead Sequencers.
/// They look at the cell they are about to enter, execute its command, and then move there.
/// This allows them to "consume" the music as they traverse it.
pub fn process_siren_logic(
    vm: &mut ChimeraVM,
    agent: &super::PrologueAgent,
    grid_snapshot: &[Vec<Value>],
) -> Option<(super::PrologueAgent, Option<(usize, usize)>)> {
    let (y, x) = (agent.y, agent.x);
    let mut siren_state = if let Value::Str(s) = &agent.state {
        s.parse::<SirenState>().unwrap_or(SirenState::default())
    } else {
        SirenState::default()
    };

    // 1. Determine Target Position (Look Ahead)
    let (dy, dx) = match siren_state.direction {
        0 => (-1, 0), // N
        1 => (0, 1),  // E
        2 => (1, 0),  // S
        3 => (0, -1), // W
        _ => (0, 0),
    };

    let next_pos_opt = normalize_coords(y as i64 + dy, x as i64 + dx);

    // Wrap Logic
    let (ty, tx) = if let Some(pos) = next_pos_opt {
        pos
    } else {
        match siren_state.direction {
            0 => (crate::vm::GRID_SIZE - 1, x), // N -> Bottom
            1 => (y, 0),                        // E -> Left
            2 => (0, x),                        // S -> Top
            3 => (y, crate::vm::GRID_SIZE - 1), // W -> Right
            _ => (y, x),
        }
    };

    // 2. Read Logic from Target Cell
    let target_val = &grid_snapshot[ty][tx];

    if let Value::Str(s) = target_val {
        // Interpret String as Music/Command
        for char in s.chars() {
            match char {
                'A'..='G' => {
                    let note_base = match char {
                        'C' => 0,
                        'D' => 2,
                        'E' => 4,
                        'F' => 5,
                        'G' => 7,
                        'A' => 9,
                        'B' => 11,
                        _ => 0,
                    };
                    let midi_note = 60 + note_base + (siren_state.octave * 12);
                    play_note(vm, midi_note as i32, &siren_state, tx, ty);
                }
                'a'..='g' => {
                    let note_base = match char {
                        'c' => 1,
                        'd' => 3,
                        'e' => 4,
                        'f' => 6,
                        'g' => 8,
                        'a' => 10,
                        'b' => 11,
                        _ => 0,
                    };
                    let midi_note = 60 + note_base + (siren_state.octave * 12);
                    play_note(vm, midi_note as i32, &siren_state, tx, ty);
                }
                '0'..='9' => {
                    if let Some(digit) = char.to_digit(10) {
                        siren_state.octave = digit as i64 - 5;
                    }
                }
                '^' => siren_state.octave += 1,
                'v' => siren_state.octave -= 1,
                '>' => siren_state.velocity = siren_state.velocity.saturating_add(10),
                '<' => siren_state.velocity = siren_state.velocity.saturating_sub(10),
                '!' => siren_state.velocity = 127,
                '~' => {}                         // Rest
                'k' => siren_state.direction = 0, // North
                'l' => siren_state.direction = 1, // East
                'j' => siren_state.direction = 2, // South
                'h' => siren_state.direction = 3, // West
                'N' => siren_state.direction = 0, // Legacy/Explicit
                'S' => siren_state.direction = 2, // Legacy/Explicit
                'W' => siren_state.direction = 3, // Legacy/Explicit
                // 'E' is Note E, so we use 'l' for East.
                '*' => {
                    let mut rng = rand::thread_rng();
                    siren_state.direction = rng.gen_range(0..4);
                }
                'w' => siren_state.waveform = (siren_state.waveform + 1) % 5,
                _ => {}
            }
        }
    } else if let Value::Int(n) = target_val {
        if *n > 0 && *n < 128 {
            play_note(vm, *n as i32, &siren_state, tx, ty);
        }
    }

    // 3. Move to Target (ty, tx)
    // Update agent state (which might have changed direction/octave during processing)
    let mut updated_agent = agent.clone();
    updated_agent.state = siren_state.to_value();

    Some((updated_agent, Some((ty, tx))))
}

fn play_note(vm: &mut ChimeraVM, midi_note: i32, state: &SirenState, x: usize, y: usize) {
    let freq = 440.0 * 2.0f32.powf((midi_note as f32 - 69.0) / 12.0);

    #[cfg(feature = "resonance")]
    if let Some(tx) = &vm.audio_tx {
        let _ = tx.send(AudioCommand::Tone {
            x,
            y,
            frequency: freq,
            strength: state.velocity as f32 / 127.0,
            duration_ms: (60000 / state.bpm.max(1)) as u64, // Duration based on BPM
        });
    }

    vm.output.push(format!(
        "SIREN: Note {} ({:.1}Hz) Vel {} at {},{}",
        midi_note, freq, state.velocity, x, y
    ));
}
