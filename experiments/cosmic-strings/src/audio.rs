use crate::physics::CosmicString;

#[cfg(feature = "audio")]
use rodio::{source::Source, OutputStream, OutputStreamHandle, Sink};
#[cfg(feature = "audio")]
use std::sync::{Arc, Mutex};
#[cfg(feature = "audio")]
use std::time::Duration;

pub struct AudioSystem {
    #[cfg(feature = "audio")]
    _stream: OutputStream,
    #[cfg(feature = "audio")]
    _stream_handle: OutputStreamHandle,
    #[cfg(feature = "audio")]
    sink: Sink,
    #[cfg(feature = "audio")]
    source_data: Arc<Mutex<SharedData>>,
}

#[cfg(feature = "audio")]
struct SharedData {
    freq: f32,
    volume: f32,
}

#[cfg(feature = "audio")]
struct DynamicSource {
    data: Arc<Mutex<SharedData>>,
    phase: f32,
    sample_rate: u32,
}

#[cfg(feature = "audio")]
impl Iterator for DynamicSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        let (freq, volume) = {
            let data = self.data.lock().unwrap();
            (data.freq, data.volume)
        };

        self.phase = (self.phase + freq / self.sample_rate as f32) % 1.0;
        let sample = (self.phase * 2.0 * std::f32::consts::PI).sin();
        Some(sample * volume)
    }
}

#[cfg(feature = "audio")]
impl Source for DynamicSource {
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
        None
    }
}

impl AudioSystem {
    pub fn new() -> anyhow::Result<Self> {
        #[cfg(feature = "audio")]
        {
            let (_stream, stream_handle) = OutputStream::try_default()?;
            let sink = Sink::try_new(&stream_handle)?;

            let data = Arc::new(Mutex::new(SharedData {
                freq: 440.0,
                volume: 0.0,
            }));

            let source = DynamicSource {
                data: data.clone(),
                phase: 0.0,
                sample_rate: 44100,
            };

            sink.append(source);

            Ok(Self {
                _stream,
                _stream_handle: stream_handle,
                sink,
                source_data: data,
            })
        }
        #[cfg(not(feature = "audio"))]
        {
            Ok(Self {})
        }
    }

    pub fn update(&mut self, string: &CosmicString) {
        #[cfg(feature = "audio")]
        {
            if string.nodes.len() < 2 {
                return;
            }

            let segments = string.nodes.len() - 1;
            let total_length = string.rest_length * segments as f32;

            // mu = mass / rest_length (linear density)
            // Node mass is 1.0. mu = 1.0 / rest_length
            let mu = 1.0 / string.rest_length;

            if mu > 0.0 && total_length > 0.0 {
                let freq = (1.0 / (2.0 * total_length)) * (string.tension / mu).sqrt();

                // Volume based on Kinetic Energy
                let ke = string.total_kinetic_energy();
                let volume = (ke * 0.1).clamp(0.0, 0.5);

                if let Ok(mut data) = self.source_data.lock() {
                    data.freq = freq.max(20.0);
                    data.volume = volume;
                }
            }
        }
        #[cfg(not(feature = "audio"))]
        {
            let _ = string;
        }
    }
}
