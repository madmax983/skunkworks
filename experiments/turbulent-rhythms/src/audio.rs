use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam::channel::Receiver;
use std::collections::HashMap;
use std::sync::{Arc, Mutex};

#[derive(Debug, PartialEq)]
pub enum AudioEvent {
    NoteOn { id: usize, freq: f32 },
    NoteOff { id: usize },
}

struct Voice {
    freq: f32,
    phase: f32,
    envelope: f32,
    target_envelope: f32,
}

impl Voice {
    fn new(freq: f32) -> Self {
        Self {
            freq,
            phase: 0.0,
            envelope: 0.0,
            target_envelope: 1.0,
        }
    }
}

pub struct AudioEngine {
    #[allow(dead_code)] // Keep stream alive
    stream: cpal::Stream,
}

impl AudioEngine {
    pub fn new(rx: Receiver<AudioEvent>) -> anyhow::Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No output device available"))?;
        let config = device.default_output_config()?;

        let sample_rate = config.sample_rate().0 as f32;
        let channels = config.channels() as usize;

        let active_voices: Arc<Mutex<HashMap<usize, Voice>>> = Arc::new(Mutex::new(HashMap::new()));
        let active_voices_clone = active_voices.clone();

        // Audio processing thread (outside the callback to handle events)
        std::thread::spawn(move || {
            while let Ok(event) = rx.recv() {
                let mut voices = active_voices_clone.lock().unwrap();
                match event {
                    AudioEvent::NoteOn { id, freq } => {
                        voices.insert(id, Voice::new(freq));
                    }
                    AudioEvent::NoteOff { id } => {
                        if let Some(voice) = voices.get_mut(&id) {
                            voice.target_envelope = 0.0;
                        }
                    }
                }
            }
        });

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    write_data(data, channels, &active_voices, sample_rate)
                },
                err_fn,
                None,
            )?,
            _ => return Err(anyhow::anyhow!("Unsupported sample format")),
        };

        stream.play()?;

        Ok(Self { stream })
    }
}

fn write_data(
    output: &mut [f32],
    channels: usize,
    active_voices: &Arc<Mutex<HashMap<usize, Voice>>>,
    sample_rate: f32,
) {
    let mut voices = active_voices.lock().unwrap();

    for frame in output.chunks_mut(channels) {
        let mut sample = 0.0;

        let mut dead_ids = Vec::new();
        for (id, voice) in voices.iter_mut() {
            // Simple Sine Wave
            sample += (voice.phase * 2.0 * std::f32::consts::PI).sin() * voice.envelope * 0.1;

            voice.phase += voice.freq / sample_rate;
            if voice.phase > 1.0 {
                voice.phase -= 1.0;
            }

            let attack_speed = 0.05;
            let release_speed = 0.005;

            if voice.target_envelope > voice.envelope {
                voice.envelope += attack_speed;
                if voice.envelope > voice.target_envelope {
                    voice.envelope = voice.target_envelope;
                }
            } else if voice.target_envelope < voice.envelope {
                voice.envelope -= release_speed;
                if voice.envelope < voice.target_envelope {
                    voice.envelope = voice.target_envelope;
                }
            }

            if voice.target_envelope == 0.0 && voice.envelope <= 0.001 {
                dead_ids.push(*id);
            }
        }

        for id in dead_ids {
            voices.remove(&id);
        }

        for sample_out in frame.iter_mut() {
            *sample_out = sample;
        }
    }
}
