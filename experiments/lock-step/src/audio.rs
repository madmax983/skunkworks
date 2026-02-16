use crate::synth::{Envelope, EnvelopeState, Oscillator, Waveform};
use anyhow::Result;
#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
#[cfg(feature = "audio")]
use crossbeam::channel::{Receiver, Sender};

#[derive(Clone, Copy)]
pub enum AudioEvent {
    PlayNote {
        frequency: f32,
        duration: f32,
        waveform: Waveform,
    },
}

pub trait AudioBackend: Send + Sync {
    fn play_note(&self, frequency: f32, duration: f32, waveform: Waveform);
}

#[derive(Clone)]
pub struct DummyBackend;

impl DummyBackend {
    pub fn new() -> Result<Self> {
        Ok(Self)
    }
}

impl AudioBackend for DummyBackend {
    fn play_note(&self, _frequency: f32, _duration: f32, _waveform: Waveform) {
        // No-op
    }
}

#[cfg(feature = "audio")]
pub struct CpalBackend {
    _stream: cpal::Stream,
}

#[cfg(feature = "audio")]
#[derive(Clone)]
pub struct AudioHandle {
    sender: Sender<AudioEvent>,
}

#[cfg(feature = "audio")]
impl AudioBackend for AudioHandle {
    fn play_note(&self, frequency: f32, duration: f32, waveform: Waveform) {
        let _ = self.sender.send(AudioEvent::PlayNote {
            frequency,
            duration,
            waveform,
        });
    }
}

#[cfg(feature = "audio")]
struct Voice {
    osc: Oscillator,
    env: Envelope,
}

#[cfg(feature = "audio")]
impl CpalBackend {
    pub fn new() -> Result<(Self, AudioHandle)> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No output device"))?;
        let config = device.default_output_config()?;
        let sample_rate = config.sample_rate().0 as f32;
        let channels = config.channels() as usize;

        let (sender, receiver) = crossbeam::channel::unbounded();

        let mut voices: Vec<Voice> = Vec::new();

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    write_data(data, channels, &receiver, &mut voices, sample_rate)
                },
                err_fn,
                None,
            )?,
            _ => return Err(anyhow::anyhow!("Unsupported sample format")),
        };

        stream.play()?;

        Ok((Self { _stream: stream }, AudioHandle { sender }))
    }
}

#[cfg(feature = "audio")]
fn write_data(
    output: &mut [f32],
    channels: usize,
    receiver: &Receiver<AudioEvent>,
    voices: &mut Vec<Voice>,
    sample_rate: f32,
) {
    // Process new events once per callback
    while let Ok(event) = receiver.try_recv() {
        match event {
            AudioEvent::PlayNote {
                frequency,
                duration,
                waveform,
            } => {
                let osc = Oscillator::new(frequency, sample_rate, waveform);
                let mut env = Envelope::new(0.005, duration, 0.0, 0.1, sample_rate);
                env.trigger();
                voices.push(Voice { osc, env });
            }
        }
    }

    for frame in output.chunks_mut(channels) {
        let mut sample = 0.0;

        let mut i = 0;
        while i < voices.len() {
            let voice = &mut voices[i];
            let osc_val = voice.osc.next_sample();
            let env_val = voice.env.next_sample();

            sample += osc_val * env_val;

            if voice.env.level <= 0.0001
                && (voice.env.state == EnvelopeState::Sustain
                    || voice.env.state == EnvelopeState::Idle)
            {
                voices.swap_remove(i);
            } else {
                i += 1;
            }
        }

        // Clip
        if sample > 0.8 {
            sample = 0.8;
        }
        if sample < -0.8 {
            sample = -0.8;
        }

        for sample_out in frame.iter_mut() {
            *sample_out = sample;
        }
    }
}
