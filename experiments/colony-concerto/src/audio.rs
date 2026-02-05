use crossbeam::channel::Receiver;
use std::thread;

#[cfg(feature = "audio")]
use anyhow::Context;
#[cfg(feature = "audio")]
use std::sync::{Arc, Mutex};
#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "audio")]
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub enum SoundEvent {
    #[allow(dead_code)]
    BuildStart(usize), // frequency based on node index
    Contention,
    BuildComplete,
}

pub struct AudioEngine {
    #[cfg(feature = "audio")]
    _stream: cpal::Stream,
}

#[cfg(feature = "audio")]
struct SynthState {
    active_notes: Vec<(SoundEvent, f32)>, // Event, Time alive
}

impl AudioEngine {
    pub fn new(rx: Receiver<SoundEvent>) -> anyhow::Result<Self> {
        #[cfg(feature = "audio")]
        {
            let host = cpal::default_host();
            let device = host
                .default_output_device()
                .context("failed to find output device")?;
            let config = device.default_output_config()?;

            let sample_rate = config.sample_rate().0 as f32;
            let channels = config.channels() as usize;

            let state = Arc::new(Mutex::new(SynthState {
                active_notes: Vec::with_capacity(32),
            }));

            let state_cb = state.clone();
            // We need to move rx into the closure, but we can't because it's not Clone (Receiver is not Clone in std, but crossbeam Receiver IS Clone).
            // Wait, crossbeam Receiver IS Clone.
            let rx_cb = rx.clone();

            let stream = match config.sample_format() {
                cpal::SampleFormat::F32 => device.build_output_stream(
                    &config.into(),
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        write_data(data, channels, &state_cb, sample_rate, &rx_cb)
                    },
                    |err| eprintln!("an error occurred on stream: {}", err),
                    None,
                )?,
                _ => return Err(anyhow::anyhow!("Unsupported sample format")),
            }?;

            stream.play()?;

            Ok(Self { _stream: stream })
        }
        #[cfg(not(feature = "audio"))]
        {
            // Drain the channel to prevent memory leak
            thread::spawn(move || {
                while let Ok(_) = rx.recv() {}
            });
            println!("Audio disabled (missing 'audio' feature). Running in TUI-only mode.");
            Ok(Self {})
        }
    }
}

#[cfg(feature = "audio")]
fn write_data(
    output: &mut [f32],
    channels: usize,
    state: &Arc<Mutex<SynthState>>,
    sample_rate: f32,
    rx: &Receiver<SoundEvent>,
) {
    let mut s = state.lock().unwrap();

    // Check for new events
    while let Ok(event) = rx.try_recv() {
        s.active_notes.push((event, 0.0));
    }

    for frame in output.chunks_mut(channels) {
        let mut sample = 0.0;

        // Process active notes
        s.active_notes.retain_mut(|(event, time)| {
            *time += 1.0 / sample_rate;
            let t = *time;

            let amp = match event {
                SoundEvent::BuildStart(idx) => {
                    // Map idx to frequency (pentatonic scale or something)
                    let base_freq = 220.0;
                    let semitones = (*idx % 12) as f32; // Simple chromatic for now
                    let freq = base_freq * 2.0f32.powf(semitones / 12.0);

                    let env = (-5.0 * t).exp();
                    (t * freq * 2.0 * PI).sin() * env * 0.3
                }
                SoundEvent::Contention => {
                    // Dissonant buzz
                    let freq = 110.0;
                    let phase = (t * freq).fract(); // Sawtooth-ish
                    let osc = phase * 2.0 - 1.0;
                    let env = (-10.0 * t).exp();
                    osc * env * 0.2
                }
                SoundEvent::BuildComplete => {
                    // High ping
                    let freq = 880.0;
                    let env = (-20.0 * t).exp();
                    (t * freq * 2.0 * PI).sin() * env * 0.1
                }
            };

            sample += amp;

            // Cleanup duration
            match event {
                SoundEvent::BuildStart(_) => t < 1.0,
                SoundEvent::Contention => t < 0.5,
                SoundEvent::BuildComplete => t < 0.2,
            }
        });

        // Hard clipper
        sample = sample.max(-0.8).min(0.8);

        for channel_sample in frame.iter_mut() {
            *channel_sample = sample;
        }
    }
}
