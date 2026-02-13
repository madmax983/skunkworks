#[cfg(feature = "audio")]
use rodio::{OutputStream, Sink, Source};

#[derive(Clone, Copy, Debug)]
pub enum AudioEvent {
    CrossingZero(usize, f32), // Body index, radius (for pitch)
}

pub struct AudioEngine {
    #[cfg(feature = "audio")]
    _stream: Option<OutputStream>,
    #[cfg(feature = "audio")]
    stream_handle: Option<rodio::OutputStreamHandle>,
}

impl AudioEngine {
    pub fn new() -> Self {
        #[cfg(feature = "audio")]
        {
            let stream_result = OutputStream::try_default();
            if let Ok((stream, handle)) = stream_result {
                return Self {
                    _stream: Some(stream),
                    stream_handle: Some(handle),
                };
            } else {
                eprintln!("Failed to initialize audio stream");
                return Self {
                    _stream: None,
                    stream_handle: None,
                };
            }
        }
        #[cfg(not(feature = "audio"))]
        Self {}
    }

    #[allow(unused_variables)]
    pub fn play_event(&self, event: AudioEvent) {
        #[cfg(feature = "audio")]
        if let Some(handle) = &self.stream_handle {
            match event {
                AudioEvent::CrossingZero(_idx, radius) => {
                    // Map radius to frequency.
                    let freq = (10000.0 / (radius + 1.0)).clamp(50.0, 1000.0);
                    let source = rodio::source::SineWave::new(freq)
                        .take_duration(std::time::Duration::from_millis(200))
                        .amplify(0.1);

                    let _ = handle.play_raw(source.convert_samples());
                }
            }
        }
    }
}
