use rodio::{OutputStream, OutputStreamHandle, Sink, Source};
use std::time::Duration;

pub struct AudioEngine {
    _stream: OutputStream,
    stream_handle: OutputStreamHandle,
}

impl AudioEngine {
    pub fn new() -> Self {
        // Handle potential failure gracefully (e.g. no audio device)
        // But for this experiment, we assume audio is available or we crash/log.
        let (_stream, stream_handle) = OutputStream::try_default().expect("Failed to initialize audio output stream");
        Self {
            _stream,
            stream_handle,
        }
    }

    pub fn play_tone(&self, freq: f32, duration_ms: u64) {
        let source = rodio::source::SineWave::new(freq)
            .take_duration(Duration::from_millis(duration_ms))
            .amplify(0.1); // Lower volume

        // Use play_raw to avoid Sink overhead for short beeps
        let _ = self.stream_handle.play_raw(source.convert_samples());
    }
}
