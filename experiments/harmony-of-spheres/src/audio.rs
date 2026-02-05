use std::sync::{Arc, RwLock};

#[derive(Clone, Copy, Debug)]
pub struct Oscillator {
    pub frequency: f32,
    pub amplitude: f32,
    // Phase is maintained by the audio thread
}

pub struct SharedState {
    pub oscillators: Vec<Oscillator>,
    pub master_volume: f32,
}

impl Default for SharedState {
    fn default() -> Self {
        Self::new()
    }
}

impl SharedState {
    pub fn new() -> Self {
        Self {
            oscillators: Vec::new(),
            master_volume: 0.1,
        }
    }
}

pub struct AudioHandle {
    #[cfg(feature = "audio")]
    pub _stream: Option<cpal::Stream>,
}

#[cfg(not(feature = "audio"))]
pub fn init_audio(_state: Arc<RwLock<SharedState>>) -> Result<AudioHandle, anyhow::Error> {
    Ok(AudioHandle {})
}

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

#[cfg(feature = "audio")]
pub fn init_audio(state: Arc<RwLock<SharedState>>) -> Result<AudioHandle, anyhow::Error> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or(anyhow::anyhow!("No output device available"))?;
    let config = device.default_output_config()?;

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), state),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), state),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), state),
        _ => return Err(anyhow::anyhow!("Unsupported sample format")),
    }?;

    Ok(AudioHandle {
        _stream: Some(stream),
    })
}

#[cfg(feature = "audio")]
fn run<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    state: Arc<RwLock<SharedState>>,
) -> Result<cpal::Stream, anyhow::Error>
where
    T: cpal::Sample + cpal::SizedSample + cpal::FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;
    let mut phases: Vec<f32> = Vec::new();

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            let (oscillators, master_vol) = {
                if let Ok(s) = state.read() {
                    (s.oscillators.clone(), s.master_volume)
                } else {
                    return;
                }
            };

            if phases.len() < oscillators.len() {
                phases.resize(oscillators.len(), 0.0);
            }

            for frame in data.chunks_mut(channels) {
                let mut sample_value = 0.0;

                for (i, osc) in oscillators.iter().enumerate() {
                    if i >= phases.len() {
                        break;
                    }

                    let val = (phases[i] * 2.0 * std::f32::consts::PI).sin();
                    sample_value += val * osc.amplitude;

                    phases[i] += osc.frequency / sample_rate;
                    if phases[i] > 1.0 {
                        phases[i] -= 1.0;
                    }
                }

                sample_value *= master_vol;
                sample_value = sample_value.clamp(-1.0, 1.0);

                let sample: T = T::from_sample(sample_value);
                for sample_out in frame.iter_mut() {
                    *sample_out = sample;
                }
            }
        },
        err_fn,
        None,
    )?;

    Ok(stream)
}
