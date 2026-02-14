#[cfg(feature = "audio")]
use crossbeam_channel::Receiver;
use crossbeam_channel::Sender;
use std::cmp::Ordering;
use std::sync::Arc;

#[derive(Debug, Clone, Copy, PartialEq, Eq)]
pub enum Voice {
    Kick,
    Snare,
    Hihat,
    Clave,
    Synth(u32),
}

#[derive(Debug, Clone)]
pub struct RhythmEvent {
    pub timestamp: u64,
    #[allow(dead_code)]
    pub voice: Voice,
    #[allow(dead_code)]
    pub volume: f32,
    #[allow(dead_code)]
    pub source_id: usize,
}

impl Ord for RhythmEvent {
    fn cmp(&self, other: &Self) -> Ordering {
        other.timestamp.cmp(&self.timestamp)
    }
}
impl PartialOrd for RhythmEvent {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        Some(self.cmp(other))
    }
}
impl PartialEq for RhythmEvent {
    fn eq(&self, other: &Self) -> bool {
        self.timestamp == other.timestamp
    }
}
impl Eq for RhythmEvent {}

pub struct AudioEngine {
    #[allow(dead_code)]
    sample_rate: u32,
    #[cfg(feature = "audio")]
    _stream: cpal::Stream,
    #[cfg(not(feature = "audio"))]
    _simulation_thread: std::thread::JoinHandle<()>,
}

impl AudioEngine {
    pub fn sample_rate(&self) -> u32 {
        self.sample_rate
    }

    #[cfg(feature = "audio")]
    pub fn new() -> anyhow::Result<(Self, Sender<RhythmEvent>, Arc<std::sync::atomic::AtomicU64>)> {
        use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
        use rand::Rng;
        use std::collections::BinaryHeap;
        use std::f32::consts::TAU;

        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No audio output device found"))?;
        let config = device.default_output_config()?;
        let sample_rate = config.sample_rate().0;
        let channels = config.channels() as usize;

        let (tx, rx) = crossbeam_channel::unbounded();
        let current_sample_atomic = Arc::new(std::sync::atomic::AtomicU64::new(0));
        let current_sample_atomic_clone = current_sample_atomic.clone();

        struct ActiveVoice {
            voice: Voice,
            start_sample: u64,
            volume: f32,
            phase: f32,
        }

        struct AudioState {
            current_sample: u64,
            events: BinaryHeap<RhythmEvent>,
            receiver: Receiver<RhythmEvent>,
            active_voices: Vec<ActiveVoice>,
            sample_rate: f32,
        }

        let mut state = AudioState {
            current_sample: 0,
            events: BinaryHeap::new(),
            receiver: rx,
            active_voices: Vec::new(),
            sample_rate: sample_rate as f32,
        };

        fn synthesize_voice(v: &mut ActiveVoice, age: f32, sample_rate: f32) -> (f32, bool) {
            match v.voice {
                Voice::Kick => {
                    let freq = 150.0 * (-age * 20.0).exp().max(0.3);
                    v.phase += freq / sample_rate * TAU;
                    if v.phase > TAU {
                        v.phase -= TAU;
                    }

                    let amp = (-age * 5.0).exp();
                    let signal = v.phase.sin();
                    // Add some click
                    let click = if age < 0.005 {
                        (rand::random::<f32>() * 2.0 - 1.0) * 0.5
                    } else {
                        0.0
                    };

                    ((signal + click) * amp * v.volume, amp > 0.001)
                }
                Voice::Snare => {
                    let amp = (-age * 15.0).exp();
                    let tone_freq = 180.0;
                    v.phase += tone_freq / sample_rate * TAU;
                    if v.phase > TAU {
                        v.phase -= TAU;
                    }
                    let tone = v.phase.sin();
                    let noise = rand::random::<f32>() * 2.0 - 1.0;

                    ((tone * 0.3 + noise * 0.7) * amp * v.volume, amp > 0.001)
                }
                Voice::Hihat => {
                    let amp = (-age * 40.0).exp();
                    let noise = rand::random::<f32>() * 2.0 - 1.0;
                    (noise * amp * v.volume * 0.5, amp > 0.001)
                }
                Voice::Clave => {
                    let amp = (-age * 30.0).exp();
                    let freq = 2500.0;
                    v.phase += freq / sample_rate * TAU;
                    if v.phase > TAU {
                        v.phase -= TAU;
                    }
                    (v.phase.sin() * amp * v.volume * 0.3, amp > 0.001)
                }
                Voice::Synth(note) => {
                    let amp = (-age * 3.0).exp();
                    let freq = 220.0 * (2.0f32).powf(note as f32 / 12.0);
                    v.phase += freq / sample_rate * TAU;
                    if v.phase > TAU {
                        v.phase -= TAU;
                    }

                    let mod_idx = 2.0 * (-age).exp();
                    let signal = (v.phase + (v.phase * 2.0).sin() * mod_idx).sin();

                    (signal * amp * v.volume * 0.4, amp > 0.001)
                }
            }
        }

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config.into(),
                move |output: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    // Receive new events
                    while let Ok(event) = state.receiver.try_recv() {
                        state.events.push(event);
                    }

                    for frame in output.chunks_mut(channels) {
                        let now = state.current_sample;

                        // Trigger events scheduled for 'now'
                        while let Some(evt) = state.events.peek() {
                            if evt.timestamp <= now {
                                let evt = state.events.pop().unwrap();
                                state.active_voices.push(ActiveVoice {
                                    voice: evt.voice,
                                    start_sample: now,
                                    volume: evt.volume,
                                    phase: 0.0,
                                });
                            } else {
                                break;
                            }
                        }

                        // Mix voices
                        let mut mix_sample = 0.0;
                        let sr = state.sample_rate;

                        state.active_voices.retain_mut(|v| {
                            let age = (now - v.start_sample) as f32 / sr;
                            let (s, keep) = synthesize_voice(v, age, sr);
                            mix_sample += s;
                            keep
                        });

                        mix_sample = mix_sample.tanh();

                        for sample_out in frame.iter_mut() {
                            *sample_out = mix_sample;
                        }

                        state.current_sample += 1;
                    }

                    current_sample_atomic_clone
                        .store(state.current_sample, std::sync::atomic::Ordering::Relaxed);
                },
                err_fn,
                None,
            )?,
            _ => return Err(anyhow::anyhow!("Unsupported sample format")),
        };

        stream.play()?;

        Ok((
            AudioEngine {
                sample_rate,
                _stream: stream,
            },
            tx,
            current_sample_atomic,
        ))
    }

    #[cfg(not(feature = "audio"))]
    pub fn new() -> anyhow::Result<(Self, Sender<RhythmEvent>, Arc<std::sync::atomic::AtomicU64>)> {
        let (tx, rx) = crossbeam_channel::unbounded();
        let current_sample_atomic = Arc::new(std::sync::atomic::AtomicU64::new(0));
        let current_sample_atomic_clone = current_sample_atomic.clone();
        let sample_rate = 44100;

        let handle = std::thread::spawn(move || {
            let start = std::time::Instant::now();
            loop {
                std::thread::sleep(std::time::Duration::from_millis(16));
                let elapsed = start.elapsed().as_secs_f64();
                let samples = (elapsed * sample_rate as f64) as u64;
                current_sample_atomic_clone.store(samples, std::sync::atomic::Ordering::Relaxed);

                // Drain events so the channel doesn't fill up memory
                while let Ok(_) = rx.try_recv() {}
            }
        });

        Ok((
            AudioEngine {
                sample_rate,
                _simulation_thread: handle,
            },
            tx,
            current_sample_atomic,
        ))
    }
}
