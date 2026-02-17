use anyhow::Result;
use crossbeam_channel::{bounded, Sender};

#[derive(Debug, PartialEq, Clone, Copy)]
pub enum AudioEvent {
    LockAttempt(usize),
    LockAcquired(usize),
    LockReleased(usize),
    Blocked(usize),
    WorkTick(usize),
}

pub struct AudioEngine {
    #[cfg(feature = "audio")]
    _stream: Option<Box<dyn Any>>, // Keep stream alive
    tx: Sender<AudioEvent>,
}

impl AudioEngine {
    #[cfg(not(feature = "audio"))]
    pub fn new() -> Result<Self> {
        let (tx, _) = bounded(1024);
        Ok(Self { tx })
    }

    #[cfg(feature = "audio")]
    pub fn new() -> Result<Self> {
        use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
        use cpal::{Stream, StreamConfig};
        // use std::sync::{Arc, Mutex}; // Unused

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

        // Active Sounds State
        struct ActiveSound {
            kind: SoundKind,
            t: f32, // Time in seconds
            amp: f32,
            pan: f32, // -1.0 to 1.0
        }

        enum SoundKind {
            Kick, // Low sine sweep
            Snare, // Noise + Tone
            Click, // High short tick
            Buzz, // Saw wave (Blocked)
            Pluck(f32), // Frequency
        }

        let mut active_sounds = Vec::<ActiveSound>::with_capacity(64);

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
    rx: &crossbeam_channel::Receiver<AudioEvent>,
    active_sounds: &mut Vec<ActiveSound>,
) {
    use SoundKind::*;
    use AudioEvent::*;

    // 1. Process new events
    while let Ok(event) = rx.try_recv() {
        // Map Thread ID to Pan (-0.8 to 0.8)
        let thread_id = match event {
            LockAttempt(id) | LockAcquired(id) | LockReleased(id) | Blocked(id) | WorkTick(id) => id,
        };
        // Simple hash for pan
        let pan = ((thread_id % 5) as f32 / 2.0 - 1.0) * 0.8;

        match event {
            LockAttempt(_) => active_sounds.push(ActiveSound {
                kind: Click,
                t: 0.0,
                amp: 0.3,
                pan,
            }),
            LockAcquired(_) => active_sounds.push(ActiveSound {
                kind: Kick,
                t: 0.0,
                amp: 0.8,
                pan: 0.0, // Kicks centered
            }),
            LockReleased(_) => active_sounds.push(ActiveSound {
                kind: Snare,
                t: 0.0,
                amp: 0.5,
                pan: 0.0, // Snares centered
            }),
            Blocked(_) => active_sounds.push(ActiveSound {
                kind: Buzz,
                t: 0.0,
                amp: 0.4,
                pan,
            }),
            WorkTick(_) => active_sounds.push(ActiveSound {
                kind: Pluck(440.0 + (thread_id as f32 * 55.0)),
                t: 0.0,
                amp: 0.1,
                pan,
            }),
        }
    }

    // 2. Generate audio
    for frame in output.chunks_mut(channels) {
        let mut left = 0.0;
        let mut right = 0.0;
        let dt = 1.0 / sample_rate;

        // Retain only sounds that are still active
        active_sounds.retain_mut(|sound| {
            sound.t += dt;

            let s = match sound.kind {
                Kick => {
                    // Sine sweep 100 -> 40 Hz
                    let freq = 100.0 - (60.0 * (sound.t * 10.0).min(1.0));
                    let env = (1.0 - sound.t * 5.0).max(0.0);
                    if env <= 0.0 { return false; }
                    (sound.t * freq * 2.0 * std::f32::consts::PI).sin() * env * sound.amp
                }
                Snare => {
                    // Noise
                    let noise = (rand::random::<f32>() * 2.0 - 1.0);
                    let env = (1.0 - sound.t * 15.0).max(0.0);
                    if env <= 0.0 { return false; }
                    noise * env * sound.amp
                }
                Click => {
                    // High blip
                    let val = (sound.t * 2000.0 * 2.0 * std::f32::consts::PI).sin();
                    let env = (1.0 - sound.t * 100.0).max(0.0); // Very short
                    if env <= 0.0 { return false; }
                    val * env * sound.amp
                }
                Buzz => {
                    // Sawtooth low freq
                    let freq = 60.0;
                    let val = ((sound.t * freq).fract() * 2.0 - 1.0);
                    let env = (1.0 - sound.t * 4.0).max(0.0);
                    if env <= 0.0 { return false; }
                    val * env * sound.amp
                }
                Pluck(freq) => {
                    let val = (sound.t * freq * 2.0 * std::f32::consts::PI).sin();
                    let env = (-sound.t * 8.0).exp();
                    if env <= 0.01 { return false; }
                    val * env * sound.amp
                }
            };

            // Panning
            // pan = -1 (Left), 0 (Center), 1 (Right)
            // L = s * (1 - pan)/2? No.
            // Equal power panning: L = cos(p), R = sin(p)?
            // Linear approximation: L = (1-pan)/2, R = (1+pan)/2 (normalized)
            let p = (sound.pan + 1.0) / 2.0; // 0..1
            left += s * (1.0 - p).sqrt();
            right += s * p.sqrt();

            true
        });

        // Soft clipper
        left = left.max(-0.9).min(0.9);
        right = right.max(-0.9).min(0.9);

        if channels >= 2 {
            frame[0] = left;
            frame[1] = right;
        } else {
            frame[0] = (left + right) * 0.5;
        }
    }
}

// Helper types for process_audio
#[cfg(feature = "audio")]
struct ActiveSound {
    kind: SoundKind,
    t: f32,
    amp: f32,
    pan: f32,
}

#[cfg(feature = "audio")]
enum SoundKind {
    Kick,
    Snare,
    Click,
    Buzz,
    Pluck(f32),
}
