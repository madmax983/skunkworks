use hound;
use std::f32::consts::PI;

pub struct Musician {
    pub buffer: Vec<i16>,
    sample_rate: u32,
}

impl Musician {
    pub fn new() -> Self {
        Self {
            buffer: Vec::new(),
            sample_rate: 44100,
        }
    }

    pub fn play_note(&mut self, frequency: f32, duration: f32) {
        let sample_count = (self.sample_rate as f32 * duration) as usize;
        let amplitude = i16::MAX as f32 * 0.2; // 20% volume to avoid clipping when mixing potentially

        for i in 0..sample_count {
            let t = i as f32 / self.sample_rate as f32;
            // Simple Sine Wave
            let sample = (t * frequency * 2.0 * PI).sin() * amplitude;
            self.buffer.push(sample as i16);
        }
    }

    pub fn save(&self, path: &str) -> anyhow::Result<()> {
        let spec = hound::WavSpec {
            channels: 1,
            sample_rate: self.sample_rate,
            bits_per_sample: 16,
            sample_format: hound::SampleFormat::Int,
        };
        let mut writer = hound::WavWriter::create(path, spec)?;
        for sample in &self.buffer {
            writer.write_sample(*sample)?;
        }
        writer.finalize()?;
        Ok(())
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_play_note() {
        let mut musician = Musician::new();
        musician.play_note(440.0, 1.0);
        assert!(!musician.buffer.is_empty());
        assert_eq!(musician.buffer.len(), 44100);
    }
}
