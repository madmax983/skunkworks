use std::sync::{Arc, Mutex};

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

#[repr(C)]
#[derive(Debug, Clone, Copy, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Params {
    pub damping: f32,
    pub freq: f32,
    pub amp: f32,
    pub time: f32,
}

impl Default for Params {
    fn default() -> Self {
        Self {
            damping: 0.99,
            freq: 5.0,
            amp: 1.0,
            time: 0.0,
        }
    }
}

pub struct AudioSystem {
    #[cfg(feature = "audio")]
    _stream: cpal::Stream,
    pub params: Arc<Mutex<Params>>,
}

impl AudioSystem {
    pub fn new() -> anyhow::Result<Self> {
        let params = Arc::new(Mutex::new(Params::default()));

        #[cfg(feature = "audio")]
        {
            let host = cpal::default_host();
            let device = host
                .default_output_device()
                .ok_or_else(|| anyhow::anyhow!("No audio output device available"))?;

            let config = device.default_output_config()?;
            let sample_rate = config.sample_rate().0 as f32;
            let params_clone = params.clone();
            let mut phase = 0.0;

            let err_fn = |err| eprintln!("Audio stream error: {}", err);

            let stream = match config.sample_format() {
                cpal::SampleFormat::F32 => device.build_output_stream(
                    &config.into(),
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        let current_params = {
                            let p = params_clone.lock().unwrap();
                            *p
                        };

                        for sample in data.iter_mut() {
                            let audio_freq = current_params.freq * 10.0;
                            *sample = (phase * 2.0 * std::f32::consts::PI).sin()
                                * 0.1
                                * current_params.amp;
                            phase = (phase + audio_freq / sample_rate) % 1.0;
                        }
                    },
                    err_fn,
                    None,
                )?,
                _ => return Err(anyhow::anyhow!("Unsupported sample format (F32 required)")),
            };

            stream.play()?;

            Ok(Self {
                _stream: stream,
                params,
            })
        }

        #[cfg(not(feature = "audio"))]
        {
            println!(
                "Audio disabled (build with --features audio). Running in silent simulation mode."
            );
            Ok(Self { params })
        }
    }
}
