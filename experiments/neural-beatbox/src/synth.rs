use anyhow::Result;
use crossbeam_channel::{bounded, Sender, Receiver};

#[cfg(feature = "audio")]
use anyhow::anyhow;
#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "audio")]
use cpal::{Stream, StreamConfig};
#[cfg(feature = "audio")]
use rand::Rng;

struct Xorshift32 {
    state: u32,
}

impl Xorshift32 {
    fn new(seed: u32) -> Self {
        Self { state: if seed == 0 { 123456789 } else { seed } }
    }
    fn next_f32(&mut self) -> f32 {
        let mut x = self.state;
        x ^= x << 13;
        x ^= x >> 17;
        x ^= x << 5;
        self.state = x;
        // Map to -1.0..1.0
        // (x as f32 / u32::MAX as f32) * 2.0 - 1.0
        (x as f32 * 4.6566129e-10) * 2.0 - 1.0
    }
}

#[derive(Clone, Copy)]
#[allow(dead_code)]
pub enum Waveform {
    Sine,
    Square,
    Saw,
    Noise,
}

pub enum AudioCommand {
    Trigger {
        frequency: f32,
        decay: f32,
        amplitude: f32,
        waveform: Waveform,
    },
}

#[cfg(feature = "audio")]
const MAX_VOICES: usize = 32;

#[cfg(feature = "audio")]
struct Voice {
    frequency: f32,
    phase: f32,
    envelope: f32,
    decay: f32,
    waveform: Waveform,
    active: bool,
    rng: Xorshift32,
}

#[cfg(feature = "audio")]
impl Voice {
    fn new() -> Self {
        let mut seed_rng = rand::thread_rng();
        Self {
            frequency: 440.0,
            phase: 0.0,
            envelope: 0.0,
            decay: 0.0,
            waveform: Waveform::Sine,
            active: false,
            rng: Xorshift32::new(seed_rng.gen()),
        }
    }

    fn trigger(&mut self, frequency: f32, decay: f32, amplitude: f32, waveform: Waveform) {
        self.frequency = frequency;
        self.decay = decay;
        self.envelope = amplitude;
        self.waveform = waveform;
        self.active = true;
        self.phase = 0.0;
    }

    fn next_sample(&mut self, sample_rate: f32) -> f32 {
        if !self.active {
            return 0.0;
        }

        let sample = match self.waveform {
            Waveform::Sine => (self.phase * 2.0 * std::f32::consts::PI).sin(),
            Waveform::Square => if (self.phase * 2.0 * std::f32::consts::PI).sin() > 0.0 { 1.0 } else { -1.0 },
            Waveform::Saw => self.phase * 2.0 - 1.0,
            Waveform::Noise => self.rng.next_f32(),
        };

        let output = sample * self.envelope;

        // Advance phase
        self.phase += self.frequency / sample_rate;
        if self.phase > 1.0 {
            self.phase -= 1.0;
        }

        // Decay envelope
        self.envelope *= self.decay;
        if self.envelope < 0.001 {
            self.active = false;
        }

        output
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
            AudioCommand::Trigger { frequency, decay, amplitude, waveform } => {
                let idx = self.next_voice;
                self.voices[idx].trigger(frequency, decay, amplitude, waveform);
                self.next_voice = (self.next_voice + 1) % MAX_VOICES;
            }
        }
    }

    fn process(&mut self) -> f32 {
        let mut mix = 0.0;
        for voice in &mut self.voices {
            mix += voice.next_sample(self.sample_rate);
        }
        mix.tanh()
    }
}

pub struct AudioHandle {
    #[cfg(feature = "audio")]
    _stream: Option<Stream>,
}

pub fn init_audio() -> Result<(AudioHandle, Sender<AudioCommand>)> {
    let (cmd_tx, cmd_rx) = bounded(1024);

    // We try to init audio, if it fails (no device), we fallback to silent mode.
    // If feature is disabled, we just run silent.

    #[cfg(feature = "audio")]
    match init_cpal(cmd_rx.clone()) {
        Ok(stream) => return Ok((AudioHandle { _stream: Some(stream) }, cmd_tx)),
        Err(e) => {
            eprintln!("Audio init failed: {}. Running silent.", e);
        }
    }

    // Fallback: drain channel
    std::thread::spawn(move || {
        while cmd_rx.recv().is_ok() {}
    });

    #[cfg(feature = "audio")]
    return Ok((AudioHandle { _stream: None }, cmd_tx));

    #[cfg(not(feature = "audio"))]
    return Ok((AudioHandle {}, cmd_tx));
}

#[cfg(feature = "audio")]
fn init_cpal(cmd_rx: Receiver<AudioCommand>) -> Result<Stream> {
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or_else(|| anyhow!("No output device"))?;
    let config: StreamConfig = device.default_output_config()?.into();
    let sample_rate = config.sample_rate.0 as f32;

    let mut model = AudioModel::new(sample_rate);
    let err_fn = |err| eprintln!("Audio stream error: {}", err);

    let stream = device.build_output_stream(
        &config,
        move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
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
    Ok(stream)
}
