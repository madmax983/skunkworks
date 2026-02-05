use std::sync::{Arc, Mutex};

pub struct AudioParams {
    pub frequency: f32,
    pub amplitude: f32,
}

pub struct AudioEngine {
    params: Arc<Mutex<AudioParams>>,
}

impl AudioEngine {
    pub fn new() -> Self {
        Self {
            params: Arc::new(Mutex::new(AudioParams {
                frequency: 440.0,
                amplitude: 0.1,
            })),
        }
    }

    pub fn get_params(&self) -> Arc<Mutex<AudioParams>> {
        self.params.clone()
    }

    pub fn next_sample(&self, phase: &mut f32, sample_rate: f32) -> f32 {
        let params = self.params.lock().unwrap();
        let freq = params.frequency;
        let amp = params.amplitude;

        let val = (*phase * 2.0 * std::f32::consts::PI).sin() * amp;

        // Update phase
        *phase += freq / sample_rate;
        if *phase > 1.0 {
            *phase -= 1.0;
        }

        val
    }

    #[cfg(feature = "audio")]
    pub fn start_stream(&self) -> anyhow::Result<cpal::Stream> {
        use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No output device available"))?;
        let config = device.default_output_config()?;
        let sample_rate = config.sample_rate().0 as f32;

        let engine = self.clone(); // Clone ARC

        // We need to move state into the closure
        let mut phase = 0.0;

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    for sample in data.iter_mut() {
                        *sample = engine.next_sample(&mut phase, sample_rate);
                    }
                },
                err_fn,
                None,
            )?,
            _ => return Err(anyhow::anyhow!("Unsupported sample format")),
        };

        stream.play()?;
        Ok(stream)
    }

    #[cfg(not(feature = "audio"))]
    pub fn start_stream(&self) -> anyhow::Result<()> {
        log::info!("Audio disabled: feature 'audio' not enabled.");
        Ok(())
    }
}

// Clone for AudioEngine to allow passing into closure
impl Clone for AudioEngine {
    fn clone(&self) -> Self {
        Self {
            params: self.params.clone(),
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_oscillator() {
        let engine = AudioEngine::new();
        let mut phase = 0.0;
        let sample_rate = 44100.0;

        // 1. Check initial output (phase 0)
        // sin(0) = 0
        let s1 = engine.next_sample(&mut phase, sample_rate);
        assert!(s1.abs() < 1e-5, "Initial sample should be near 0");

        // 2. Advance a bit
        let s2 = engine.next_sample(&mut phase, sample_rate);
        // Phase increment = 440 / 44100 ~= 0.01
        // sin(0.01 * 2pi) > 0
        assert!(s2 > 0.0, "Sample should be increasing for sine wave");

        // 3. Change frequency
        {
            let params = engine.get_params();
            let mut p = params.lock().unwrap();
            p.frequency = 880.0;
        }

        // Run for a bit and see if it moves faster?
        let _ = engine.next_sample(&mut phase, sample_rate);
    }
}
