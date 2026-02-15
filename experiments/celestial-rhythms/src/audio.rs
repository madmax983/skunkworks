use std::sync::{Arc, Mutex};
use anyhow::Result;

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "audio")]
use cpal::Stream;

#[derive(Clone, Debug)]
pub struct Oscillator {
    pub frequency: f32, // Hz
    pub amplitude: f32, // 0.0 to 1.0
    pub phase: f32,     // 0.0 to 2*PI
}

impl Oscillator {
    pub fn new(freq: f32, amp: f32) -> Self {
        Self {
            frequency: freq,
            amplitude: amp,
            phase: 0.0,
        }
    }
}

pub struct AudioEngine {
    #[cfg(feature = "audio")]
    _stream: Stream,
    shared_state: Arc<Mutex<Vec<Oscillator>>>,
}

impl AudioEngine {
    pub fn new() -> Result<Self> {
        let shared_state = Arc::new(Mutex::new(Vec::new()));

        #[cfg(feature = "audio")]
        {
            use anyhow::Context;

            let host = cpal::default_host();
            let device = host
                .default_output_device()
                .context("failed to find output device")?;
            let config = device.default_output_config()?;

            let sample_rate = config.sample_rate().0 as f32;
            let channels = config.channels() as usize;

            let state_for_callback = shared_state.clone();

            let stream = match config.sample_format() {
                cpal::SampleFormat::F32 => device.build_output_stream(
                    &config.into(),
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        write_data(data, channels, &state_for_callback, sample_rate)
                    },
                    |err| eprintln!("an error occurred on stream: {}", err),
                    None,
                )?,
                _ => return Err(anyhow::anyhow!("Unsupported sample format")),
            };

            stream.play()?;

            Ok(Self {
                _stream: stream,
                shared_state,
            })
        }

        #[cfg(not(feature = "audio"))]
        {
            println!("Audio disabled (missing 'audio' feature).");
            Ok(Self {
                shared_state,
            })
        }
    }

    /// Updates the frequency and amplitude of oscillators.
    pub fn update_oscillators(&self, params: &[(f32, f32)]) {
        let mut state = self.shared_state.lock().unwrap();

        if state.len() < params.len() {
             state.resize(params.len(), Oscillator::new(0.0, 0.0));
        } else if state.len() > params.len() {
             state.truncate(params.len());
        }

        for (i, (freq, amp)) in params.iter().enumerate() {
            state[i].frequency = *freq;
            state[i].amplitude = *amp;
        }
    }
}

#[cfg(feature = "audio")]
fn write_data(
    output: &mut [f32],
    channels: usize,
    state: &Arc<Mutex<Vec<Oscillator>>>,
    sample_rate: f32,
) {
    let mut oscillators = state.lock().unwrap();

    for frame in output.chunks_mut(channels) {
        let mut sample = 0.0;

        for osc in oscillators.iter_mut() {
            sample += osc.phase.sin() * osc.amplitude;

            // Advance phase
            osc.phase += osc.frequency * 2.0 * std::f32::consts::PI / sample_rate;
            if osc.phase > 2.0 * std::f32::consts::PI {
                osc.phase -= 2.0 * std::f32::consts::PI;
            }
        }

        // Soft clipping
        sample = (sample * 0.5).tanh();

        for channel_sample in frame.iter_mut() {
            *channel_sample = sample;
        }
    }
}
