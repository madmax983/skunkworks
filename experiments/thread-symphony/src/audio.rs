use crate::conductor::Instrument;
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::Sender;

pub struct AudioEngine {
    _stream: cpal::Stream,
    sender: Sender<Instrument>,
}

struct Voice {
    instrument: Option<Instrument>,
    phase: f32,
    envelope: f32,
    noise_seed: u32,
}

impl Voice {
    fn new() -> Self {
        Self {
            instrument: None,
            phase: 0.0,
            envelope: 0.0,
            noise_seed: 12345,
        }
    }

    fn trigger(&mut self, instrument: Instrument) {
        self.instrument = Some(instrument);
        self.phase = 0.0;
        self.envelope = 1.0;
    }

    fn process(&mut self, sample_rate: f32) -> f32 {
        if self.envelope <= 0.001 {
            return 0.0;
        }

        let output = match self.instrument {
            Some(Instrument::Kick) => {
                // Sine sweep 150 -> 50
                let freq = 50.0 + 100.0 * self.envelope.powi(2);
                self.phase += freq / sample_rate;
                let signal = (self.phase * 2.0 * std::f32::consts::PI).sin();
                self.envelope *= 0.995; // Decay
                signal
            }
            Some(Instrument::Snare) => {
                // Noise + Tone
                let tone = (self.phase * 200.0 * 2.0 * std::f32::consts::PI).sin();
                self.phase += 200.0 / sample_rate;

                // Simple noise
                self.noise_seed = self
                    .noise_seed
                    .wrapping_mul(1664525)
                    .wrapping_add(1013904223);
                let noise = (self.noise_seed as f32 / u32::MAX as f32) * 2.0 - 1.0;

                let signal = 0.5 * tone + 0.5 * noise;
                self.envelope *= 0.99;
                signal
            }
            Some(Instrument::Hat) => {
                // High freq noise
                self.noise_seed = self
                    .noise_seed
                    .wrapping_mul(1664525)
                    .wrapping_add(1013904223);
                let noise = (self.noise_seed as f32 / u32::MAX as f32) * 2.0 - 1.0;
                self.envelope *= 0.95; // Fast decay
                noise * 0.5
            }
            Some(Instrument::Crash) => {
                self.noise_seed = self
                    .noise_seed
                    .wrapping_mul(1664525)
                    .wrapping_add(1013904223);
                let noise = (self.noise_seed as f32 / u32::MAX as f32) * 2.0 - 1.0;
                self.envelope *= 0.999; // Slow decay
                noise * 0.3
            }
            None => 0.0,
        };

        output * self.envelope
    }
}

impl AudioEngine {
    pub fn new() -> anyhow::Result<Self> {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow::anyhow!("No output device available"))?;

        let config = device.default_output_config()?;
        let sample_rate = config.sample_rate().0 as f32;
        let channels = config.channels() as usize;

        let (sender, receiver) = crossbeam_channel::unbounded();

        // Max polyphony = 8
        let mut voices: Vec<Voice> = (0..8).map(|_| Voice::new()).collect();
        let mut next_voice = 0;

        let stream = device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // Check for new events
                while let Ok(inst) = receiver.try_recv() {
                    voices[next_voice].trigger(inst);
                    next_voice = (next_voice + 1) % voices.len();
                }

                // Fill buffer
                for frame in data.chunks_mut(channels) {
                    let mut sample = 0.0;
                    for voice in &mut voices {
                        sample += voice.process(sample_rate);
                    }

                    // Clip
                    sample = sample.clamp(-1.0, 1.0);

                    for s in frame {
                        *s = sample;
                    }
                }
            },
            |err| eprintln!("Audio stream error: {}", err),
            None,
        )?;

        stream.play()?;

        Ok(Self {
            _stream: stream,
            sender,
        })
    }

    pub fn play(&self, instrument: Instrument) {
        let _ = self.sender.send(instrument);
    }

    pub fn get_sender(&self) -> Sender<Instrument> {
        self.sender.clone()
    }
}
