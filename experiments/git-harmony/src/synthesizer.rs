use crate::parser::{DiffState, LineChange};
use rodio::{OutputStream, Sink, Source};
use std::time::{Duration, Instant};
use std::hash::{Hash, Hasher};
use std::collections::hash_map::DefaultHasher;

pub struct Synthesizer {
    // Audio stuff
    _stream: Option<OutputStream>,
    _stream_handle: Option<rodio::OutputStreamHandle>,

    // State
    diff: DiffState,
    current_file_idx: usize,
    current_hunk_idx: usize,
    current_line_idx: usize,

    last_tick: Instant,
    tick_duration: Duration,

    pub active_notes: Vec<VisualNote>,
}

#[derive(Debug, Clone)]
pub struct VisualNote {
    pub pitch: f32, // Normalized 0.0 to 1.0 (relative to some max freq)
    pub color_hue: f32, // 0.0 to 360.0
    pub text: String,
    pub life: f32, // 1.0 to 0.0
    pub is_add: bool,
    pub is_remove: bool,
}

impl Synthesizer {
    pub fn new(diff: DiffState) -> Self {
        let (stream, stream_handle) = match OutputStream::try_default() {
            Ok((s, h)) => (Some(s), Some(h)),
            Err(_) => (None, None),
        };

        Self {
            _stream: stream,
            _stream_handle: stream_handle,
            diff,
            current_file_idx: 0,
            current_hunk_idx: 0,
            current_line_idx: 0,
            last_tick: Instant::now(),
            tick_duration: Duration::from_millis(100), // Tempo
            active_notes: Vec::new(),
        }
    }

    pub fn tick(&mut self) {
        // Update active notes life
        self.active_notes.retain_mut(|n| {
            n.life -= 0.02; // Fade speed
            n.life > 0.0
        });

        if self.last_tick.elapsed() < self.tick_duration {
            return;
        }
        self.last_tick = Instant::now();

        // Play next line
        if let Some(event) = self.next_event() {
             self.play_event(event);
        }
    }

    fn next_event(&mut self) -> Option<LineChange> {
        if self.diff.files.is_empty() {
            return None;
        }

        if self.current_file_idx >= self.diff.files.len() {
            // Loop back to start
            self.current_file_idx = 0;
            self.current_hunk_idx = 0;
            self.current_line_idx = 0;
        }

        let file = &self.diff.files[self.current_file_idx];
        if self.current_hunk_idx >= file.hunks.len() {
             self.current_file_idx += 1;
             self.current_hunk_idx = 0;
             self.current_line_idx = 0;
             return self.next_event(); // Recurse
        }

        let hunk = &file.hunks[self.current_hunk_idx];
        if self.current_line_idx >= hunk.lines.len() {
            self.current_hunk_idx += 1;
            self.current_line_idx = 0;
            return self.next_event();
        }

        let line = hunk.lines[self.current_line_idx].clone();
        self.current_line_idx += 1;
        Some(line)
    }

    fn play_event(&mut self, event: LineChange) {
        let (base_freq, hue, text, is_add, is_remove) = match &event {
            LineChange::Added(s) => (440.0, 120.0, s.clone(), true, false), // A4, Green
            LineChange::Removed(s) => (220.0, 0.0, s.clone(), false, true), // A3, Red
            LineChange::Context(s) => (330.0, 240.0, s.clone(), false, false), // E4, Blue
        };

        // Modulate frequency based on content hash to make it "sing" the code
        let mut hasher = DefaultHasher::new();
        text.hash(&mut hasher);
        let hash = hasher.finish();
        let modifier = (hash % 12) as f32; // 12 semitones
        let freq = base_freq * 2.0_f32.powf(modifier / 12.0);

        // Add visual note
        self.active_notes.push(VisualNote {
            pitch: (freq - 200.0) / 800.0, // Rough normalization
            color_hue: hue,
            text,
            life: 1.0,
            is_add,
            is_remove,
        });

        // Play audio if available
        if let Some(handle) = &self._stream_handle {
             let duration = if is_add || is_remove {
                 Duration::from_millis(150)
             } else {
                 Duration::from_millis(50)
             };

             let source = rodio::source::SineWave::new(freq)
                .take_duration(duration)
                .amplify(0.1);

             if let Ok(sink) = Sink::try_new(handle) {
                 sink.append(source);
                 sink.detach();
             }
        }
    }
}
