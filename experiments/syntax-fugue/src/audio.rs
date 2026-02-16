use anyhow::Result;
use rodio::{source::SineWave, OutputStream, OutputStreamHandle, Sink, Source};
use std::time::Duration;

pub struct AudioEngine {
    _stream: Option<OutputStream>,
    stream_handle: Option<OutputStreamHandle>,
    sinks: Vec<Sink>,
}

impl AudioEngine {
    pub fn new() -> Result<Self> {
        let res = OutputStream::try_default();
        match res {
            Ok((stream, handle)) => Ok(Self {
                _stream: Some(stream),
                stream_handle: Some(handle),
                sinks: Vec::new(),
            }),
            Err(e) => {
                eprintln!(
                    "Warning: Audio initialization failed: {}. Running in silent mode.",
                    e
                );
                Ok(Self {
                    _stream: None,
                    stream_handle: None,
                    sinks: Vec::new(),
                })
            }
        }
    }

    pub fn ensure_voice_capacity(&mut self, capacity: usize) {
        if self.stream_handle.is_none() {
            return;
        }
        let handle = self.stream_handle.as_ref().unwrap();

        while self.sinks.len() < capacity {
            if let Ok(sink) = Sink::try_new(handle) {
                self.sinks.push(sink);
            } else {
                eprintln!("Failed to create audio sink");
                break;
            }
        }
    }

    pub fn play_note(&self, voice_idx: usize, frequency: f32, duration: f32, volume: f32) {
        if self.stream_handle.is_none() {
            return;
        }

        if let Some(sink) = self.sinks.get(voice_idx) {
            let source = SineWave::new(frequency)
                .take_duration(Duration::from_secs_f32(duration))
                .amplify(volume);

            // Append always appends to the queue.
            sink.append(source);
            // Sinks play automatically if not paused.
        }
    }

    pub fn is_voice_busy(&self, voice_idx: usize) -> bool {
        if let Some(sink) = self.sinks.get(voice_idx) {
            !sink.empty()
        } else {
            false
        }
    }
}
