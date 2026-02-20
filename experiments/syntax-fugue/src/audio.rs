use anyhow::Result;
use std::f32::consts::PI;

#[cfg(feature = "audio")]
use rodio::{OutputStream, OutputStreamHandle, Source};

#[derive(Debug, Clone, Copy, PartialEq)]
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

        // Clip amplitude to 0 if time is beyond duration (though Iterator handles this too)
        if t > total_duration {
            return 0.0;
        }

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

#[cfg(feature = "audio")]
impl Source for SynthSource {
    fn current_frame_len(&self) -> Option<usize> {
        None
    }
    fn channels(&self) -> u16 {
        1
    }
    fn sample_rate(&self) -> u32 {
        self.sample_rate
    }
    fn total_duration(&self) -> Option<std::time::Duration> {
        Some(std::time::Duration::from_secs_f32(self.duration_samples as f32 / self.sample_rate as f32))
    }
}

pub struct AudioEngine {
    active_voices: Vec<SynthSource>,
    master_buffer: Vec<f32>,
    pub current_amplitude: f32, // For visualization
    sample_rate: u32,

    #[cfg(feature = "audio")]
    _stream: Option<OutputStream>,
    #[cfg(feature = "audio")]
    stream_handle: Option<OutputStreamHandle>,
}

impl AudioEngine {
    pub fn new() -> Result<Self> {
        #[cfg(feature = "audio")]
        let (stream, stream_handle) = {
            match OutputStream::try_default() {
                Ok((s, h)) => (Some(s), Some(h)),
                Err(_) => (None, None), // Fallback if no audio device
            }
        };

        Ok(Self {
            active_voices: Vec::new(),
            master_buffer: Vec::new(),
            current_amplitude: 0.0,
            sample_rate: 44100,
            #[cfg(feature = "audio")]
            _stream: stream,
            #[cfg(feature = "audio")]
            stream_handle,
        })
    }

    pub fn ensure_voice_capacity(&mut self, _capacity: usize) {
        // No-op for virtual engine
    }

    pub fn play_synth_note(&mut self, _voice_idx: usize, frequency: f32, duration: f32, volume: f32, waveform: Waveform, adsr: Adsr) {
        let source = SynthSource::new(frequency, duration, volume, waveform, adsr);

        // Push to active voices for VISUALIZATION and WAV recording
        // We clone the source? SynthSource is not cloneable easily (stateful iterator).
        // So we create two sources?

        // Create a copy for Realtime Audio
        #[cfg(feature = "audio")]
        if let Some(handle) = &self.stream_handle {
             let realtime_source = SynthSource::new(frequency, duration, volume, waveform, adsr);
             if let Err(e) = handle.play_raw(realtime_source) {
                 eprintln!("Audio playback error: {}", e);
             }
        }

        // Push original to active_voices for offline rendering / visualization
        self.active_voices.push(source);
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
            sample_sum = sample_sum.clamp(-1.0, 1.0);

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
