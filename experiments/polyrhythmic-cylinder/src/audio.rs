use rand::Rng;
use std::collections::VecDeque;
use std::sync::mpsc::{channel, Receiver, Sender};
use std::sync::{Arc, Mutex};

#[derive(Clone)]
pub enum AudioCommand {
    Pluck { frequency: f32, volume: f32 },
}

struct Voice {
    buffer: Vec<f32>,
    index: usize,
    active: bool,
    sample_rate: f32,
    frequency: f32,
}

impl Voice {
    fn new(sample_rate: f32) -> Self {
        Self {
            buffer: Vec::new(),
            index: 0,
            active: false,
            sample_rate,
            frequency: 0.0,
        }
    }

    fn pluck(&mut self, frequency: f32, volume: f32) {
        if frequency <= 0.0 {
            return;
        }
        self.frequency = frequency;

        // Karplus-Strong period
        let period = (self.sample_rate / frequency).round() as usize;
        if period < 2 {
            return;
        }

        // Resize buffer
        self.buffer.resize(period, 0.0);

        // Fill with noise burst
        let mut rng = rand::thread_rng();
        for i in 0..period {
            self.buffer[i] = (rng.gen::<f32>() * 2.0 - 1.0) * volume;
        }

        self.index = 0;
        self.active = true;
    }

    fn process(&mut self) -> f32 {
        if !self.active || self.buffer.is_empty() {
            return 0.0;
        }

        let len = self.buffer.len();
        let current_val = self.buffer[self.index];

        // Previous value (wrapping)
        let prev_index = if self.index == 0 {
            len - 1
        } else {
            self.index - 1
        };
        let prev_val = self.buffer[prev_index];

        // Filter
        let decay = 0.996;
        let new_val = (current_val + prev_val) * 0.5 * decay;

        // Write back (feedback)
        self.buffer[self.index] = new_val;

        // Advance
        self.index = (self.index + 1) % len;

        // If energy is low, deactivate?
        // Simple heuristic: if we wrapped around 100 times?
        // Or just let it decay to denormal.
        // For efficiency, maybe check magnitude occasionally.

        current_val
    }
}

pub struct AudioModel {
    voices: Vec<Voice>,
    command_rx: Receiver<AudioCommand>,
    sample_rate: f32,
}

impl AudioModel {
    pub fn new(command_rx: Receiver<AudioCommand>, sample_rate: f32) -> Self {
        let mut voices = Vec::with_capacity(16);
        for _ in 0..16 {
            voices.push(Voice::new(sample_rate));
        }

        Self {
            voices,
            command_rx,
            sample_rate,
        }
    }

    pub fn process_block(&mut self, output: &mut [f32]) {
        // Process commands at start of block
        while let Ok(cmd) = self.command_rx.try_recv() {
            match cmd {
                AudioCommand::Pluck { frequency, volume } => {
                    // Find free voice
                    if let Some(voice) = self.voices.iter_mut().find(|v| !v.active) {
                        voice.pluck(frequency, volume);
                    } else {
                        // Steal voice with lowest amplitude? Or just random.
                        // For now, round robin by finding oldest?
                        // Let's just steal the first one.
                        if let Some(voice) = self.voices.first_mut() {
                            voice.pluck(frequency, volume);
                        }
                    }
                }
            }
        }

        for sample in output.iter_mut() {
            let mut mix = 0.0;
            for voice in &mut self.voices {
                mix += voice.process();
            }
            // Soft clip
            *sample = mix.tanh();
        }
    }
}

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

#[cfg(feature = "audio")]
pub struct AudioHandle {
    _stream: Option<cpal::Stream>,
}

#[cfg(not(feature = "audio"))]
pub struct AudioHandle {}

pub fn start_audio() -> (Sender<AudioCommand>, AudioHandle) {
    let (tx, rx) = channel();

    #[cfg(feature = "audio")]
    {
        let host = cpal::default_host();
        let device = host.default_output_device();

        if let Some(device) = device {
            if let Ok(config) = device.default_output_config() {
                let sample_rate = config.sample_rate().0 as f32;
                let mut model = AudioModel::new(rx, sample_rate);

                let err_fn = |err| eprintln!("Audio stream error: {}", err);

                let stream = match config.sample_format() {
                    cpal::SampleFormat::F32 => device.build_output_stream(
                        &config.into(),
                        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                            model.process_block(data);
                        },
                        err_fn,
                        None,
                    ),
                    _ => Err(cpal::BuildStreamError::StreamConfigNotSupported),
                };

                if let Ok(stream) = stream {
                    stream.play().unwrap();
                    return (
                        tx,
                        AudioHandle {
                            _stream: Some(stream),
                        },
                    );
                }
            }
        }

        // Fallback if audio init failed but feature is enabled
        // Spawn a thread to drain channel
        std::thread::spawn(move || while let Ok(_) = rx.recv() {});
        return (tx, AudioHandle { _stream: None });
    }

    #[cfg(not(feature = "audio"))]
    {
        std::thread::spawn(move || while let Ok(_) = rx.recv() {});
        (tx, AudioHandle {})
    }
}
