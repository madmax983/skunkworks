use std::sync::{Arc, RwLock};

pub struct Oscillator {
    pub frequency: f32,
    pub amplitude: f32,
}

pub struct SharedState {
    pub oscillators: Vec<Oscillator>,
}

impl SharedState {
    pub fn new() -> Self {
        Self { oscillators: Vec::new() }
    }
}

pub struct AudioHandle {
    #[cfg(feature = "audio")]
    _stream: cpal::Stream,
}

pub fn init_audio(_state: Arc<RwLock<SharedState>>) -> anyhow::Result<AudioHandle> {
    #[cfg(feature = "audio")]
    {
        use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

        let host = cpal::default_host();
        let device = host.default_output_device().ok_or_else(|| anyhow::anyhow!("No audio device"))?;
        let config = device.default_output_config()?;
        let sample_rate = config.sample_rate().0 as f32;
        let channels = config.channels() as usize;

        let state_clone = _state.clone();
        let mut phases = Vec::new(); // Phase for each oscillator, we might need to manage this dynamically

        // Simplification: Just sum up all oscillators.
        // But oscillators change every frame (cleared and re-added).
        // This causes phase discontinuities (clicks).
        // A proper synth handles this by tracking active notes.
        // For this experiment, we'll accept some clicks or just use a fixed number of oscillators?
        // Let's use a simple approach: render whatever is in the state *right now*.
        // To avoid clicks, we should track phase per "voice" but we don't have voices.
        // We'll just maintain a global time or phase.

        let mut global_phase = 0.0;

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    if let Ok(state) = state_clone.read() {
                        for frame in data.chunks_mut(channels) {
                            let mut sample = 0.0;
                            for osc in &state.oscillators {
                                sample += (global_phase * osc.frequency * 2.0 * std::f32::consts::PI).sin() * osc.amplitude;
                            }

                            // Soft clipping
                            sample = sample.tanh();

                            for sample_out in frame.iter_mut() {
                                *sample_out = sample;
                            }
                            global_phase = (global_phase + 1.0 / sample_rate) % 1000.0; // Wrap to avoid float precision loss eventually
                        }
                    }
                },
                err_fn,
                None,
            )?,
            _ => return Err(anyhow::anyhow!("Unsupported sample format")),
        };

        stream.play()?;

        Ok(AudioHandle { _stream: stream })
    }

    #[cfg(not(feature = "audio"))]
    {
        Ok(AudioHandle {})
    }
}
