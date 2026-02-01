use crate::harvester::MusicalCommit;
use std::f32::consts::PI;

pub struct Synthesizer {
    phase: f32,
    sample_rate: u32,
}

impl Synthesizer {
    pub fn new(sample_rate: u32) -> Self {
        Self { phase: 0.0, sample_rate }
    }

    pub fn hash_to_freq(hash: &str) -> f32 {
        // Simple mapping: Take first 4 chars -> u16 -> map to range
        let slice = if hash.len() >= 4 { &hash[0..4] } else { hash };
        let val = u16::from_str_radix(slice, 16).unwrap_or(0);

        // Map 0..65535 to 100..1000 Hz
        let min_freq = 100.0;
        let max_freq = 1000.0;
        let ratio = val as f32 / 65535.0;

        min_freq + (ratio * (max_freq - min_freq))
    }

    pub fn generate_next_sample(&mut self, commit: &MusicalCommit) -> f32 {
        let freq = Self::hash_to_freq(&commit.hash);

        // Advance phase
        self.phase += 2.0 * PI * freq / self.sample_rate as f32;
        if self.phase > 2.0 * PI {
            self.phase -= 2.0 * PI;
        }

        // Sine wave
        self.phase.sin()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_to_freq() {
        let hash = "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2";
        let freq = Synthesizer::hash_to_freq(hash);
        assert!(freq >= 100.0 && freq <= 1000.0, "Frequency should be audible");
    }

    #[test]
    fn test_generate_sample() {
        let mut synth = Synthesizer::new(44100);
        let commit = MusicalCommit {
            hash: "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2".to_string(),
            author: "Genesis".to_string(),
            timestamp: 1234567890,
        };
        let sample = synth.generate_next_sample(&commit);
        assert!(sample >= -1.0 && sample <= 1.0, "Sample should be normalized");
    }
}
