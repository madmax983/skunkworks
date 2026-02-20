use crossbeam_channel::{Receiver, TryRecvError};
use hound::{WavSpec, WavWriter};
use std::f32::consts::PI;
use std::thread;
use std::time::{Duration, Instant};

#[derive(Debug, Clone, Copy)]
pub enum AudioCommand {
    Kick,
    Snare,
    Hat,
    Stop,
}

trait AudioSource: Send {
    fn next_sample(&mut self) -> Option<f32>;
}

struct KickDrum {
    phase: f32,
    frequency: f32,
    envelope: f32,
}

impl KickDrum {
    fn new() -> Self {
        Self {
            phase: 0.0,
            frequency: 150.0, // Start high for punch
            envelope: 1.0,
        }
    }
}

impl AudioSource for KickDrum {
    fn next_sample(&mut self) -> Option<f32> {
        if self.envelope < 0.001 {
            return None;
        }
        self.phase += self.frequency / 44100.0 * 2.0 * PI;
        let sample = self.phase.sin() * self.envelope;
        self.frequency *= 0.999; // Pitch drop
        self.envelope *= 0.9995;
        Some(sample)
    }
}

struct SnareDrum {
    envelope: f32,
}

impl SnareDrum {
    fn new() -> Self {
        Self { envelope: 1.0 }
    }
}

impl AudioSource for SnareDrum {
    fn next_sample(&mut self) -> Option<f32> {
        if self.envelope < 0.001 {
            return None;
        }
        let noise = (rand::random::<f32>() * 2.0 - 1.0) * self.envelope;
        self.envelope *= 0.995;
        Some(noise)
    }
}

struct Hat {
    envelope: f32,
}

impl Hat {
    fn new() -> Self {
        Self { envelope: 0.5 }
    }
}

impl AudioSource for Hat {
    fn next_sample(&mut self) -> Option<f32> {
        if self.envelope < 0.001 {
            return None;
        }
        let noise = (rand::random::<f32>() * 2.0 - 1.0) * self.envelope; // High pass filter would be better but simple noise is ok
        self.envelope *= 0.9; // Fast decay
        Some(noise)
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
        let mut writer = WavWriter::create("syncopated_rhythm.wav", spec).expect("Failed to create WAV file");

        let mut active_sounds: Vec<Box<dyn AudioSource>> = Vec::new();
        let start_time = Instant::now();
        let mut samples_written = 0;
        let sample_rate = 44100;

        loop {
            // Check for new commands
            loop {
                match receiver.try_recv() {
                    Ok(cmd) => match cmd {
                        AudioCommand::Kick => active_sounds.push(Box::new(KickDrum::new())),
                        AudioCommand::Snare => active_sounds.push(Box::new(SnareDrum::new())),
                        AudioCommand::Hat => active_sounds.push(Box::new(Hat::new())),
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
                    sample = sample.max(-1.0).min(1.0);

                    writer.write_sample((sample * i16::MAX as f32) as i16).unwrap();
                }
                samples_written += samples_to_write;
            }

            thread::sleep(Duration::from_millis(1));
        }
    })
}
