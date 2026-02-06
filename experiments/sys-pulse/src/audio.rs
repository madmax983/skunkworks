use crate::monitor::SystemStats;
use crate::rhythm::Euclidean;
use std::sync::{Arc, RwLock};

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "audio")]
use std::f32::consts::PI;

pub struct AudioEngine {
    #[cfg(feature = "audio")]
    _stream: cpal::Stream,
}

#[cfg(feature = "audio")]
struct SynthState {
    phase: f32,
    tick_counter: usize,
    step_index: usize,
    pattern: Vec<bool>,
    samples_per_step: usize,

    // Envelope
    env_phase: f32,
    is_playing: bool,

    // Cached Params
    cached_cpu: f32,
    cached_mem: f32,

    // RNG
    rng_seed: u32,
}

impl AudioEngine {
    pub fn new(stats: Arc<RwLock<SystemStats>>) -> anyhow::Result<Self> {
        #[cfg(feature = "audio")]
        {
            let host = cpal::default_host();
            let device = host
                .default_output_device()
                .ok_or_else(|| anyhow::anyhow!("No output device available"))?;

            let config = device.default_output_config()?;
            let sample_rate = config.sample_rate().0 as f32;
            let channels = config.channels() as usize;

            let mut synth = SynthState {
                phase: 0.0,
                tick_counter: 0,
                step_index: 0,
                pattern: Euclidean::generate(5, 16), // Default
                samples_per_step: (sample_rate * 0.15) as usize, // Default ~100ms
                env_phase: 0.0,
                is_playing: false,
                cached_cpu: 0.0,
                cached_mem: 0.0,
                rng_seed: 12345,
            };

            let stream = match config.sample_format() {
                cpal::SampleFormat::F32 => device.build_output_stream(
                    &config.into(),
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        process_audio(data, channels, sample_rate, &stats, &mut synth);
                    },
                    |err| eprintln!("Stream error: {}", err),
                    None,
                )?,
                _ => return Err(anyhow::anyhow!("Unsupported sample format")),
            };

            stream.play()?;
            Ok(Self { _stream: stream })
        }
        #[cfg(not(feature = "audio"))]
        {
            println!("Audio disabled.");
            Ok(Self {})
        }
    }
}

#[cfg(feature = "audio")]
fn process_audio(
    output: &mut [f32],
    channels: usize,
    sample_rate: f32,
    stats: &Arc<RwLock<SystemStats>>,
    synth: &mut SynthState,
) {
    // Try to update stats occasionally (every step?)
    // Using try_read to avoid blocking
    if let Ok(lock) = stats.try_read() {
        synth.cached_cpu = lock.cpu_usage; // 0-100
        synth.cached_mem = lock.memory_usage; // 0-1

        // Map CPU to Density (k)
        // k from 1 to 16
        let k = ((synth.cached_cpu / 100.0) * 16.0).round().max(1.0) as usize;
        let k = k.min(16);

        // If k changed, regenerate pattern?
        // Or just regenerate every time (cheap enough for 16 steps?)
        // Let's regenerate if we are at step 0 to avoid glitching mid-measure
        if synth.step_index == 0 && synth.tick_counter == 0 {
             synth.pattern = Euclidean::generate(k, 16);

             // Map Memory to Tempo? Higher memory -> Slower?
             // Or fixed tempo?
             // Let's make tempo fixed for now (120 BPM = 125ms per 16th)
             // sample_rate * 0.125
             synth.samples_per_step = (sample_rate * 0.125) as usize;
        }
    }

    for frame in output.chunks_mut(channels) {
        // Sequencer Logic
        synth.tick_counter += 1;
        if synth.tick_counter >= synth.samples_per_step {
            synth.tick_counter = 0;
            synth.step_index = (synth.step_index + 1) % 16;

            // Check if we should trigger
            if synth.step_index < synth.pattern.len() && synth.pattern[synth.step_index] {
                synth.is_playing = true;
                synth.env_phase = 0.0;
            }
        }

        // Synthesis Logic
        let mut sample = 0.0;
        if synth.is_playing {
            synth.env_phase += 1.0 / sample_rate;

            // Simple Envelope (Decay)
            let decay = 0.1 + (synth.cached_mem * 0.5); // Memory extends decay
            let amp = (-synth.env_phase / decay).exp();

            if amp < 0.001 {
                synth.is_playing = false;
            }

            // Oscillator
            // Pitch based on step index? Or fixed?
            // Let's map Swap to Pitch shift?
            let base_freq = 220.0;
            // Pentatonic scale logic?
            // Just noise + sine for Kick/Tom sound
            let freq = base_freq * (1.0 - (synth.step_index as f32 / 32.0)); // Drop pitch slightly over the bar

            let sine = (synth.phase * 2.0 * PI).sin();

            // LCG Noise
            synth.rng_seed = synth.rng_seed.wrapping_mul(1103515245).wrapping_add(12345);
            let noise = (synth.rng_seed as f32 / u32::MAX as f32) * 2.0 - 1.0;

            // Mix: More noise if CPU is high?
            let mix = synth.cached_cpu / 150.0; // 0.0 - 0.6
            let osc = sine * (1.0 - mix) + noise * mix;

            sample = osc * amp * 0.5;

            synth.phase = (synth.phase + freq / sample_rate) % 1.0;
        }

        for channel in frame {
            *channel = sample;
        }
    }
}
