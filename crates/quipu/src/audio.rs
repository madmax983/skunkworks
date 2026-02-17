use anyhow::Result;
use crossbeam_channel::{bounded, Sender};
use std::any::Any;

#[derive(Debug, Clone, Copy)]
#[allow(dead_code)]
pub enum AudioEvent {
    Kick,
    Snare,
    HiHat,
    Pluck(f32), // Frequency
}

pub struct AudioEngine {
    _stream: Option<Box<dyn Any>>, // Keep stream alive
    tx: Sender<AudioEvent>,
}

#[cfg(not(feature = "audio"))]
impl AudioEngine {
    pub fn new() -> Result<Self> {
        let (tx, _) = bounded(1024);
        Ok(Self { _stream: None, tx })
    }

    pub fn get_sender(&self) -> Sender<AudioEvent> {
        self.tx.clone()
    }
}

#[cfg(feature = "audio")]
use crossbeam_channel::Receiver;

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

#[cfg(feature = "audio")]
struct ActiveSound {
    kind: SoundKind,
    t: f32, // Time in seconds
    amp: f32,
}

#[cfg(feature = "audio")]
enum SoundKind {
    Kick,
    Snare,
    HiHat,
    Pluck(f32),
}

#[cfg(feature = "audio")]
impl AudioEngine {
    pub fn new() -> Result<Self> {
        let host = cpal::default_host();
        let device = match host.default_output_device() {
            Some(d) => d,
            None => {
                eprintln!("No audio output device found. Running in silent mode.");
                let (tx, _) = bounded(1024);
                return Ok(Self { _stream: None, tx });
            }
        };

        let config = device.default_output_config()?;
        let sample_rate = config.sample_rate().0 as f32;
        let channels = config.channels() as usize;

        let (tx, rx) = bounded::<AudioEvent>(1024);

        let mut active_sounds: Vec<ActiveSound> = Vec::with_capacity(64);

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    process_audio(data, channels, sample_rate, &rx, &mut active_sounds);
                },
                err_fn,
                None,
            )?,
            _ => return Err(anyhow::anyhow!("Unsupported sample format")),
        };

        stream.play()?;

        Ok(Self {
            _stream: Some(Box::new(stream)),
            tx,
        })
    }

    pub fn get_sender(&self) -> Sender<AudioEvent> {
        self.tx.clone()
    }
}

#[cfg(feature = "audio")]
fn process_audio(
    output: &mut [f32],
    channels: usize,
    sample_rate: f32,
    rx: &Receiver<AudioEvent>,
    active_sounds: &mut Vec<ActiveSound>,
) {
    // 1. Process new events
    while let Ok(event) = rx.try_recv() {
        match event {
            AudioEvent::Kick => active_sounds.push(ActiveSound {
                kind: SoundKind::Kick,
                t: 0.0,
                amp: 0.8,
            }),
            AudioEvent::Snare => active_sounds.push(ActiveSound {
                kind: SoundKind::Snare,
                t: 0.0,
                amp: 0.6,
            }),
            AudioEvent::HiHat => active_sounds.push(ActiveSound {
                kind: SoundKind::HiHat,
                t: 0.0,
                amp: 0.4,
            }),
            AudioEvent::Pluck(f) => active_sounds.push(ActiveSound {
                kind: SoundKind::Pluck(f),
                t: 0.0,
                amp: 0.5,
            }),
        }
    }

    // 2. Generate audio
    for frame in output.chunks_mut(channels) {
        let mut sample = 0.0;
        let dt = 1.0 / sample_rate;

        // Retain only sounds that are still active
        active_sounds.retain_mut(|sound| {
            sound.t += dt;

            let s = match sound.kind {
                SoundKind::Kick => {
                    // Sine sweep 120 -> 40 Hz
                    let freq = 120.0 - (80.0 * (sound.t * 8.0).min(1.0));
                    let env = (1.0 - sound.t * 5.0).max(0.0);
                    if env <= 0.0 {
                        return false;
                    }
                    (sound.t * freq * 2.0 * std::f32::consts::PI).sin() * env * sound.amp
                }
                SoundKind::Snare => {
                    // Noise + Tone
                    let noise =
                        (rand::random::<f32>() * 2.0 - 1.0) * (1.0 - sound.t * 10.0).max(0.0);
                    let tone = (sound.t * 180.0 * 2.0 * std::f32::consts::PI).sin()
                        * (1.0 - sound.t * 6.0).max(0.0);
                    let val = noise * 0.8 + tone * 0.2;
                    if sound.t > 0.15 {
                        return false;
                    }
                    val * sound.amp
                }
                SoundKind::HiHat => {
                    // High freq noise
                    let noise = (rand::random::<f32>() * 2.0 - 1.0);
                    let env = (1.0 - sound.t * 30.0).max(0.0);
                    if env <= 0.0 {
                        return false;
                    }
                    noise * env * sound.amp
                }
                SoundKind::Pluck(freq) => {
                    // Karplus-Strong-ish or just simple plucked string (sine w/ exp decay)
                    let val = (sound.t * freq * 2.0 * std::f32::consts::PI).sin();
                    let env = (-sound.t * 4.0).exp();
                    if env <= 0.001 {
                        return false;
                    }
                    val * env * sound.amp
                }
            };

            sample += s;
            true
        });

        // Soft clipper
        sample = sample.max(-0.9).min(0.9);

        for channel in frame.iter_mut() {
            *channel = sample;
        }
    }
}
