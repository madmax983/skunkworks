use crossbeam::queue::SegQueue;
use std::sync::Arc;

#[derive(Debug, Clone, Copy)]
pub enum Note {
    Kick,
    Snare,
    HatClosed,
    HatOpen,
    Clap,
}

pub struct AudioEngine {
    #[cfg(feature = "audio")]
    _stream: cpal::Stream,
    pub event_queue: Arc<SegQueue<Note>>,
}

#[cfg(not(feature = "audio"))]
impl AudioEngine {
    pub fn new() -> anyhow::Result<Self> {
        Ok(Self { event_queue: Arc::new(SegQueue::new()) })
    }
}

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

#[cfg(feature = "audio")]
impl AudioEngine {
    pub fn new() -> anyhow::Result<Self> {
        let event_queue = Arc::new(SegQueue::new());
        let host = cpal::default_host();
        let device = host.default_output_device().ok_or_else(|| anyhow::anyhow!("No audio device available"))?;
        let config = device.default_output_config()?;
        let sample_rate = config.sample_rate().0 as f32;
        let channels = config.channels() as usize;

        let queue_clone = event_queue.clone();

        let mut synth_state = SynthState::new(sample_rate);

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _| write_data(data, channels, &queue_clone, &mut synth_state),
                err_fn,
                None,
            )?,
            _ => return Err(anyhow::anyhow!("Unsupported sample format")),
        };
        stream.play()?;

        Ok(Self {
            _stream: stream,
            event_queue,
        })
    }
}

#[cfg(feature = "audio")]
struct SynthState {
    sample_rate: f32,
    active_notes: Vec<ActiveNote>,
    rng_state: u32,
}

#[cfg(feature = "audio")]
struct ActiveNote {
    note_type: Note,
    phase: f32,
    envelope: f32,
    duration: f32,
}

#[cfg(feature = "audio")]
impl SynthState {
    fn new(sample_rate: f32) -> Self {
        Self {
            sample_rate,
            active_notes: Vec::with_capacity(64),
            rng_state: 12345,
        }
    }

    // Removed next_random method to avoid borrow issues
}

#[cfg(feature = "audio")]
fn next_random(state: &mut u32) -> f32 {
    let mut x = *state;
    x ^= x << 13;
    x ^= x >> 17;
    x ^= x << 5;
    *state = x;
    (x as f32 / u32::MAX as f32) * 2.0 - 1.0
}

#[cfg(feature = "audio")]
fn write_data(output: &mut [f32], channels: usize, queue: &SegQueue<Note>, state: &mut SynthState) {
    for frame in output.chunks_mut(channels) {
        // Process new events (limit to prevent infinite loop if spamming)
        let mut events_processed = 0;
        while let Some(note) = queue.pop() {
            let (duration, envelope) = match note {
                Note::Kick => (0.5, 1.0),
                Note::Snare => (0.3, 0.8),
                Note::HatClosed => (0.05, 0.4),
                Note::HatOpen => (0.3, 0.5),
                Note::Clap => (0.2, 0.7),
            };

            state.active_notes.push(ActiveNote {
                note_type: note,
                phase: 0.0,
                envelope,
                duration,
            });
            events_processed += 1;
            if events_processed > 10 { break; }
        }

        let mut sample = 0.0;
        let dt = 1.0 / state.sample_rate;

        // Split borrow
        let rng_state = &mut state.rng_state;
        let active_notes = &mut state.active_notes;

        // Process active notes
        active_notes.retain_mut(|note| {
            note.phase += dt;
            note.envelope -= dt / note.duration; // Linear decay for now

            if note.envelope <= 0.0 {
                return false;
            }

            let s = match note.note_type {
                Note::Kick => {
                    // Sine sweep: 150Hz -> 50Hz
                    let freq_mod = (1.0 - note.phase / note.duration).max(0.0).powi(2);
                    let freq = 50.0 + 100.0 * freq_mod;
                    (note.phase * freq * 2.0 * std::f32::consts::PI).sin() * note.envelope
                },
                Note::Snare => {
                    // White noise + Sine
                    let noise = next_random(rng_state);
                    let tone = (note.phase * 180.0 * 2.0 * std::f32::consts::PI).sin();
                    (noise * 0.8 + tone * 0.2) * note.envelope
                },
                Note::HatClosed => {
                    // High pass noise (simulated by simple noise)
                    next_random(rng_state) * note.envelope
                },
                Note::HatOpen => {
                     next_random(rng_state) * note.envelope
                },
                Note::Clap => {
                    // Burst of noise
                    let noise = next_random(rng_state);
                    noise * note.envelope
                }
            };

            sample += s;
            true
        });

        // Soft clipper
        sample = sample.tanh();

        for channel in frame {
            *channel = sample;
        }
    }
}
