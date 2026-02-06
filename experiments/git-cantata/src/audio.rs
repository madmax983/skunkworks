use crate::git::CommitData;
#[cfg(feature = "audio")]
use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
#[cfg(feature = "audio")]
use std::f32::consts::PI;
#[cfg(feature = "audio")]
use std::time::Duration;

pub struct AudioEngine {
    #[cfg(feature = "audio")]
    state: Option<AudioState>,
}

#[cfg(feature = "audio")]
struct AudioState {
    _stream: OutputStream,
    _stream_handle: OutputStreamHandle,
    sink: Sink,
}

impl AudioEngine {
    pub fn new() -> Self {
        #[cfg(feature = "audio")]
        {
            match OutputStream::try_default() {
                Ok((stream, handle)) => match Sink::try_new(&handle) {
                    Ok(sink) => Self {
                        state: Some(AudioState {
                            _stream: stream,
                            _stream_handle: handle,
                            sink,
                        }),
                    },
                    Err(_) => Self { state: None },
                },
                Err(_) => Self { state: None },
            }
        }
        #[cfg(not(feature = "audio"))]
        Self {}
    }

    pub fn play_commit(&self, _commit: &CommitData) {
        #[cfg(feature = "audio")]
        if let Some(state) = &self.state {
            if _commit.changes.is_empty() {
                return;
            }
            let source = CommitSource::new(_commit.clone());
            state.sink.append(source);
        }
    }

    pub fn is_active(&self) -> bool {
        #[cfg(feature = "audio")]
        return self.state.is_some();
        #[cfg(not(feature = "audio"))]
        return false;
    }
}

#[cfg(feature = "audio")]
struct CommitSource {
    commit: CommitData,
    current_file_idx: usize,
    samples_played_in_file: usize,
    sample_rate: u32,
    samples_per_file: usize,
    // Synth state
    phase: f32,
}

#[cfg(feature = "audio")]
impl CommitSource {
    fn new(commit: CommitData) -> Self {
        Self {
            commit,
            current_file_idx: 0,
            samples_played_in_file: 0,
            sample_rate: 48000,
            samples_per_file: 4800, // 0.1s per file for rapid fire
            phase: 0.0,
        }
    }
}

#[cfg(feature = "audio")]
impl Iterator for CommitSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.current_file_idx >= self.commit.changes.len() {
            return None;
        }

        let change = &self.commit.changes[self.current_file_idx];

        // Synth Parameters
        let base_freq = 220.0 + (self.commit.hash.as_bytes()[0] as f32); // 220..475 Hz base
        let added_freq = (change.insertions as f32).min(1000.0) * 2.0;
        let freq = base_freq + added_freq;

        let modulator = (change.deletions as f32).min(100.0) * 5.0; // FM index

        let t = self.samples_played_in_file as f32 / self.sample_rate as f32;

        // Waveform based on extension
        let sample = match change.extension.as_str() {
            "rs" => (self.phase * 2.0 * PI).sin(),
            "toml" | "json" | "yaml" => {
                if (self.phase * 2.0 * PI).sin() > 0.0 {
                    1.0
                } else {
                    -1.0
                }
            }
            "md" | "txt" => (rand::random::<f32>() * 2.0 - 1.0) * 0.5,
            _ => (self.phase * 2.0 - 1.0), // Sawtooth
        };

        // Apply Envelope (simple attack/decay)
        let progress = self.samples_played_in_file as f32 / self.samples_per_file as f32;
        let envelope = if progress < 0.1 {
            progress * 10.0
        } else {
            1.0 - ((progress - 0.1) / 0.9)
        };

        // FM Modulation
        let fm = (t * modulator).sin() * 0.5;
        self.phase += (freq * (1.0 + fm)) / self.sample_rate as f32;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }

        let output = sample * envelope * 0.5; // Master volume

        self.samples_played_in_file += 1;
        if self.samples_played_in_file >= self.samples_per_file {
            self.samples_played_in_file = 0;
            self.current_file_idx += 1;
        }

        Some(output)
    }
}

#[cfg(feature = "audio")]
impl Source for CommitSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }

    fn channels(&self) -> u16 {
        1
    }

    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    fn total_duration(&self) -> Option<Duration> {
        let total_samples = self.commit.changes.len() * self.samples_per_file;
        Some(Duration::from_secs_f32(
            total_samples as f32 / self.sample_rate as f32,
        ))
    }
}
