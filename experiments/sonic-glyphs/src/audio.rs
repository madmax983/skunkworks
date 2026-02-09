use rand::Rng;

pub struct AudioSim {
    pub time: f64,
    pub bass: f32,
    pub mid: f32,
    pub treble: f32,
    pub bpm: f32,
    pub last_beat: f64,
    pub is_beat: bool,
}

impl AudioSim {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            bass: 0.0,
            mid: 0.0,
            treble: 0.0,
            bpm: 120.0,
            last_beat: 0.0,
            is_beat: false,
        }
    }

    pub fn update(&mut self, dt: f64) {
        self.time += dt;
        let mut rng = rand::thread_rng();

        // Simulate Bass (Kick drum pattern)
        // 120 BPM = 2 beats per second = 0.5s per beat
        let beat_duration = 60.0 / self.bpm as f64;
        let beat_phase = (self.time % beat_duration) / beat_duration;

        // Pulse at the start of the beat
        let kick = (-10.0 * beat_phase).exp(); // sharp decay

        self.bass = kick as f32 * 0.8 + (self.time as f32 * 0.5).sin().abs() * 0.2;

        // Simulate Mid (Synth melody)
        // Sine wave + noise
        self.mid = (self.time as f32 * 3.0).sin().abs() * 0.5 + rng.gen::<f32>() * 0.2;

        // Simulate Treble (Hi-hats)
        // Fast random spikes
        if rng.gen::<f32>() > 0.8 {
            self.treble = rng.gen::<f32>();
        } else {
            self.treble *= 0.8; // decay
        }

        // Beat detection logic
        if self.time - self.last_beat >= beat_duration {
            self.is_beat = true;
            self.last_beat = self.time;
        } else {
            self.is_beat = false;
        }

        // Clamp values
        self.bass = self.bass.clamp(0.0, 1.0);
        self.mid = self.mid.clamp(0.0, 1.0);
        self.treble = self.treble.clamp(0.0, 1.0);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_audio_sim() {
        let mut audio = AudioSim::new();
        audio.update(0.1);

        assert!(audio.bass >= 0.0 && audio.bass <= 1.0);
        assert!(audio.mid >= 0.0 && audio.mid <= 1.0);
        assert!(audio.treble >= 0.0 && audio.treble <= 1.0);

        // Run for a bit to trigger a beat
        for _ in 0..10 {
            audio.update(0.1);
        }
    }
}
