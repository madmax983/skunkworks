#[derive(Debug, Clone, Copy)]
pub enum AudioEvent {
    Spike(usize),
}

#[cfg(feature = "audio")]
pub mod engine {
    use super::*;
    use anyhow::anyhow;
    use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
    use std::f32::consts::PI;
    use std::sync::mpsc::{self, Sender};

    pub struct AudioHandle {
        // We must keep the stream alive
        pub stream: cpal::Stream,
        pub sender: Sender<AudioEvent>,
    }

    struct Voice {
        active: bool,
        freq: f32,
        phase: f32,
        envelope_time: f32,
        decay: f32,
    }

    impl Voice {
        fn new(freq: f32, decay: f32) -> Self {
            Self {
                active: false,
                freq,
                phase: 0.0,
                envelope_time: 0.0,
                decay,
            }
        }

        fn trigger(&mut self) {
            self.active = true;
            self.envelope_time = 0.0;
            self.phase = 0.0;
        }

        fn sample(&mut self, dt: f32) -> f32 {
            if !self.active {
                return 0.0;
            }

            self.phase += self.freq * 2.0 * PI * dt;
            if self.phase > 2.0 * PI {
                self.phase -= 2.0 * PI;
            }

            self.envelope_time += dt;
            let amp = (-self.envelope_time * self.decay).exp();

            if amp < 0.001 {
                self.active = false;
                return 0.0;
            }

            amp * self.phase.sin()
        }
    }

    pub fn init() -> anyhow::Result<AudioHandle> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow!("No output device available"))?;
        let config = device.default_output_config()?;

        let sample_rate = config.sample_rate().0 as f32;
        let channels = config.channels() as usize;

        let (sender, receiver) = mpsc::channel();

        // Fixed set of voices mapped to neuron IDs
        let mut voices = vec![
            Voice::new(110.0, 5.0),  // Neuron 0: Kick-ish
            Voice::new(220.0, 10.0), // Neuron 1
            Voice::new(440.0, 10.0), // Neuron 2
            Voice::new(880.0, 20.0), // Neuron 3: Ping
        ];

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // Process events
                while let Ok(event) = receiver.try_recv() {
                    match event {
                        AudioEvent::Spike(id) => {
                            if id < voices.len() {
                                voices[id].trigger();
                            }
                        }
                    }
                }

                // Fill buffer
                for frame in data.chunks_mut(channels) {
                    let mut sample = 0.0;
                    let dt = 1.0 / sample_rate;

                    for voice in &mut voices {
                        sample += voice.sample(dt);
                    }

                    // Clamp to prevent clipping
                    sample = sample.max(-1.0).min(1.0);

                    for s in frame {
                        *s = sample;
                    }
                }
            },
            err_fn,
            None,
        )?;

        stream.play()?;

        Ok(AudioHandle { stream, sender })
    }
}
