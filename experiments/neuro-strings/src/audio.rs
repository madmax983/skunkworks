use anyhow::Result;
use crossbeam_channel::{unbounded, Receiver, Sender};
use std::thread;

pub enum AudioCommand {
    Pluck {
        frequency: f32,
        decay: f32,
        amplitude: f32,
    },
}

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub fn init_audio() -> Result<(Option<()>, Sender<AudioCommand>)> {
    let (tx, rx) = unbounded();

    #[cfg(feature = "audio")]
    {
        thread::spawn(move || {
            run_audio_thread(rx).expect("Audio thread crashed");
        });
        Ok((Some(()), tx))
    }

    #[cfg(not(feature = "audio"))]
    {
        thread::spawn(move || {
            // Dummy consumer
            while let Ok(_) = rx.recv() {}
        });
        Ok((None, tx))
    }
}

#[cfg(feature = "audio")]
fn run_audio_thread(rx: Receiver<AudioCommand>) -> Result<()> {
    let host = cpal::default_host();
    let device = host
        .default_output_device()
        .ok_or_else(|| anyhow::anyhow!("No audio output device found"))?;

    let config = device.default_output_config()?;
    let sample_rate = config.sample_rate().0 as f32;
    let channels = config.channels() as usize;

    let mut voices: Vec<Voice> = Vec::new();

    let err_fn = |err| eprintln!("An error occurred on the output audio stream: {}", err);

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _| {
                // Process incoming commands
                while let Ok(cmd) = rx.try_recv() {
                    match cmd {
                        AudioCommand::Pluck {
                            frequency,
                            decay,
                            amplitude,
                        } => {
                            voices.push(Voice::new(frequency, decay, amplitude));
                        }
                    }
                }

                // Render audio
                for frame in data.chunks_mut(channels) {
                    let mut sample = 0.0;
                    for voice in &mut voices {
                        sample += voice.next_sample(sample_rate);
                    }

                    // Soft clipping
                    let clipped = sample.tanh() * 0.5;

                    for channel in frame.iter_mut() {
                        *channel = clipped;
                    }
                }

                // Remove dead voices
                voices.retain(|v| v.amplitude > 0.001);
            },
            err_fn,
            None,
        )?,
        _ => return Err(anyhow::anyhow!("Unsupported sample format")),
    };

    stream.play()?;

    // Keep thread alive
    loop {
        thread::sleep(std::time::Duration::from_secs(1));
    }
}

#[cfg(feature = "audio")]
struct Voice {
    frequency: f32,
    decay: f32,
    amplitude: f32,
    phase: f32,
}

#[cfg(feature = "audio")]
impl Voice {
    fn new(frequency: f32, decay: f32, amplitude: f32) -> Self {
        Self {
            frequency,
            // Convert to per-sample decay
            decay: (1.0 - decay) * 10.0,
            amplitude,
            phase: 0.0,
        }
    }

    fn next_sample(&mut self, sample_rate: f32) -> f32 {
        let sample = (self.phase * 2.0 * std::f32::consts::PI).sin() * self.amplitude;
        self.phase = (self.phase + self.frequency / sample_rate).fract();

        // Very simple exponential decay
        self.amplitude *= 1.0 - (self.decay / sample_rate);

        sample
    }
}
