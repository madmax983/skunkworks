use anyhow::Result;
use hound;
use std::f32::consts::PI;

#[derive(Debug, Clone, Copy)]
pub enum Waveform {
    Sine,
    Square,
    Saw,
    Triangle,
}

#[derive(Debug, Clone, Copy)]
pub struct Adsr {
    pub attack: f32,        // duration in seconds
    pub decay: f32,         // duration in seconds
    pub sustain_level: f32, // amplitude 0.0 - 1.0
    pub release: f32,       // duration in seconds
}

impl Default for Adsr {
    fn default() -> Self {
        Self {
            attack: 0.01,
            decay: 0.1,
            sustain_level: 0.7,
            release: 0.1,
        }
    }
}

pub struct SynthSource {
    freq: f32,
    sample_rate: u32,
    num_sample: usize,
    waveform: Waveform,
    adsr: Adsr,
    duration_samples: usize,
    volume: f32,
}

impl SynthSource {
    pub fn new(freq: f32, duration: f32, volume: f32, waveform: Waveform, adsr: Adsr) -> Self {
        let sample_rate = 44100;
        let duration_samples = (duration * sample_rate as f32) as usize;
        Self {
            freq,
            sample_rate,
            num_sample: 0,
            waveform,
            adsr,
            duration_samples,
            volume,
        }
    }

    fn apply_envelope(&self, sample_idx: usize, raw_sample: f32) -> f32 {
        let t = sample_idx as f32 / self.sample_rate as f32;
        let total_duration = self.duration_samples as f32 / self.sample_rate as f32;

        // ADSR Logic
        let amp = if t < self.adsr.attack {
            t / self.adsr.attack
        } else if t < self.adsr.attack + self.adsr.decay {
            let decay_progress = (t - self.adsr.attack) / self.adsr.decay;
            1.0 - (1.0 - self.adsr.sustain_level) * decay_progress
        } else if t < total_duration - self.adsr.release {
            self.adsr.sustain_level
        } else if t < total_duration {
            let release_start = total_duration - self.adsr.release;
            let release_progress = (t - release_start) / self.adsr.release;
            self.adsr.sustain_level * (1.0 - release_progress)
        } else {
            0.0
        };

        raw_sample * amp * self.volume
    }
}

impl Iterator for SynthSource {
    type Item = f32;

    fn next(&mut self) -> Option<Self::Item> {
        if self.num_sample >= self.duration_samples {
            return None;
        }

        let t = self.num_sample as f32 / self.sample_rate as f32;
        let phase = t * self.freq * 2.0 * PI;

        let raw_sample = match self.waveform {
            Waveform::Sine => phase.sin(),
            Waveform::Square => if phase.sin() >= 0.0 { 1.0 } else { -1.0 },
            Waveform::Saw => {
                let p = (t * self.freq) % 1.0;
                2.0 * p - 1.0
            },
            Waveform::Triangle => {
                let p = (t * self.freq) % 1.0;
                if p < 0.5 {
                    4.0 * p - 1.0
                } else {
                    3.0 - 4.0 * p
                }
            }
        };

        let sample = self.apply_envelope(self.num_sample, raw_sample);
        self.num_sample += 1;
        Some(sample)
    }
}

pub struct AudioEngine {
    active_voices: Vec<SynthSource>,
    master_buffer: Vec<f32>,
    pub current_amplitude: f32, // For visualization
    sample_rate: u32,
    samples_per_tick: usize, // e.g., 60 FPS -> 44100 / 60
}

impl AudioEngine {
    pub fn new() -> Result<Self> {
        Ok(Self {
            active_voices: Vec::new(),
            master_buffer: Vec::new(),
            current_amplitude: 0.0,
            sample_rate: 44100,
            samples_per_tick: 44100 / 60, // ~735 samples per update
        })
    }

    pub fn ensure_voice_capacity(&mut self, _capacity: usize) {
        // No-op for virtual engine
    }

    pub fn play_synth_note(&mut self, _voice_idx: usize, frequency: f32, duration: f32, volume: f32, waveform: Waveform, adsr: Adsr) {
        let source = SynthSource::new(frequency, duration, volume, waveform, adsr);
        self.active_voices.push(source);
    }

    pub fn play_note(&mut self, voice_idx: usize, frequency: f32, duration: f32, volume: f32) {
        self.play_synth_note(voice_idx, frequency, duration, volume, Waveform::Sine, Adsr::default());
    }

    pub fn is_voice_busy(&self, _voice_idx: usize) -> bool {
        // In virtual engine, we can check if any voice is active,
        // but the caller uses this to check if a specific voice finished.
        // Since we mix everything into one buffer, we don't track by index easily unless we store map.
        // For now, let's just return false to let the TUI advance freely,
        // or true if we want to block?
        // Let's return false so TUI dictates the pace based on token duration.
        false
    }

    // Called every frame by TUI loop
    pub fn update(&mut self, dt: f32) {
        let mut tick_max_amp = 0.0;
        let num_samples = (dt * self.sample_rate as f32) as usize;

        for _ in 0..num_samples {
            let mut sample_sum = 0.0;

            // Advance all voices
            self.active_voices.retain_mut(|voice| {
                if let Some(s) = voice.next() {
                    sample_sum += s;
                    true
                } else {
                    false
                }
            });

            // Hard limiter
            if sample_sum > 1.0 { sample_sum = 1.0; }
            if sample_sum < -1.0 { sample_sum = -1.0; }

            self.master_buffer.push(sample_sum);
            if sample_sum.abs() > tick_max_amp {
                tick_max_amp = sample_sum.abs();
            }
        }

        self.current_amplitude = tick_max_amp;
    }

    pub fn save_wav(&self, path: &str) -> Result<()> {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: self.sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(path, spec)?;
        let amplitude = i16::MAX as f32;

        for &sample in &self.master_buffer {
            writer.write_sample((sample * amplitude) as i16)?;
        }
        writer.finalize()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synth_source_generation() {
        let source = SynthSource::new(440.0, 1.0, 0.5, Waveform::Sine, Adsr::default());
        let samples: Vec<f32> = source.take(100).collect();
        assert_eq!(samples.len(), 100);
        for s in samples {
            assert!(s >= -1.0 && s <= 1.0);
        }
    }
}
