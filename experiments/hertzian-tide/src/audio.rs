use std::sync::atomic::AtomicU32;
use std::sync::Arc;

#[cfg(feature = "audio")]
use std::sync::atomic::Ordering;
#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

#[allow(dead_code)]
pub struct Synth {
    #[cfg(feature = "audio")]
    _stream: cpal::Stream,
    shared_freq: Arc<AtomicU32>,
}

impl Synth {
    pub fn new(shared_freq: Arc<AtomicU32>) -> Result<Self, anyhow::Error> {
        #[cfg(feature = "audio")]
        {
            let host = cpal::default_host();
            let device = host.default_output_device().ok_or_else(|| anyhow::anyhow!("No audio device"))?;
            let config = device.default_output_config()?;
            let sample_rate = config.sample_rate().0 as f32;
            let channels = config.channels() as usize;

            let freq_clone = shared_freq.clone();
            let mut phase = 0.0;

            let stream = device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for frame in data.chunks_mut(channels) {
                        let freq_bits = freq_clone.load(Ordering::Relaxed);
                        let freq = f32::from_bits(freq_bits);

                        // Simple Sine Wave
                        // phase increment = freq * 2pi / sample_rate
                        phase = (phase + freq * std::f32::consts::TAU / sample_rate) % std::f32::consts::TAU;
                        let sample = phase.sin() * 0.1; // Volume 0.1

                        for sample_out in frame.iter_mut() {
                            *sample_out = sample;
                        }
                    }
                },
                |err| eprintln!("Audio stream error: {}", err),
                None,
            )?;

            stream.play()?;

            Ok(Self {
                _stream: stream,
                shared_freq,
            })
        }

        #[cfg(not(feature = "audio"))]
        {
            Ok(Self {
                shared_freq,
            })
        }
    }
}
