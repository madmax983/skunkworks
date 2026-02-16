use std::sync::{Arc, Mutex};

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct AudioEngine {
    #[cfg(feature = "audio")]
    _stream: Option<cpal::Stream>,
    state: Arc<Mutex<AudioState>>,
    time: f32,
}

#[allow(dead_code)]
struct AudioState {
    phase: f32,
    frequency: f32,
    volume: f32,
}

impl AudioEngine {
    pub fn new() -> Self {
        let state = Arc::new(Mutex::new(AudioState {
            phase: 0.0,
            frequency: 220.0,
            volume: 0.5,
        }));

        #[cfg(feature = "audio")]
        let stream = Self::init_cpal(state.clone());

        Self {
            #[cfg(feature = "audio")]
            _stream: stream,
            state,
            time: 0.0,
        }
    }

    #[cfg(feature = "audio")]
    fn init_cpal(state: Arc<Mutex<AudioState>>) -> Option<cpal::Stream> {
        let host = cpal::default_host();
        let device = host.default_output_device()?;
        let config = device.default_output_config().ok()?;

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => {
                let state_clone = state.clone();
                let sample_rate = config.sample_rate().0 as f32;
                let mut phase = 0.0;

                device
                    .build_output_stream(
                        &config.into(),
                        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                            // Use a simple sine wave
                            // In a real loop we'd be careful about lock contention
                            // But for a moonshot, locking per buffer is fine.
                            let (freq, vol) = {
                                let s = state_clone.lock().unwrap();
                                (s.frequency, s.volume)
                            };

                            let phase_inc = freq / sample_rate;

                            for sample in data.iter_mut() {
                                *sample = (phase * 2.0 * std::f32::consts::PI).sin() * vol;
                                phase = (phase + phase_inc) % 1.0;
                            }
                        },
                        err_fn,
                        None,
                    )
                    .ok()?
            }
            _ => return None, // Only supporting F32
        };

        stream.play().ok()?;
        Some(stream)
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;

        // LFO logic: Modulate frequency over time
        // This simulates "Audio" changing.
        let lfo = (self.time * 0.5).sin() * 0.5 + 0.5; // 0..1
        let freq = 100.0 + lfo * 700.0; // 100..800 Hz

        let mut s = self.state.lock().unwrap();
        s.frequency = freq;
    }

    pub fn get_energy(&self) -> f32 {
        let s = self.state.lock().unwrap();
        // Return normalized energy (0..1)
        (s.frequency - 100.0) / 700.0
    }
}
