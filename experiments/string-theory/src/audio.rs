use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};
use crossbeam_channel::Receiver;
use crate::dsp::KarplusStrong;

pub enum AudioCommand {
    Pluck { freq: f32, decay: f32, amp: f32 },
    Kill,
}

pub struct Voice {
    pub string: KarplusStrong,
    pub active: bool,
    pub age: u64, // For voice stealing
}

pub struct PolySynth {
    voices: Vec<Voice>,
    sample_rate: f32,
    next_voice_idx: usize,
}

impl PolySynth {
    pub fn new(sample_rate: f32, polyphony: usize) -> Self {
        let mut voices = Vec::with_capacity(polyphony);
        for _ in 0..polyphony {
            voices.push(Voice {
                string: KarplusStrong::new(440.0, sample_rate, 0.99),
                active: false,
                age: 0,
            });
        }
        Self {
            voices,
            sample_rate,
            next_voice_idx: 0,
        }
    }

    pub fn pluck(&mut self, freq: f32, decay: f32, amp: f32) {
        // Round robin voice allocation for simplicity
        let idx = self.next_voice_idx;
        self.next_voice_idx = (self.next_voice_idx + 1) % self.voices.len();

        let voice = &mut self.voices[idx];
        // Re-initialize string with new params
        voice.string = KarplusStrong::new(freq, self.sample_rate, decay);
        voice.string.pluck(amp);
        voice.active = true;
        voice.age = 0;
    }

    pub fn next_sample(&mut self) -> f32 {
        let mut output = 0.0;
        for voice in self.voices.iter_mut() {
            if voice.active {
                let sample = voice.string.tick();
                output += sample;
                voice.age += 1;
                if voice.age > (self.sample_rate as u64 * 4) {
                    voice.active = false;
                }
            }
        }
        output.clamp(-1.0, 1.0)
    }
}

pub fn run_audio(rx: Receiver<AudioCommand>) -> anyhow::Result<cpal::Stream> {
    let host = cpal::default_host();
    let device = host.default_output_device().ok_or(anyhow::anyhow!("No output device"))?;
    let config = device.default_output_config()?;

    // config.sample_rate() returns u32 in this environment.
    let sample_rate = config.sample_rate() as f32;

    let mut synth = PolySynth::new(sample_rate, 32);

    let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

    let stream = match config.sample_format() {
        cpal::SampleFormat::F32 => device.build_output_stream(
            &config.into(),
            move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                // Process commands
                while let Ok(cmd) = rx.try_recv() {
                    match cmd {
                        AudioCommand::Pluck { freq, decay, amp } => {
                            synth.pluck(freq, decay, amp);
                        }
                        AudioCommand::Kill => {
                            // Handle kill?
                        }
                    }
                }

                // Render
                for sample in data.iter_mut() {
                    *sample = synth.next_sample() * 0.5; // Master volume
                }
            },
            err_fn,
            None,
        )?,
        _ => return Err(anyhow::anyhow!("Unsupported sample format")),
    };

    stream.play()?;
    Ok(stream)
}
