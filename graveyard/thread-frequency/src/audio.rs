use anyhow::Result;
use crossbeam::channel::{Receiver, TryRecvError};
use hound::{WavSpec, WavWriter};
use std::thread;
use std::time::{Duration, Instant};

const SAMPLE_RATE: u32 = 44100;

#[derive(Debug, Clone, Copy)]
pub enum AudioEvent {
    NoteOn { freq: f32, duration: f32 },
    NoteOff { freq: f32 },
    Contention, // Kick
}

struct Voice {
    freq: f32,
    phase: f32,
    envelope: f32,
    decay_rate: f32,
    active: bool,
    is_noise: bool,
}

impl Voice {
    fn new_note(freq: f32, duration: f32) -> Self {
        let decay_rate = 1.0 / (duration * SAMPLE_RATE as f32);
        Self {
            freq,
            phase: 0.0,
            envelope: 1.0,
            decay_rate,
            active: true,
            is_noise: false,
        }
    }

    fn new_noise() -> Self {
        let duration = 0.2;
        let decay_rate = 1.0 / (duration * SAMPLE_RATE as f32);
        Self {
            freq: 0.0,
            phase: 0.0,
            envelope: 1.0,
            decay_rate,
            active: true,
            is_noise: true,
        }
    }

    fn process(&mut self) -> f32 {
        if !self.active {
            return 0.0;
        }

        let output = if self.is_noise {
            let noise: f32 = rand::random::<f32>() * 2.0 - 1.0;
            noise * self.envelope
        } else {
            let value = (self.phase * 2.0 * std::f32::consts::PI).sin();
            self.phase = (self.phase + self.freq / SAMPLE_RATE as f32) % 1.0;
            value * self.envelope
        };

        self.envelope -= self.decay_rate;
        if self.envelope <= 0.0 {
            self.active = false;
            self.envelope = 0.0;
        }

        output
    }
}

pub struct AudioEngine {
    handle: Option<thread::JoinHandle<()>>,
}

impl AudioEngine {
    pub fn new(receiver: Receiver<AudioEvent>) -> Result<Self> {
        let spec = WavSpec {
            channels: 1,
            sample_rate: SAMPLE_RATE,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };

        let writer = WavWriter::create("thread_frequency_output.wav", spec)?;

        let handle = thread::spawn(move || {
            let mut writer = writer;
            let mut voices: Vec<Voice> = Vec::with_capacity(32);
            let start_time = Instant::now();
            let mut samples_written: u64 = 0;
            let mut disconnected = false;

            loop {
                // 1. Drain events
                loop {
                    match receiver.try_recv() {
                        Ok(event) => {
                            match event {
                                AudioEvent::NoteOn { freq, duration } => {
                                    voices.push(Voice::new_note(freq, duration));
                                }
                                AudioEvent::NoteOff { freq: _ } => {
                                    // Ignore
                                }
                                AudioEvent::Contention => {
                                    voices.push(Voice::new_noise());
                                }
                            }
                        }
                        Err(TryRecvError::Empty) => break,
                        Err(TryRecvError::Disconnected) => {
                            disconnected = true;
                            break;
                        }
                    }
                }

                // If disconnected and voices empty, we are done
                if disconnected && voices.is_empty() {
                    break;
                }

                // 2. Determine how many samples to write
                let elapsed = start_time.elapsed();
                let target_samples = (elapsed.as_secs_f64() * SAMPLE_RATE as f64) as u64;

                if target_samples > samples_written {
                    let mut needed = (target_samples - samples_written) as usize;

                    // Cap needed to avoid freezing if system lags
                    if needed > SAMPLE_RATE as usize {
                        needed = SAMPLE_RATE as usize;
                    }

                    for _ in 0..needed {
                        let mut sample = 0.0;
                        for voice in &mut voices {
                            sample += voice.process();
                        }

                        // Soft clip
                        sample = sample.clamp(-1.0, 1.0);

                        let amplitude = i16::MAX as f32;
                        let sample_i16 = (sample * amplitude * 0.8) as i16;

                        if let Err(e) = writer.write_sample(sample_i16) {
                            eprintln!("Error writing sample: {}", e);
                            return;
                        }
                    }
                    samples_written += needed as u64;

                    // Bulk cleanup
                    voices.retain(|v| v.active);

                } else {
                    thread::sleep(Duration::from_millis(5));
                }
            }

            // Finalize
            if let Err(e) = writer.finalize() {
                eprintln!("Error finalizing wav: {}", e);
            } else {
                println!("Audio finalized successfully.");
            }
        });

        Ok(Self { handle: Some(handle) })
    }

    pub fn join(mut self) -> Result<()> {
        if let Some(handle) = self.handle.take() {
            handle.join().map_err(|e| anyhow::anyhow!("Audio thread panicked: {:?}", e))?;
        }
        Ok(())
    }
}
