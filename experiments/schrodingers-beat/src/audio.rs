use crossbeam::channel::Receiver;
use std::thread;

#[cfg(feature = "audio")]
use anyhow::Context;
#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "audio")]
use std::f32::consts::PI;
#[cfg(feature = "audio")]
use std::sync::{Arc, Mutex};

#[derive(Debug, Clone, Copy)]
pub enum SoundEvent {
    LockAcquired(usize),
    LockReleased(usize),
    Contention(usize),
}

pub struct AudioEngine {
    #[cfg(feature = "audio")]
    _stream: cpal::Stream,
}

#[cfg(feature = "audio")]
struct SynthState {
    phase: f32,
    active_notes: Vec<(SoundEvent, f32)>,
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
                phase: 0.0,
                active_notes: Vec::new(),
            }));

            let state_cb = state.clone();
            let stream = match config.sample_format() {
                cpal::SampleFormat::F32 => device.build_output_stream(
                    &config.into(),
                    move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                        write_data(data, channels, &state_cb, sample_rate, &rx)
                    },
                    |err| eprintln!("an error occurred on stream: {}", err),
                    None,
                )?,
                _ => return Err(anyhow::anyhow!("Unsupported sample format")),
            };

            stream.play()?;

            Ok(Self { _stream: stream })
        }
        #[cfg(not(feature = "audio"))]
        {
            // Drain the channel to prevent memory leak
            thread::spawn(move || while let Ok(_) = rx.recv() {});
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
            let event_val = *event; // Deref to value

            let amp = match event_val {
                SoundEvent::LockAcquired(id) => {
                    let env = (-5.0 * t).exp();
                    match id % 4 {
                        0 => { // Kick
                            let freq = 60.0 * (-10.0 * t).exp().max(0.1);
                            (t * freq * 2.0 * PI).sin() * env * 2.0
                        },
                        1 => { // African Bell (Cowbell-ish)
                            let freq = 800.0;
                            let bell_env = (-15.0 * t).exp();
                            ((t * freq * 2.0 * PI).sin() + (t * freq * 1.5 * 2.0 * PI).sin()) * 0.5 * bell_env
                        },
                        2 => { // Snare-ish
                            let noise = (rand::random::<f32>() * 2.0 - 1.0);
                            let snare_env = (-20.0 * t).exp();
                            let tone = (t * 200.0 * 2.0 * PI).sin() * (-10.0 * t).exp();
                            (noise * 0.7 + tone * 0.3) * snare_env
                        },
                        _ => { // High Perc
                            let freq = 1200.0;
                            let perc_env = (-30.0 * t).exp();
                            (t * freq * 2.0 * PI).sin() * perc_env * 0.5
                        }
                    }
                }
                SoundEvent::LockReleased(_id) => {
                    let noise = (rand::random::<f32>() * 2.0 - 1.0);
                    let env = (-50.0 * t).exp();
                    noise * env * 0.05
                }
                SoundEvent::Contention(id) => {
                    // Dissonant "Clash" sound
                    let base_freq = 110.0 * ((id + 1) as f32);
                    let freq = base_freq + (t * 500.0).sin() * 50.0; // FM Modulation
                    let phase = (t * freq).fract();
                    let osc = phase * 2.0 - 1.0; // Sawtooth-ish
                    let env = (-5.0 * t).exp();
                    osc * env * 0.3
                }
            };

            sample += amp;

            match event_val {
                SoundEvent::LockAcquired(_) => t < 0.5,
                SoundEvent::LockReleased(_) => t < 0.1,
                SoundEvent::Contention(_) => t < 0.3,
            }
        });

        // Hard clip / Limiter
        sample = sample.clamp(-1.0, 1.0);

        for channel_sample in frame.iter_mut() {
            *channel_sample = sample;
        }

        s.phase = (s.phase + 440.0 / sample_rate) % 1.0;
    }
}
