use std::sync::{Arc, Mutex};

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct AudioEngine {
    #[cfg(feature = "audio")]
    _stream: Option<cpal::Stream>,
    state: Arc<Mutex<AudioState>>,
}

struct AudioState {
    frequency: f32,
    volume: f32,
    modulation: f32,
}

impl AudioEngine {
    pub fn new() -> Self {
        let state = Arc::new(Mutex::new(AudioState {
            frequency: 220.0,
            volume: 0.1,
            modulation: 0.0,
        }));

        #[cfg(feature = "audio")]
        let stream = Self::init_cpal(state.clone());

        Self {
            #[cfg(feature = "audio")]
            _stream: stream,
            state,
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
                let mut mod_phase = 0.0;

                device.build_output_stream(
                    &config.into(),
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        let (base_freq, vol, modulation) = {
                            let s = state_clone.lock().unwrap();
                            (s.frequency, s.volume, s.modulation)
                        };

                        let phase_inc = base_freq / sample_rate;
                        let mod_freq = base_freq * 0.5; // Harmonic ratio
                        let mod_inc = mod_freq / sample_rate;

                        // Scale modulation depth by frequency so it sounds consistent
                        let mod_idx = modulation; // 0..1

                        for sample in data.iter_mut() {
                            // FM Synthesis
                            // Modulator
                            let mod_val = (mod_phase * 2.0 * std::f32::consts::PI).sin() * mod_idx;
                            // Carrier
                            let carrier_val = ((phase + mod_val) * 2.0 * std::f32::consts::PI).sin();

                            *sample = carrier_val * vol;

                            phase = (phase + phase_inc) % 1.0;
                            mod_phase = (mod_phase + mod_inc) % 1.0;
                        }
                    },
                    err_fn,
                    None,
                ).ok()?
            },
            _ => return None,
        };

        stream.play().ok()?;
        Some(stream)
    }

    #[cfg(not(feature = "audio"))]
    fn init_cpal(_state: Arc<Mutex<AudioState>>) -> Option<()> {
        None
    }

    pub fn update(&mut self, total_chem_a: f32, total_chem_b: f32) {
        let mut s = self.state.lock().unwrap();

        // Normalize roughly
        let norm_a = (total_chem_a / 10000.0).clamp(0.0, 1.0);
        let norm_b = (total_chem_b / 2000.0).clamp(0.0, 1.0);

        // A (Substrate) drives Pitch (Low A -> Low Energy -> Low Pitch)
        // B (Activator) drives Modulation (Chaos)
        s.frequency = 100.0 + norm_a * 200.0; // 100..300 Hz
        s.modulation = norm_b * 0.5; // Depth
        s.volume = 0.15;
    }
}
