use crate::harvester::MusicalCommit;
use std::f32::consts::PI;

pub struct Synthesizer {
    phase: f32,
    mod_phase: f32,
    sample_rate: u32,
}

impl Synthesizer {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            phase: 0.0,
            mod_phase: 0.0,
            sample_rate,
        }
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
        let carrier_freq = Self::hash_to_freq(&commit.hash);

        // FM Synthesis: "Flux" determined by churn
        // Higher churn = higher modulation index = richer/noisier sound
        // Cap churn effect at some reasonable limit (e.g. 2000 lines)
        let mod_index = (commit.churn as f32).min(2000.0) / 100.0;
        let mod_freq = carrier_freq * 1.5; // Simple harmonic ratio

        // Advance phases
        self.phase += 2.0 * PI * carrier_freq / self.sample_rate as f32;
        if self.phase > 2.0 * PI {
            self.phase -= 2.0 * PI;
        }

        self.mod_phase += 2.0 * PI * mod_freq / self.sample_rate as f32;
        if self.mod_phase > 2.0 * PI {
            self.mod_phase -= 2.0 * PI;
        }

        // FM: Carrier modulated by Modulator
        let modulator = mod_index * self.mod_phase.sin();
        (self.phase + modulator).sin()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hash_to_freq() {
        let hash = "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2";
        let freq = Synthesizer::hash_to_freq(hash);
        assert!(
            (100.0..=1000.0).contains(&freq),
            "Frequency should be audible"
        );
    }

    #[test]
    fn test_generate_sample() {
        let mut synth = Synthesizer::new(44100);
        let commit = MusicalCommit {
            hash: "a1b2c3d4e5f6a1b2c3d4e5f6a1b2c3d4e5f6a1b2".to_string(),
            author: "Genesis".to_string(),
            timestamp: 1234567890,
            churn: 100,
        };
        let sample = synth.generate_next_sample(&commit);
        assert!(
            (-1.0..=1.0).contains(&sample),
            "Sample should be normalized"
        );
    }
}
