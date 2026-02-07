use crossbeam::channel::Receiver;

#[allow(dead_code)]
pub enum AudioEvent {
    Kick,
    Snare,
    HiHat,
    Perc(f32),
}

pub struct AudioEngine {
    #[cfg(feature = "audio")]
    _stream: cpal::Stream,
}

impl AudioEngine {
    pub fn new(rx: Receiver<AudioEvent>) -> anyhow::Result<Self> {
        #[cfg(feature = "audio")]
        {
            use cpal::traits::{DeviceTrait, HostTrait, StreamTrait};

            let host = cpal::default_host();
            let device = host
                .default_output_device()
                .ok_or_else(|| anyhow::anyhow!("No audio device available"))?;
            let config = device.default_output_config()?;
            let sample_rate = config.sample_rate().0 as f32;
            let channels = config.channels() as usize;

            let mut synth = Synthesizer::new(sample_rate, rx);

            let stream = device.build_output_stream(
                &config.into(),
                move |data: &mut [f32], _: &cpal::OutputCallbackInfo| {
                    synth.fill_buffer(data, channels);
                },
                |err| eprintln!("Audio error: {}", err),
                None,
            )?;

            stream.play()?;

            Ok(Self { _stream: stream })
        }

        #[cfg(not(feature = "audio"))]
        {
            // Drain receiver
            std::thread::spawn(move || {
                while let Ok(_) = rx.recv() {}
            });
            Ok(Self {})
        }
    }
}

#[cfg(feature = "audio")]
struct Synthesizer {
    sample_rate: f32,
    rx: Receiver<AudioEvent>,
    voices: Vec<Voice>,
    rng: rand::rngs::ThreadRng,
}

#[cfg(feature = "audio")]
impl Synthesizer {
    fn new(sample_rate: f32, rx: Receiver<AudioEvent>) -> Self {
        Self {
            sample_rate,
            rx,
            voices: Vec::with_capacity(32),
            rng: rand::thread_rng(),
        }
    }

    fn fill_buffer(&mut self, output: &mut [f32], channels: usize) {
        use rand::Rng;

        // Process events
        while let Ok(event) = self.rx.try_recv() {
            match event {
                AudioEvent::Kick => self.voices.push(Voice::new_kick(self.sample_rate)),
                AudioEvent::Snare => self.voices.push(Voice::new_snare(self.sample_rate)),
                AudioEvent::HiHat => self.voices.push(Voice::new_hihat(self.sample_rate)),
                AudioEvent::Perc(pitch) => self
                    .voices
                    .push(Voice::new_perc(self.sample_rate, pitch)),
            }
        }

        for frame in output.chunks_mut(channels) {
            let mut mix = 0.0;
            let mut i = 0;
            while i < self.voices.len() {
                if let Some(sample) = self.voices[i].next_sample(&mut self.rng) {
                    mix += sample;
                    i += 1;
                } else {
                    self.voices.swap_remove(i);
                }
            }

            // Limiter
            mix = mix.clamp(-0.8, 0.8);

            for sample in frame.iter_mut() {
                *sample = mix;
            }
        }
    }
}

#[cfg(feature = "audio")]
struct Voice {
    t: f32,
    sample_rate: f32,
    kind: VoiceKind,
    finished: bool,
    phase: f32, // Track phase for frequency sweeps
}

#[cfg(feature = "audio")]
enum VoiceKind {
    Kick { freq_start: f32, freq_end: f32, decay: f32 },
    Snare { decay: f32 },
    HiHat { decay: f32 },
    Perc { freq: f32, decay: f32 },
}

#[cfg(feature = "audio")]
impl Voice {
    fn new_kick(sr: f32) -> Self {
        Self {
            t: 0.0,
            sample_rate: sr,
            finished: false,
            phase: 0.0,
            kind: VoiceKind::Kick {
                freq_start: 120.0,
                freq_end: 40.0,
                decay: 0.3,
            },
        }
    }

    fn new_snare(sr: f32) -> Self {
        Self {
            t: 0.0,
            sample_rate: sr,
            finished: false,
            phase: 0.0,
            kind: VoiceKind::Snare { decay: 0.2 },
        }
    }

    fn new_hihat(sr: f32) -> Self {
        Self {
            t: 0.0,
            sample_rate: sr,
            finished: false,
            phase: 0.0,
            kind: VoiceKind::HiHat { decay: 0.05 },
        }
    }

    fn new_perc(sr: f32, freq: f32) -> Self {
        Self {
            t: 0.0,
            sample_rate: sr,
            finished: false,
            phase: 0.0,
            kind: VoiceKind::Perc { freq, decay: 0.4 },
        }
    }

    fn next_sample(&mut self, rng: &mut rand::rngs::ThreadRng) -> Option<f32> {
        use rand::Rng;
        if self.finished {
            return None;
        }

        let dt = 1.0 / self.sample_rate;
        self.t += dt;

        match self.kind {
            VoiceKind::Kick { freq_start, freq_end, decay } => {
                if self.t > decay {
                    self.finished = true;
                    return None;
                }
                let amp = 1.0 - (self.t / decay);
                // Linear frequency sweep
                let freq = freq_start + (freq_end - freq_start) * (self.t / decay);
                self.phase += freq * dt * std::f32::consts::TAU;
                Some(self.phase.sin() * amp)
            }
            VoiceKind::Snare { decay } => {
                if self.t > decay {
                    self.finished = true;
                    return None;
                }
                let amp = 1.0 - (self.t / decay);
                // White noise
                let noise = rng.gen_range(-1.0..1.0);
                Some(noise * amp * 0.5)
            }
            VoiceKind::HiHat { decay } => {
                if self.t > decay {
                    self.finished = true;
                    return None;
                }
                let amp = 1.0 - (self.t / decay);
                // High frequency noise
                let noise = rng.gen_range(-1.0..1.0);
                // Very basic high pass filter simulation (just raw noise for now)
                Some(noise * amp * 0.3)
            }
            VoiceKind::Perc { freq, decay } => {
                 if self.t > decay {
                    self.finished = true;
                    return None;
                }
                let amp = (1.0 - (self.t / decay)).powf(2.0); // Exponential decay
                self.phase += freq * dt * std::f32::consts::TAU;
                // FM modulation
                let mod_idx = 2.0 * amp;
                let modulation = (self.phase * 2.5).sin() * mod_idx;
                Some((self.phase + modulation).sin() * amp * 0.4)
            }
        }
    }
}
