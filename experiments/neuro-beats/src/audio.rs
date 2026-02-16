use rodio::{source::Source, OutputStream, OutputStreamHandle};
use std::time::Duration;

pub struct AudioEngine {
    _stream: Option<OutputStream>,
    stream_handle: Option<OutputStreamHandle>,
}

impl AudioEngine {
    pub fn new() -> Self {
        // Try to get the default output stream. If it fails (e.g. no audio device), just continue without audio.
        match OutputStream::try_default() {
            Ok((stream, stream_handle)) => Self {
                _stream: Some(stream),
                stream_handle: Some(stream_handle),
            },
            Err(e) => {
                eprintln!("Audio initialization failed: {}", e);
                Self {
                    _stream: None,
                    stream_handle: None,
                }
            }
        }
    }

    pub fn play_tone(&self, frequency: f32, duration_secs: f32) {
        if let Some(ref handle) = self.stream_handle {
            let source = rodio::source::SineWave::new(frequency)
                .take_duration(Duration::from_secs_f32(duration_secs))
                .amplify(0.10); // Lower volume to avoid clipping with many neurons

            // Fire and forget
            match handle.play_raw(source.convert_samples()) {
                Ok(_) => {}
                Err(e) => eprintln!("Failed to play sound: {}", e),
            }
        }
    }
}
