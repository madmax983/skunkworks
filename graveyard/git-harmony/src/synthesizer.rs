use git_associates::model::{DiffStats, LineChange};
#[cfg(feature = "audio")]
use rodio::{OutputStream, Sink, Source};
use std::collections::hash_map::DefaultHasher;
use std::hash::{Hash, Hasher};
use std::time::{Duration, Instant};

pub struct Synthesizer {
    // Audio stuff
    #[cfg(feature = "audio")]
    _stream: Option<OutputStream>,
    #[cfg(feature = "audio")]
    _stream_handle: Option<rodio::OutputStreamHandle>,

    // State
    diff: DiffStats,
    current_file_idx: usize,
    current_hunk_idx: usize,
    current_line_idx: usize,

    last_tick: Instant,
    tick_duration: Duration,

    pub active_notes: Vec<VisualNote>,
    pub status_msg: String,
}

#[derive(Debug, Clone)]
pub struct VisualNote {
    pub pitch: f32,     // Normalized 0.0 to 1.0 (relative to some max freq)
    pub color_hue: f32, // 0.0 to 360.0
    pub text: String,
    pub life: f32, // 1.0 to 0.0
    pub is_add: bool,
    pub is_remove: bool,
}

impl Synthesizer {
    pub fn new(diff: DiffStats, status_msg: String) -> Self {
        #[cfg(feature = "audio")]
        let (stream, stream_handle) = match OutputStream::try_default() {
            Ok((s, h)) => (Some(s), Some(h)),
            Err(_) => (None, None),
        };

        Self {
            #[cfg(feature = "audio")]
            _stream: stream,
            #[cfg(feature = "audio")]
            _stream_handle: stream_handle,
            diff,
            current_file_idx: 0,
            current_hunk_idx: 0,
            current_line_idx: 0,
            last_tick: Instant::now(),
            tick_duration: Duration::from_millis(100), // Tempo
            active_notes: Vec::new(),
            status_msg,
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

        // Prevent infinite loops if all files have 0 hunks
        let mut files_checked = 0;
        let total_files = self.diff.files.len();

        while files_checked <= total_files {
            if self.current_file_idx >= total_files {
                // Loop back to start
                self.current_file_idx = 0;
                self.current_hunk_idx = 0;
                self.current_line_idx = 0;
            }

            let file = &self.diff.files[self.current_file_idx];

            // If we are past the last hunk of this file, move to next file
            if self.current_hunk_idx >= file.hunks.len() {
                self.current_file_idx += 1;
                self.current_hunk_idx = 0;
                self.current_line_idx = 0;
                files_checked += 1;
                continue;
            }

            let hunk = &file.hunks[self.current_hunk_idx];
            // If we are past the last line of this hunk, move to next hunk
            if self.current_line_idx >= hunk.lines.len() {
                self.current_hunk_idx += 1;
                self.current_line_idx = 0;
                continue;
            }

            let line = hunk.lines[self.current_line_idx].clone();
            self.current_line_idx += 1;
            return Some(line);
        }

        None
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
        #[cfg(feature = "audio")]
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

#[cfg(test)]
mod tests {
    use super::*;
    use git_associates::model::{DiffStats, FileChange};

    #[test]
    fn test_next_event_infinite_loop() {
        let file_change = FileChange {
            path: "test.rs".to_string(),
            extension: "rs".to_string(),
            insertions: 0,
            deletions: 0,
            is_binary: false,
            hunks: vec![], // No hunks
        };

        let diff = DiffStats {
            files: vec![file_change],
            total_added: 0,
            total_removed: 0,
        };

        let mut synth = Synthesizer::new(diff, "Test".to_string());

        // This should return None, not recurse infinitely
        let event = synth.next_event();
        assert!(event.is_none());
    }
}
