use anyhow::Result;
use crossbeam_channel::{unbounded, Receiver, Sender};

#[cfg(feature = "audio")]
use anyhow::anyhow;

#[cfg(feature = "audio")]
use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

pub struct AudioHandle {
    #[cfg(feature = "audio")]
    _stream: cpal::Stream,

    pub patterns_tx: Sender<Vec<Vec<bool>>>,
    pub step_rx: Receiver<usize>,
}

pub fn init_audio(bpm: f32) -> Result<AudioHandle> {
    // In dummy mode, we don't need the receiving end of patterns
    #[cfg(feature = "audio")]
    let (patterns_tx, patterns_rx) = unbounded::<Vec<Vec<bool>>>();
    #[cfg(not(feature = "audio"))]
    let (patterns_tx, _patterns_rx) = unbounded::<Vec<Vec<bool>>>();

    let (step_tx, step_rx) = unbounded::<usize>();

    #[cfg(feature = "audio")]
    {
        let host = cpal::default_host();
        let device = host
            .default_output_device()
            .ok_or_else(|| anyhow!("No output device available"))?;
        let config = device.default_output_config()?;
        let sample_rate = config.sample_rate().0 as f32;
        let channels = config.channels() as usize;

        let mut engine = SynthesisEngine::new(sample_rate, bpm, channels, patterns_rx, step_tx);

        let err_fn = |err| eprintln!("an error occurred on stream: {}", err);

        let stream = match config.sample_format() {
            cpal::SampleFormat::F32 => device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    engine.fill_buffer(data);
                },
                err_fn,
                None,
            )?,
            _ => return Err(anyhow!("Unsupported sample format")),
        };

        stream.play()?;

        Ok(AudioHandle {
            _stream: stream,
            patterns_tx,
            step_rx,
        })
    }

    #[cfg(not(feature = "audio"))]
    {
        // In dummy mode, we need a thread to simulate the clock so the TUI still animates
        let step_tx = step_tx.clone();
        std::thread::spawn(move || {
            let mut current_step = 0;
            let step_duration = 60.0 / bpm / 4.0; // 16th notes
            loop {
                std::thread::sleep(std::time::Duration::from_secs_f32(step_duration));
                current_step = (current_step + 1) % 16;
                if step_tx.send(current_step).is_err() {
                    break;
                }
            }
        });

        Ok(AudioHandle {
            patterns_tx,
            step_rx,
        })
    }
}

// Synthesis Logic (only compiled if audio is enabled)
#[cfg(feature = "audio")]
struct SynthesisEngine {
    sample_rate: f32,
    _bpm: f32, // suppressed warning
    channels: usize,

    // State
    patterns_rx: Receiver<Vec<Vec<bool>>>,
    step_tx: Sender<usize>,

    patterns: Vec<Vec<bool>>,
    current_step: usize,
    samples_per_step: usize,
    phase: usize, // current sample index within a step

    // Voices
    voices: Vec<Voice>,
}

#[cfg(feature = "audio")]
struct Voice {
    frequency: f32,
    envelope: f32,
    active: bool,
}

#[cfg(feature = "audio")]
impl SynthesisEngine {
    fn new(
        sample_rate: f32,
        bpm: f32,
        channels: usize,
        patterns_rx: Receiver<Vec<Vec<bool>>>,
        step_tx: Sender<usize>,
    ) -> Self {
        let samples_per_step = (sample_rate * 60.0 / bpm / 4.0) as usize; // 16th notes
        Self {
            sample_rate,
            _bpm: bpm,
            channels,
            patterns_rx,
            step_tx,
            patterns: vec![],
            current_step: 0,
            samples_per_step,
            phase: 0,
            voices: vec![],
        }
    }

    fn fill_buffer(&mut self, output: &mut [f32]) {
        // Check for new patterns
        if let Ok(new_patterns) = self.patterns_rx.try_recv() {
            self.patterns = new_patterns;
            // Resize voices if needed
            if self.voices.len() != self.patterns.len() {
                self.voices = self
                    .patterns
                    .iter()
                    .enumerate()
                    .map(|(i, _)| {
                        let freq = 110.0 * (2.0f32).powf(i as f32 / 12.0); // Chromatic scale starting at A2
                        Voice {
                            frequency: freq,
                            envelope: 0.0,
                            active: false,
                        }
                    })
                    .collect();
            }
        }

        for frame in output.chunks_mut(self.channels) {
            // Clock tick
            if self.phase >= self.samples_per_step {
                self.phase = 0;
                self.current_step = (self.current_step + 1) % 16;
                let _ = self.step_tx.try_send(self.current_step);

                // Trigger voices
                for (i, voice) in self.voices.iter_mut().enumerate() {
                    if let Some(pattern) = self.patterns.get(i) {
                        if !pattern.is_empty() && pattern[self.current_step % pattern.len()] {
                            voice.envelope = 1.0;
                            voice.active = true;
                        }
                    }
                }
            }
            self.phase += 1;

            // Mix
            let mut sample_l = 0.0;
            let mut sample_r = 0.0;

            for (i, voice) in self.voices.iter_mut().enumerate() {
                if voice.active {
                    // Simple decay noise/sine for percussion feel.
                    let noise = (rand::random::<f32>() * 2.0 - 1.0) * 0.5;
                    let sine = (voice.frequency * self.phase as f32 * 2.0 * std::f32::consts::PI
                        / self.sample_rate)
                        .sin();

                    let signal = (sine * 0.5 + noise * 0.5) * voice.envelope;

                    // Pan
                    let pan = (i as f32 / self.voices.len().max(1) as f32) * 2.0 - 1.0; // -1 to 1

                    sample_l += signal * (1.0 - pan).min(1.0);
                    sample_r += signal * (1.0 + pan).min(1.0);

                    voice.envelope *= 0.9995; // Decay
                    if voice.envelope < 0.001 {
                        voice.active = false;
                    }
                }
            }

            // Soft clip / Limiter
            sample_l = sample_l.clamp(-0.8, 0.8);
            sample_r = sample_r.clamp(-0.8, 0.8);

            if self.channels >= 2 {
                frame[0] = sample_l;
                frame[1] = sample_r;
            } else {
                frame[0] = (sample_l + sample_r) * 0.5;
            }
        }
    }
}
