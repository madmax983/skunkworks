#[cfg(feature = "audio")]
use rodio::{source::Source, OutputStream, OutputStreamHandle};
#[cfg(feature = "audio")]
use std::time::Duration;

pub struct AudioEngine {
    #[cfg(feature = "audio")]
    _stream: Option<OutputStream>,
    #[cfg(feature = "audio")]
    stream_handle: Option<OutputStreamHandle>,
}

impl AudioEngine {
    pub fn new() -> Self {
        #[cfg(feature = "audio")]
        {
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
        #[cfg(not(feature = "audio"))]
        {
            Self {}
        }
    }

    #[allow(unused_variables)]
    pub fn play_tone(&self, frequency: f32, duration_secs: f32) {
        #[cfg(feature = "audio")]
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
