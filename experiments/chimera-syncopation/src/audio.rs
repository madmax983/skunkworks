use crate::model::AudioCommand;
use crossbeam_channel::{Receiver, TryRecvError};
use hound::{WavSpec, WavWriter};
use std::f32::consts::PI;
use std::thread;
use std::time::{Duration, Instant};

enum Drum {
    Kick {
        phase: f32,
        frequency: f32,
        envelope: f32,
    },
    Snare {
        envelope: f32,
    },
    Hat {
        envelope: f32,
    },
}

impl Drum {
    fn next_sample(&mut self) -> Option<f32> {
        match self {
            Drum::Kick {
                phase,
                frequency,
                envelope,
            } => {
                if *envelope < 0.001 {
                    return None;
                }
                *phase += *frequency / 44100.0 * 2.0 * PI;
                let sample = phase.sin() * *envelope;
                *frequency *= 0.999; // Pitch drop
                *envelope *= 0.9995;
                Some(sample)
            }
            Drum::Snare { envelope } => {
                if *envelope < 0.001 {
                    return None;
                }
                let noise = (rand::random::<f32>() * 2.0 - 1.0) * *envelope;
                *envelope *= 0.995;
                Some(noise)
            }
            Drum::Hat { envelope } => {
                if *envelope < 0.001 {
                    return None;
                }
                let noise = (rand::random::<f32>() * 2.0 - 1.0) * *envelope; // High pass filter would be better but simple noise is ok
                *envelope *= 0.9; // Fast decay
                Some(noise)
            }
        }
    }
}

pub fn start_audio_thread(receiver: Receiver<AudioCommand>) -> thread::JoinHandle<()> {
    thread::spawn(move || {
        let spec = WavSpec {
            channels: 1,
            sample_rate: 44100,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer =
            WavWriter::create("chimera_syncopation.wav", spec).expect("Failed to create WAV file");

        let mut active_sounds: Vec<Drum> = Vec::new();
        let start_time = Instant::now();
        let mut samples_written = 0;
        let sample_rate = 44100;

        loop {
            // Check for new commands
            loop {
                match receiver.try_recv() {
                    Ok(cmd) => match cmd {
                        AudioCommand::Play(0) => active_sounds.push(Drum::Kick {
                            phase: 0.0,
                            frequency: 150.0,
                            envelope: 1.0,
                        }),
                        AudioCommand::Play(1) => active_sounds.push(Drum::Snare { envelope: 1.0 }),
                        AudioCommand::Play(2) => active_sounds.push(Drum::Hat { envelope: 0.5 }),
                        AudioCommand::Play(_) => {} // Ignore unknown
                        AudioCommand::Stop => return,
                    },
                    Err(TryRecvError::Empty) => break,
                    Err(TryRecvError::Disconnected) => return,
                }
            }

            // Calculate how many samples we should have written by now
            let elapsed = start_time.elapsed().as_secs_f64();
            let target_samples = (elapsed * sample_rate as f64) as u64;
            let samples_to_write = target_samples.saturating_sub(samples_written);

            if samples_to_write > 0 {
                for _ in 0..samples_to_write {
                    let mut sample = 0.0;
                    // Mix active sounds
                    let mut i = 0;
                    while i < active_sounds.len() {
                        if let Some(s) = active_sounds[i].next_sample() {
                            sample += s;
                            i += 1;
                        } else {
                            active_sounds.swap_remove(i);
                        }
                    }

                    // Hard clipper
                    sample = sample.clamp(-1.0, 1.0);

                    writer
                        .write_sample((sample * i16::MAX as f32) as i16)
                        .unwrap();
                }
                samples_written += samples_to_write;
            }

            thread::sleep(Duration::from_millis(1));
        }
    })
}
