use anyhow::Result;
use crossbeam_channel::Receiver;

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub enum AudioEvent {
    Spike(usize),
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
                None => return Ok(Self { _stream: stream_dummy(rx)? }), // Should probably error or return dummy
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

// Dummy function to satisfy type checker if needed, but actually if feature is on, we expect cpal types.
// If feature is off, struct has no stream.

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
                    AudioEvent::Spike(idx) => {
                        // Pentatonic scale mapping: C major pentatonic
                        // C, D, E, G, A
                        // MIDI: 60, 62, 64, 67, 69
                        let notes = [261.63, 293.66, 329.63, 392.00, 440.00];

                        // Reduce idx to a manageable range or map specific neurons
                        // Let's use lower bits for note, higher bits for octave
                        let note_idx = idx % 5;
                        let octave_shift = (idx / 100) % 4; // 0..3

                        // Base pitch
                        let mut freq = notes[note_idx];

                        // Shift octaves (down 1, up 0..2)
                        // 2^octave_shift
                        // Let's create a wider spread
                        let mult = match octave_shift {
                            0 => 0.5,
                            1 => 1.0,
                            2 => 2.0,
                            _ => 4.0,
                        };
                        freq *= mult;

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

// Simple Polyphonic Synth Logic
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
                decay: 0.999, // Determines length of "ping"
            });
        }
        Self { voices, sample_rate }
    }

    fn trigger(&mut self, freq: f32) {
        // Steal voice with lowest envelope
        let mut min_env = 100.0;
        let mut idx = 0;

        // First look for inactive
        for (i, v) in self.voices.iter().enumerate() {
            if !v.active {
                idx = i;
                // min_env = -1.0; // Found perfect candidate
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
        self.voices[idx].envelope = 0.2; // Volume
    }

    fn next_sample(&mut self) -> f32 {
        let mut output = 0.0;
        for voice in &mut self.voices {
            if voice.active {
                // Sine wave
                let val = (voice.phase * 2.0 * std::f32::consts::PI).sin();
                output += val * voice.envelope;

                // Advance phase
                voice.phase += voice.freq / self.sample_rate;
                if voice.phase > 1.0 {
                    voice.phase -= 1.0;
                }

                // Decay envelope
                voice.envelope *= voice.decay;
                if voice.envelope < 0.001 {
                    voice.active = false;
                }
            }
        }
        // Hard clipper / limiter
        output.max(-0.8).min(0.8)
    }
}

// Dummy for compilation if cpal feature on but device init fails?
// No, relying on anyhow error bubbling.
#[cfg(feature = "audio")]
fn stream_dummy(_rx: Receiver<AudioEvent>) -> Result<cpal::Stream> {
   Err(anyhow::anyhow!("No audio device found"))
}
