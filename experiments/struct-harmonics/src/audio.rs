#[cfg(feature = "audio")]
use rodio::{OutputStream, OutputStreamHandle, Sink, source::Source};
#[cfg(feature = "audio")]
use std::time::Duration;

pub struct AudioEngine {
    #[cfg(feature = "audio")]
    _stream: Option<OutputStream>,
    #[cfg(feature = "audio")]
    stream_handle: Option<OutputStreamHandle>,
}

impl AudioEngine {
    pub fn new() -> Option<Self> {
        #[cfg(feature = "audio")]
        {
            match OutputStream::try_default() {
                Ok((stream, handle)) => Some(Self {
                    _stream: Some(stream),
                    stream_handle: Some(handle),
                }),
                Err(_) => None,
            }
        }
        #[cfg(not(feature = "audio"))]
        {
            Some(Self {})
        }
    }

    pub fn play_freq(&self, freq: f32, volume: f32) {
        #[cfg(feature = "audio")]
        if let Some(handle) = &self.stream_handle {
            if let Ok(sink) = Sink::try_new(handle) {
                let source = rodio::source::SineWave::new(freq)
                    .take_duration(Duration::from_millis(150))
                    .amplify(volume)
                    .fade_in(Duration::from_millis(10))
                    .fade_out(Duration::from_millis(140));
                sink.append(source);
                sink.detach();
            }
        }
        #[cfg(not(feature = "audio"))]
        {
            // No-op
            let _ = freq;
            let _ = volume;
        }
    }
}

pub fn map_mass_to_freq(mass: f32) -> f32 {
    let clamped_mass = mass.clamp(1.0, 50.0);
    let t = 1.0 - ((clamped_mass - 1.0) / 49.0);
    100.0 + (t * 780.0)
}
