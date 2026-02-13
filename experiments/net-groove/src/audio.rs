use std::sync::Arc;
use std::time::Duration;
use anyhow::Result;
use rand::Rng;

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

use crate::shared::GrooveState;

const SAMPLE_RATE: u32 = 44100;
const BPM: f64 = 120.0;

#[allow(dead_code)]
pub struct AudioEngine {
    // Keep reference if needed for other methods, but suppressed warning for now
    state: Arc<GrooveState>,
    #[cfg(feature = "audio")]
    _stream: cpal::Stream,
    #[cfg(not(feature = "audio"))]
    _thread: std::thread::JoinHandle<()>,
}

impl AudioEngine {
    pub fn new(state: Arc<GrooveState>) -> Result<Self> {
        let state_clone = state.clone();

        #[cfg(feature = "audio")]
        {
            let host = cpal::default_host();
            let device = host.default_output_device().ok_or_else(|| anyhow::anyhow!("No audio device"))?;
            let config = device.default_output_config()?;
            let sample_rate = config.sample_rate().0;
            let channels = config.channels() as usize;

            let mut synth = Synthesizer::new(sample_rate, state_clone);

            let stream = device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    synth.fill_buffer(data, channels);
                },
                |err| eprintln!("Audio error: {}", err),
                None,
            )?;

            stream.play()?;

            Ok(Self {
                state,
                _stream: stream,
            })
        }

        #[cfg(not(feature = "audio"))]
        {
            // Simulation thread for when audio is disabled
            let handle = std::thread::spawn(move || {
                let mut synth = Synthesizer::new(SAMPLE_RATE, state_clone);
                // Buffer for 10ms of audio
                let buffer_size = (SAMPLE_RATE / 100) as usize;
                let mut buffer = vec![0.0; buffer_size];

                loop {
                    synth.fill_buffer(&mut buffer, 1);
                    std::thread::sleep(Duration::from_millis(10));
                }
            });

            Ok(Self {
                state,
                _thread: handle,
            })
        }
    }
}

struct Synthesizer {
    sample_rate: u32,
    state: Arc<GrooveState>,
    current_sample: u64,

    // Instrument States
    kick_phase: Option<f32>,
    snare_phase: Option<f32>,
    hat_phase: Option<f32>,

    // Sequencer
    next_kick_sample: u64,
    next_snare_sample: u64,
    next_hat_sample: u64,
}

impl Synthesizer {
    fn new(sample_rate: u32, state: Arc<GrooveState>) -> Self {
        Self {
            sample_rate,
            state,
            current_sample: 0,
            kick_phase: None,
            snare_phase: None,
            hat_phase: None,
            next_kick_sample: 0,
            next_snare_sample: 0,
            next_hat_sample: 0,
        }
    }

    fn fill_buffer(&mut self, buffer: &mut [f32], channels: usize) {
        for frame in buffer.chunks_mut(channels) {
            let sample = self.next_sample();
            for channel in frame.iter_mut() {
                *channel = sample;
            }
        }
    }

    fn next_sample(&mut self) -> f32 {
        self.current_sample += 1;
        // let t = self.current_sample as f64 / self.sample_rate as f64; // Unused for now

        // Sequencer Logic
        let samples_per_beat = (self.sample_rate as f64 * 60.0 / BPM) as u64;
        let beat_duration_samples = samples_per_beat;

        // Latency offsets (Swing)
        let kick_offset = (self.state.get_kick_latency().as_secs_f64() * self.sample_rate as f64) as u64;
        let snare_offset = (self.state.get_snare_latency().as_secs_f64() * self.sample_rate as f64) as u64;
        let hat_offset = (self.state.get_hat_latency().as_secs_f64() * self.sample_rate as f64) as u64;

        // Trigger Logic
        if self.current_sample >= self.next_kick_sample {
            self.kick_phase = Some(0.0);
            self.state.increment_beat();

            // Calculate next kick time: +2 beats
            let next_grid = self.next_kick_sample - kick_offset + (beat_duration_samples * 2);
            let new_latency = (self.state.get_kick_latency().as_secs_f64() * self.sample_rate as f64) as u64;
            self.next_kick_sample = next_grid + new_latency;

            if self.next_kick_sample < self.current_sample {
                 self.next_kick_sample = self.current_sample + beat_duration_samples;
            }
        }

        if self.current_sample >= self.next_snare_sample {
             self.snare_phase = Some(0.0);
             // Snare +2 beats
             let next_grid = self.next_snare_sample - snare_offset + (beat_duration_samples * 2);
             let new_latency = (self.state.get_snare_latency().as_secs_f64() * self.sample_rate as f64) as u64;
             self.next_snare_sample = next_grid + new_latency;

             if self.next_snare_sample < self.current_sample {
                 self.next_snare_sample = self.current_sample + beat_duration_samples;
             }
        }

        if self.current_sample >= self.next_hat_sample {
             self.hat_phase = Some(0.0);
             // Hat +0.5 beats
             let next_grid = self.next_hat_sample - hat_offset + (beat_duration_samples / 2);
             let new_latency = (self.state.get_hat_latency().as_secs_f64() * self.sample_rate as f64) as u64;
             self.next_hat_sample = next_grid + new_latency;

              if self.next_hat_sample < self.current_sample {
                 self.next_hat_sample = self.current_sample + (beat_duration_samples / 2);
             }
        }

        // Initialize logic for start (hacky fix for 0 initialization)
        if self.current_sample == 1 {
            self.next_kick_sample = beat_duration_samples * 0 + kick_offset; // Beat 1
            self.next_snare_sample = beat_duration_samples * 1 + snare_offset; // Beat 2
            self.next_hat_sample = beat_duration_samples * 0 + hat_offset; // Beat 1
        }

        // Synthesis
        let mut out = 0.0;

        // Kick
        if let Some(p) = self.kick_phase {
            let freq = 150.0 * (-p * 5.0).exp();
            let amp = (-p * 5.0).exp();
            out += (p * freq * 2.0 * std::f32::consts::PI).sin() * amp * 0.8;

            self.kick_phase = Some(p + 1.0/self.sample_rate as f32);
            if p > 0.5 { self.kick_phase = None; }
        }

        // Snare
        if let Some(p) = self.snare_phase {
            let noise: f32 = rand::thread_rng().gen_range(-1.0..1.0);
            let tone = (p * 200.0 * 2.0 * std::f32::consts::PI).sin();
            let amp = (-p * 15.0).exp();
            out += (noise * 0.5 + tone * 0.5) * amp * 0.6;

            self.snare_phase = Some(p + 1.0/self.sample_rate as f32);
            if p > 0.3 { self.snare_phase = None; }
        }

        // Hat
        if let Some(p) = self.hat_phase {
            let noise: f32 = rand::thread_rng().gen_range(-1.0..1.0);
            let amp = (-p * 40.0).exp();
            out += noise * amp * 0.3;

            self.hat_phase = Some(p + 1.0/self.sample_rate as f32);
            if p > 0.1 { self.hat_phase = None; }
        }

        out.clamp(-1.0, 1.0)
    }
}
