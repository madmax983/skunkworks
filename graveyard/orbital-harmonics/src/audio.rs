#[cfg(feature = "audio")]
use rodio::{OutputStream, OutputStreamHandle, Sink, source::Source};
#[cfg(feature = "audio")]
use std::time::Duration;

pub struct AudioEngine {
    #[cfg(feature = "audio")]
    _stream: Option<OutputStream>,
    #[cfg(feature = "audio")]
    stream_handle: Option<OutputStreamHandle>,
    enabled: bool,
}

impl AudioEngine {
    pub fn new() -> Self {
        #[cfg(feature = "audio")]
        {
            match OutputStream::try_default() {
                Ok((stream, handle)) => Self {
                    _stream: Some(stream),
                    stream_handle: Some(handle),
                    enabled: true,
                },
                Err(_) => Self {
                    _stream: None,
                    stream_handle: None,
                    enabled: false,
                },
            }
        }
        #[cfg(not(feature = "audio"))]
        {
            Self { enabled: false }
        }
    }

    pub fn play_freq(&self, freq: f32) {
        // Prevent unused variable warning when audio feature is disabled
        #[cfg(not(feature = "audio"))]
        let _ = freq;

        if self.enabled {
            #[cfg(feature = "audio")]
            if let Some(handle) = &self.stream_handle {
                if let Ok(sink) = Sink::try_new(handle) {
                    let source = rodio::source::SineWave::new(freq)
                        .take_duration(Duration::from_millis(200))
                        .amplify(0.10)
                        .fade_in(Duration::from_millis(10))
                        .fade_out(Duration::from_millis(190));
                    sink.append(source);
                    sink.detach();
                }
            }
        }
    }
}

// Simple Pentatonic Mapping
// Maps a metric (like radius) to a frequency from a scale
pub fn map_to_scale(val: f64) -> f32 {
    // Arbitrary mapping: larger radius = lower pitch
    // Base C4 = 261.63
    // We want a range. Say val goes from 50 to 200.

    let scale = [
        130.81, // C3
        146.83, // D3
        164.81, // E3
        196.00, // G3
        220.00, // A3
        261.63, // C4
        293.66, // D4
        329.63, // E4
        392.00, // G4
        440.00, // A4
        523.25, // C5
        587.33, // D5
        659.25, // E5
        783.99, // G5
        880.00, // A5
    ];

    // Invert: smaller val -> higher index
    // Let's assume input val is roughly 30.0 to 150.0
    // Normalized 0..1
    let clamped = val.clamp(30.0, 150.0);
    let norm = 1.0 - (clamped - 30.0) / 120.0; // 1.0 (small r) to 0.0 (large r)

    let idx = (norm * (scale.len() as f64 - 1.0)).round() as usize;
    scale[idx]
}
