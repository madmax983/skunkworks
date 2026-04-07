use anyhow::Result;
use rand::Rng;
use std::sync::Arc;

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

#[cfg(not(feature = "audio"))]
pub type AudioStream = ();

#[cfg(feature = "audio")]
pub type AudioStream = cpal::Stream;

use crate::state::SharedState;

pub struct GlottalSource {
    phase: f32,
    pub frequency: f32,
    pub sample_rate: f32,
    pub tenseness: f32,
}

impl GlottalSource {
    pub fn new(frequency: f32) -> Self {
        Self {
            phase: 0.0,
            frequency,
            sample_rate: 44100.0,
            tenseness: 0.6, // Default mix
        }
    }

    pub fn set_frequency(&mut self, freq: f32) {
        self.frequency = freq;
    }

    pub fn next_sample(&mut self) -> f32 {
        // Naive sawtooth for tone
        self.phase += self.frequency / self.sample_rate;
        if self.phase >= 1.0 {
            self.phase -= 1.0;
        }
        let tone = (self.phase * 2.0) - 1.0;

        // Noise (Aspiration)
        let noise = rand::thread_rng().gen_range(-1.0..1.0);

        // Mix based on tenseness
        // High tenseness = more tone, less noise.
        // Low tenseness = more noise (whisper).
        let t = self.tenseness;
        let aspiration = noise * (1.0 - t) * 0.3; // Scale noise down a bit
        let voiced = tone * t;

        voiced + aspiration
    }
}

pub struct VocalTract {
    areas: Vec<f32>,
    k: Vec<f32>,

    // Double buffers for delay lines
    forward: Vec<f32>,
    backward: Vec<f32>,
    next_forward: Vec<f32>,
    next_backward: Vec<f32>,

    pub glottal_reflection: f32,
    pub lip_reflection: f32,
}

impl VocalTract {
    pub fn new(len: usize, default_area: f32) -> Self {
        let mut tract = Self {
            areas: vec![default_area; len],
            k: vec![0.0; len - 1], // N areas -> N-1 junctions
            forward: vec![0.0; len],
            backward: vec![0.0; len],
            next_forward: vec![0.0; len],
            next_backward: vec![0.0; len],
            glottal_reflection: 0.7,
            lip_reflection: -0.85,
        };
        tract.recalculate_reflections();
        tract
    }

    pub fn update_areas(&mut self, new_areas: &[f32]) {
        for (i, &area) in new_areas.iter().enumerate() {
            if i < self.areas.len() {
                self.areas[i] = area.max(0.001);
            }
        }
        self.recalculate_reflections();
    }

    fn recalculate_reflections(&mut self) {
        for i in 0..self.areas.len() - 1 {
            let a1 = self.areas[i];
            let a2 = self.areas[i + 1];
            self.k[i] = (a2 - a1) / (a2 + a1);
        }
    }

    pub fn process(&mut self, input: f32) -> f32 {
        let n = self.areas.len();
        if n == 0 {
            return input;
        }

        // Handle Glottis (Input)
        let f0 = input + self.glottal_reflection * self.backward[0];
        self.next_forward[0] = f0;

        // Handle Internal Junctions (0 to N-2)
        for i in 0..n - 1 {
            let k = self.k[i];
            let f_in = self.forward[i];
            let b_in = self.backward[i + 1];

            // Scattering
            let w = k * (f_in - b_in);

            self.next_forward[i + 1] = f_in + w;
            self.next_backward[i] = b_in + w;
        }

        // Handle Lips (Output)
        let f_last = self.forward[n - 1];
        let b_last = self.lip_reflection * f_last;
        self.next_backward[n - 1] = b_last;

        // Swap buffers
        std::mem::swap(&mut self.forward, &mut self.next_forward);
        std::mem::swap(&mut self.backward, &mut self.next_backward);

        f_last
    }
}

#[cfg(feature = "audio")]
pub fn start_audio(state: Arc<SharedState>) -> Result<AudioStream> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or(anyhow::anyhow!("No output device found"))?;
    let config = device.default_output_config()?;

    match config.sample_format() {
        cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), state),
        cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), state),
        cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), state),
        _ => Err(anyhow::anyhow!("Unsupported sample format")),
    }
}

#[cfg(not(feature = "audio"))]
pub fn start_audio(_state: Arc<SharedState>) -> Result<AudioStream> {
    Ok(())
}

#[cfg(feature = "audio")]
fn run<T>(
    device: &cpal::Device,
    config: &cpal::StreamConfig,
    state: Arc<SharedState>,
) -> Result<AudioStream>
where
    T: cpal::Sample + cpal::FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let mut glottis = GlottalSource::new(110.0);
    glottis.sample_rate = sample_rate;

    let initial_params = state.params.read();
    let mut tract = VocalTract::new(initial_params.areas.len(), 1.0);
    tract.update_areas(&initial_params.areas);
    drop(initial_params);

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            // Read params
            let params = state.params.read();

            if !params.is_speaking {
                for sample in data.iter_mut() {
                    *sample = T::from_sample(0.0);
                }
                state.metrics.set_level(0.0);
                return;
            }

            glottis.frequency = params.frequency;
            glottis.tenseness = params.tenseness;
            tract.update_areas(&params.areas);
            drop(params);

            let mut max_amp: f32 = 0.0;

            for frame in data.chunks_mut(channels) {
                let source = glottis.next_sample();
                let output = tract.process(source);
                max_amp = max_amp.max(output.abs());

                let sample_val = T::from_sample(output);
                for sample in frame.iter_mut() {
                    *sample = sample_val;
                }
            }
            state.metrics.set_level(max_amp);
        },
        err_fn,
        None,
    )?;

    stream.play()?;

    Ok(stream)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_glottal_source_output() {
        let mut source = GlottalSource::new(100.0);
        let mut max_amp: f32 = 0.0;
        for _ in 0..1000 {
            let s = source.next_sample();
            max_amp = max_amp.max(s.abs());
        }
        assert!(max_amp > 0.0, "Source should produce non-zero signal");
    }

    #[test]
    fn test_tract_propagation() {
        let mut tract = VocalTract::new(10, 1.0);
        tract.process(1.0);
        let mut received_energy = false;
        for _ in 0..50 {
            let out = tract.process(0.0);
            if out.abs() > 0.0001 {
                received_energy = true;
                break;
            }
        }
        assert!(received_energy, "Tract should propagate signal to output");
    }
}
