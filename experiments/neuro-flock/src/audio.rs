use anyhow::Result;
use crossbeam_channel::Receiver;

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub enum AudioEvent {
    #[allow(dead_code)]
    PlayFreq(f32),
}

pub struct AudioEngine {
    #[cfg(feature = "audio")]
    _stream: cpal::Stream,
}

impl AudioEngine {
    pub fn new(rx: Receiver<AudioEvent>) -> Result<Self> {
        #[cfg(not(feature = "audio"))]
        {
            // Avoid unused variable warning
            let _ = rx;
            Ok(Self {})
        }

        #[cfg(feature = "audio")]
        {
            let host = cpal::default_host();
            // Fail gracefully if no device
            let device = match host.default_output_device() {
                Some(d) => d,
                None => return Ok(Self { _stream: stream_dummy(rx)? }),
            };

            let config = device.default_output_config()?;

            let stream = match config.sample_format() {
                cpal::SampleFormat::F32 => run::<f32>(&device, &config.into(), rx)?,
                cpal::SampleFormat::I16 => run::<i16>(&device, &config.into(), rx)?,
                cpal::SampleFormat::U16 => run::<u16>(&device, &config.into(), rx)?,
                _ => return Err(anyhow::anyhow!("Unsupported sample format")),
            };

            stream.play()?;

            Ok(Self { _stream: stream })
        }
    }
}

#[cfg(feature = "audio")]
fn run<T>(device: &cpal::Device, config: &cpal::StreamConfig, rx: Receiver<AudioEvent>) -> Result<cpal::Stream>
where
    T: cpal::Sample + cpal::FromSample<f32>,
{
    let sample_rate = config.sample_rate.0 as f32;
    let channels = config.channels as usize;

    let mut synth = PolySynth::new(sample_rate);

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = device.build_output_stream(
        config,
        move |data: &mut [T], _: &cpal::OutputCallbackInfo| {
            // Process events
            while let Ok(event) = rx.try_recv() {
                match event {
                    AudioEvent::PlayFreq(freq) => {
                        synth.trigger(freq);
                    }
                }
            }

            // Fill buffer
            for frame in data.chunks_mut(channels) {
                let value = synth.next_sample();
                let sample = T::from_sample(value);
                for sample_out in frame.iter_mut() {
                    *sample_out = sample;
                }
            }
        },
        err_fn,
        None,
    )?;

    Ok(stream)
}

#[cfg(feature = "audio")]
struct Voice {
    active: bool,
    freq: f32,
    phase: f32,
    envelope: f32,
    decay: f32,
}

#[cfg(feature = "audio")]
struct PolySynth {
    voices: Vec<Voice>,
    sample_rate: f32,
}

#[cfg(feature = "audio")]
impl PolySynth {
    fn new(sample_rate: f32) -> Self {
        let mut voices = Vec::with_capacity(32);
        for _ in 0..32 {
            voices.push(Voice {
                active: false,
                freq: 0.0,
                phase: 0.0,
                envelope: 0.0,
                decay: 0.995, // Slower decay for more "pad" like sound
            });
        }
        Self { voices, sample_rate }
    }

    fn trigger(&mut self, freq: f32) {
        // Steal voice with lowest envelope
        let mut min_env = 100.0;
        let mut idx = 0;

        for (i, v) in self.voices.iter().enumerate() {
            if !v.active {
                idx = i;
                break;
            }
            if v.envelope < min_env {
                min_env = v.envelope;
                idx = i;
            }
        }

        self.voices[idx].active = true;
        self.voices[idx].freq = freq;
        self.voices[idx].phase = 0.0;
        self.voices[idx].envelope = 0.3; // Volume
    }

    fn next_sample(&mut self) -> f32 {
        let mut output = 0.0;
        for voice in &mut self.voices {
            if voice.active {
                let val = (voice.phase * 2.0 * std::f32::consts::PI).sin();
                output += val * voice.envelope;

                voice.phase += voice.freq / self.sample_rate;
                if voice.phase > 1.0 {
                    voice.phase -= 1.0;
                }

                voice.envelope *= voice.decay;
                if voice.envelope < 0.001 {
                    voice.active = false;
                }
            }
        }
        output.max(-0.8).min(0.8)
    }
}

#[cfg(feature = "audio")]
fn stream_dummy(_rx: Receiver<AudioEvent>) -> Result<cpal::Stream> {
   Err(anyhow::anyhow!("No audio device found"))
}
