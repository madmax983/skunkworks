use std::sync::Arc;

#[derive(Clone, Debug)]
pub struct Token {
    pub text: String,
    pub pitch: f32,    // Hz
    pub duration: f32, // Relative beats (usually 1.0)
    pub velocity: f32, // 0.0 - 1.0
}

#[derive(Clone, Debug)]
pub struct TrackState {
    pub tokens: Arc<Vec<Token>>,
    pub current_index: usize,
    pub name: String,
}

pub struct Track {
    pub tokens: Arc<Vec<Token>>,
    pub current_index: usize,
    pub phase: f32, // 0.0 to 1.0 within the current token
    pub speed_multiplier: f32,
    pub name: String,
}

impl Track {
    pub fn new(name: &str, content: &str) -> Self {
        let tokens = parse_content(content);
        // Ensure at least one token to avoid panics
        let tokens = if tokens.is_empty() {
            vec![Token {
                text: "<empty>".to_string(),
                pitch: 440.0,
                duration: 1.0,
                velocity: 0.0,
            }]
        } else {
            tokens
        };

        Self {
            tokens: Arc::new(tokens),
            current_index: 0,
            phase: 0.0,
            speed_multiplier: 1.0,
            name: name.to_string(),
        }
    }

    // Returns Some((frequency, velocity)) if a new note triggered
    pub fn tick(&mut self, dt_secs: f32, bpm: f32) -> Option<(f32, f32)> {
        let beat_duration = 60.0 / bpm;
        let token_duration = self.tokens[self.current_index].duration;

        let increment = (dt_secs / (beat_duration * token_duration)) * self.speed_multiplier;

        self.phase += increment;

        if self.phase >= 1.0 {
            self.phase -= 1.0;
            self.current_index = (self.current_index + 1) % self.tokens.len();
            let token = &self.tokens[self.current_index];
            return Some((token.pitch, token.velocity));
        }
        None
    }

    pub fn get_state(&self) -> TrackState {
        TrackState {
            tokens: self.tokens.clone(),
            current_index: self.current_index,
            name: self.name.clone(),
        }
    }
}

fn parse_content(content: &str) -> Vec<Token> {
    content.split_whitespace().map(|s| {
        let len = s.len();
        let scale_degrees = [0, 3, 5, 7, 10, 12, 15, 17, 19, 22, 24];
        let degree_idx = len % scale_degrees.len();
        let degree = scale_degrees[degree_idx];

        let pitch = if len < 4 {
             880.0 * 2.0f32.powf(degree as f32 / 12.0)
        } else {
             220.0 * 2.0f32.powf((degree as f32) / 12.0)
        };

        let velocity = if s.starts_with("//") {
            0.2
        } else if ["fn", "pub", "impl", "struct", "let", "mut"].contains(&s) {
            0.9
        } else {
            0.6
        };

        Token {
            text: s.to_string(),
            pitch,
            duration: 1.0,
            velocity,
        }
    }).collect()
}

pub struct RenderState {
    pub track1: TrackState,
    pub track2: TrackState,
    pub bpm: f32,
    pub drift: f32,
}

pub struct Engine {
    pub track1: Track,
    pub track2: Track,
    pub bpm: f32,
}

impl Engine {
    pub fn new(content1: &str, content2: &str) -> Self {
        Self {
            track1: Track::new("Voice 1", content1),
            track2: Track::new("Voice 2", content2),
            bpm: 120.0 * 4.0, // 16th notes
        }
    }

    pub fn set_drift(&mut self, drift: f32) {
        self.track2.speed_multiplier = 1.0 + drift;
    }

    pub fn get_render_state(&self) -> RenderState {
        RenderState {
            track1: self.track1.get_state(),
            track2: self.track2.get_state(),
            bpm: self.bpm,
            drift: self.track2.speed_multiplier - 1.0,
        }
    }
}
