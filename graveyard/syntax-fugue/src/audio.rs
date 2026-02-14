use crate::music::FugueEvent;
use anyhow::Result;
use rodio::{OutputStream, Sink, Source};
use std::time::Duration;

pub struct Synthesizer {
    _stream: OutputStream,
    _stream_handle: rodio::OutputStreamHandle,
    sinks: Vec<Sink>,
}

impl Synthesizer {
    pub fn new() -> Result<Self> {
        let (_stream, stream_handle) = OutputStream::try_default()?;
        let mut sinks = Vec::new();
        // Create 8 voices
        for _ in 0..8 {
            let sink = Sink::try_new(&stream_handle)?;
            sinks.push(sink);
        }

        Ok(Self {
            _stream,
            _stream_handle: stream_handle,
            sinks,
        })
    }

    // Non-blocking: queues notes to the appropriate sink
    pub fn play_event(&self, event: &FugueEvent) {
        match event {
            FugueEvent::SubjectEntry {
                voice_id, notes, ..
            }
            | FugueEvent::CounterSubject { voice_id, notes }
            | FugueEvent::Ostinato {
                voice_id,
                pattern: notes,
            } => {
                let voice_idx = voice_id % self.sinks.len();
                let sink = &self.sinks[voice_idx];

                for note in notes {
                    let freq = 440.0 * 2.0f32.powf((note.pitch as f32 - 69.0) / 12.0);
                    let source = rodio::source::SineWave::new(freq)
                        .take_duration(Duration::from_millis(note.duration_ms))
                        .amplify(note.velocity as f32 / 127.0 * 0.2); // 0.2 master volume

                    sink.append(source);
                }
            }
            FugueEvent::Modulation { .. } => {
                // Could change a global tuning parameter here if we wanted
            }
            _ => {}
        }
    }

    pub fn stop_all(&self) {
        for sink in &self.sinks {
            sink.stop();
        }
    }
}
