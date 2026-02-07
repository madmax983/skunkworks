#[cfg(feature = "audio")]
use std::sync::atomic::{AtomicU32, Ordering};
#[cfg(feature = "audio")]
use std::sync::Arc;
#[cfg(feature = "audio")]
use rodio::Source;

pub struct Drone {
    #[cfg(feature = "audio")]
    freq: Arc<AtomicU32>,
    #[cfg(feature = "audio")]
    _stream: rodio::OutputStream,
    #[cfg(feature = "audio")]
    _handle: rodio::OutputStreamHandle,
    #[cfg(feature = "audio")]
    _sink: rodio::Sink,
}

impl Drone {
    pub fn new() -> Option<Self> {
        #[cfg(feature = "audio")]
        {
            let (stream, handle) = rodio::OutputStream::try_default().ok()?;
            let sink = rodio::Sink::try_new(&handle).ok()?;

            let freq = Arc::new(AtomicU32::new(440_f32.to_bits()));
            let source = DroneSource {
                freq: freq.clone(),
                phase: 0.0,
                sample_rate: 44100,
            };

            sink.append(source);
            // Sink usually plays automatically or needs explicit play?
            // sink.play(); // sink.play() is for pausing/resuming usually. appending starts playback.

            Some(Self {
                freq,
                _stream: stream,
                _handle: handle,
                _sink: sink,
            })
        }
        #[cfg(not(feature = "audio"))]
        {
            Some(Self {})
        }
    }

    pub fn set_freq(&self, f: f32) {
        #[cfg(feature = "audio")]
        self.freq.store(f.to_bits(), Ordering::Relaxed);

        #[cfg(not(feature = "audio"))]
        let _ = f;
    }
}

#[cfg(feature = "audio")]
struct DroneSource {
    freq: Arc<AtomicU32>,
    phase: f32,
    sample_rate: u32,
}

#[cfg(feature = "audio")]
impl Iterator for DroneSource {
    type Item = f32;

    fn next(&mut self) -> Option<f32> {
        let f_bits = self.freq.load(Ordering::Relaxed);
        let f = f32::from_bits(f_bits);

        self.phase += f * 2.0 * std::f32::consts::PI / self.sample_rate as f32;
        if self.phase > 2.0 * std::f32::consts::PI {
            self.phase -= 2.0 * std::f32::consts::PI;
        }

        Some(self.phase.sin() * 0.1) // Low volume
    }
}

#[cfg(feature = "audio")]
impl Source for DroneSource {
    fn current_frame_len(&self) -> Option<usize> { None }
    fn channels(&self) -> u16 { 1 }
    fn sample_rate(&self) -> u32 { self.sample_rate }
    fn total_duration(&self) -> Option<std::time::Duration> { None }
}
