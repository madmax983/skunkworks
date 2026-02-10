use std::fmt;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Mode {
    Ionian,
    Dorian,
    Phrygian,
    Lydian,
    Mixolydian,
    Aeolian,
    Locrian,
}

impl Mode {
    pub fn intervals(&self) -> &'static [u8] {
        match self {
            Mode::Ionian => &[2, 2, 1, 2, 2, 2, 1],
            Mode::Dorian => &[2, 1, 2, 2, 2, 1, 2],
            Mode::Phrygian => &[1, 2, 2, 2, 1, 2, 2],
            Mode::Lydian => &[2, 2, 2, 1, 2, 2, 1],
            Mode::Mixolydian => &[2, 2, 1, 2, 2, 1, 2],
            Mode::Aeolian => &[2, 1, 2, 2, 1, 2, 2],
            Mode::Locrian => &[1, 2, 2, 1, 2, 2, 2],
        }
    }
}

#[derive(Debug, Clone)]
pub struct Note {
    pub pitch: u8,    // MIDI note
    pub velocity: u8, // 0-127
    pub duration_ms: u64,
}

#[derive(Debug, Clone)]
pub enum FugueEvent {
    SubjectEntry {
        name: String,
        voice_id: usize,
        notes: Vec<Note>,
    },
    CounterSubject {
        voice_id: usize,
        notes: Vec<Note>,
    },
    Episode {
        intensity: f32,
    },
    Ostinato {
        voice_id: usize,
        pattern: Vec<Note>,
    },
    Modulation {
        to_mode: Mode,
    },
    Cadence,
    Silence {
        duration_ms: u64,
    },
}

impl fmt::Display for FugueEvent {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        match self {
            FugueEvent::SubjectEntry { name, voice_id, .. } => {
                write!(f, "Subject Entry [{}]: Voice {}", name, voice_id)
            }
            FugueEvent::CounterSubject { voice_id, .. } => {
                write!(f, "Counter Subject: Voice {}", voice_id)
            }
            FugueEvent::Episode { intensity } => write!(f, "Episode (Intensity: {:.2})", intensity),
            FugueEvent::Ostinato { voice_id, .. } => write!(f, "Ostinato: Voice {}", voice_id),
            FugueEvent::Modulation { to_mode } => write!(f, "Modulation to {:?}", to_mode),
            FugueEvent::Cadence => write!(f, "Cadence"),
            FugueEvent::Silence { duration_ms } => write!(f, "Silence ({}ms)", duration_ms),
        }
    }
}
