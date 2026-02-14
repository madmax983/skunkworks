use anyhow::Result;
use crossbeam_channel::Receiver;

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

#[derive(Clone, Copy, Debug)]
#[allow(dead_code)]
pub struct SpikeEvent {
    pub frequency: f32,
    pub amplitude: f32,
}

#[allow(dead_code)]
struct Grain {
    frequency: f32,
    phase: f32,
    amplitude: f32,
    duration_samples: usize,
    elapsed_samples: usize,
}

impl Grain {
    #[allow(dead_code)]
    fn new(frequency: f32, amplitude: f32, sample_rate: f32) -> Self {
        let duration_ms = 100.0; // 100ms grain
        let duration_samples = (duration_ms / 1000.0 * sample_rate) as usize;
        Self {
            frequency,
            phase: 0.0,
            amplitude,
            duration_samples,
            elapsed_samples: 0,
        }
    }

    #[allow(dead_code)]
    fn next_sample(&mut self, sample_rate: f32) -> Option<f32> {
        if self.elapsed_samples >= self.duration_samples {
            return None;
        }

        let t = self.elapsed_samples as f32 / self.duration_samples as f32;
        // Simple triangular envelope: 0 -> 1 -> 0
        let env = if t < 0.5 { t * 2.0 } else { (1.0 - t) * 2.0 };

        let sample = (self.phase * 2.0 * std::f32::consts::PI).sin() * env * self.amplitude;

        self.phase += self.frequency / sample_rate;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }

        self.elapsed_samples += 1;
        Some(sample)
    }
}

pub struct AudioEngine {
    #[cfg(feature = "audio")]
    _stream: cpal::Stream,
    #[cfg(not(feature = "audio"))]
    _thread: std::thread::JoinHandle<()>,
}

impl AudioEngine {
    pub fn new(hit_rx: Receiver<SpikeEvent>) -> Result<Self> {
        #[cfg(feature = "audio")]
        {
            let host = cpal::default_host();
            let device = host.default_output_device().ok_or_else(|| anyhow::anyhow!("No audio device"))?;
            let config = device.default_output_config()?;
            let sample_rate = config.sample_rate().0 as f32;
            let channels = config.channels() as usize;

            let mut grains: Vec<Grain> = Vec::with_capacity(1024);

            let stream = device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    // Process new spikes
                    while let Ok(spike) = hit_rx.try_recv() {
                        grains.push(Grain::new(spike.frequency, spike.amplitude, sample_rate));
                    }

                    for frame in data.chunks_mut(channels) {
                        let mut mix = 0.0;
                        for grain in grains.iter_mut() {
                            if let Some(sample) = grain.next_sample(sample_rate) {
                                mix += sample;
                            }
                        }

                        // Soft clip
                        mix = mix.tanh();

                        for sample in frame.iter_mut() {
                            *sample = mix;
                        }
                    }

                    // Cleanup finished grains
                    grains.retain(|g| g.elapsed_samples < g.duration_samples);
                },
                move |err| eprintln!("Audio stream error: {}", err),
                None,
            )?;

            stream.play()?;

            Ok(Self { _stream: stream })
        }

        #[cfg(not(feature = "audio"))]
        {
             let thread = std::thread::spawn(move || {
                loop {
                    // Drain the channel to prevent overflow
                    if let Ok(_) = hit_rx.recv() {
                        // Consume events
                    }
                }
            });
            Ok(Self { _thread: thread })
        }
    }
}
