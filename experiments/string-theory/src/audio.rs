use anyhow::Result;
use crossbeam_channel::{bounded, Sender};

#[cfg(feature = "audio")]
use anyhow::anyhow;
#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "audio")]
use cpal::{Stream, StreamConfig};
#[cfg(feature = "audio")]
use rand::Rng;

#[allow(dead_code)]
pub enum AudioCommand {
    Pluck {
        frequency: f32,
        decay: f32,
        amplitude: f32,
    },
}

#[cfg(feature = "audio")]
const MAX_VOICES: usize = 64;
#[cfg(feature = "audio")]
const MAX_BUFFER_SIZE: usize = 2048; // Support down to ~20Hz

#[cfg(feature = "audio")]
struct Voice {
    buffer: Vec<f32>,
    len: usize,
    ptr: usize,
    decay: f32,
    energy: f32,
    active: bool,
}

#[cfg(feature = "audio")]
impl Voice {
    fn new() -> Self {
        Self {
            buffer: vec![0.0; MAX_BUFFER_SIZE],
            len: 0,
            ptr: 0,
            decay: 0.0,
            energy: 0.0,
            active: false,
        }
    }

    fn reset(&mut self, frequency: f32, sample_rate: f32, decay: f32, amplitude: f32) {
        let period = (sample_rate / frequency).max(2.0) as usize;
        self.len = period.min(MAX_BUFFER_SIZE);

        let mut rng = rand::thread_rng();
        for i in 0..self.len {
            self.buffer[i] = (rng.gen::<f32>() - 0.5) * 2.0 * amplitude;
        }

        self.ptr = 0;
        self.decay = decay;
        self.energy = amplitude;
        self.active = true;
    }

    fn next_sample(&mut self) -> f32 {
        if !self.active || self.len == 0 { return 0.0; }

        let current = self.buffer[self.ptr];
        let prev_idx = if self.ptr == 0 { self.len - 1 } else { self.ptr - 1 };
        let prev = self.buffer[prev_idx];

        // Karplus-Strong update
        let new_val = 0.5 * (current + prev) * self.decay;

        self.buffer[self.ptr] = new_val;
        self.ptr = (self.ptr + 1) % self.len;

        self.energy = self.energy * 0.999 + new_val.abs() * 0.001;

        if self.energy < 1e-4 {
            self.active = false;
        }

        current
    }
}

#[cfg(feature = "audio")]
struct AudioModel {
    voices: Vec<Voice>,
    sample_rate: f32,
    next_voice: usize,
}

#[cfg(feature = "audio")]
impl AudioModel {
    fn new(sample_rate: f32) -> Self {
        let mut voices = Vec::with_capacity(MAX_VOICES);
        for _ in 0..MAX_VOICES {
            voices.push(Voice::new());
        }

        Self {
            voices,
            sample_rate,
            next_voice: 0,
        }
    }

    fn handle_command(&mut self, cmd: AudioCommand) {
        match cmd {
            AudioCommand::Pluck { frequency, decay, amplitude } => {
                let idx = self.next_voice;
                self.voices[idx].reset(frequency, self.sample_rate, decay, amplitude);
                self.next_voice = (self.next_voice + 1) % MAX_VOICES;
            }
        }
    }

    fn process(&mut self) -> f32 {
        let mut mix = 0.0;
        for voice in &mut self.voices {
            mix += voice.next_sample();
        }
        mix.tanh()
    }
}

pub struct AudioHandle {
    #[cfg(feature = "audio")]
    _stream: Stream,
}

pub fn init_audio() -> Result<(AudioHandle, Sender<AudioCommand>)> {
    let (cmd_tx, cmd_rx) = bounded(1024);

    #[cfg(feature = "audio")]
    {
        let host = cpal::default_host();
        let device = host.default_output_device()
            .ok_or_else(|| anyhow!("No output device available"))?;

        let config: StreamConfig = device.default_output_config()?.into();
        let sample_rate = config.sample_rate.0 as f32;

        let mut model = AudioModel::new(sample_rate);

        let err_fn = |err| eprintln!("an error occurred on audio stream: {}", err);

        let stream = device.build_output_stream(
            &config,
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // Process commands once per buffer
                while let Ok(cmd) = cmd_rx.try_recv() {
                    model.handle_command(cmd);
                }

                for sample in data.iter_mut() {
                    *sample = model.process();
                }
            },
            err_fn,
            None,
        )?;

        stream.play()?;

        Ok((AudioHandle { _stream: stream }, cmd_tx))
    }

    #[cfg(not(feature = "audio"))]
    {
        println!("Audio disabled (missing ALSA/CPAL). Running in silent mode.");
        // Spawn a thread to drain the channel
        std::thread::spawn(move || {
            while cmd_rx.recv().is_ok() {
                // Drain
            }
        });
        Ok((AudioHandle {}, cmd_tx))
    }
}
