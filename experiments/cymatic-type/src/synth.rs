#[derive(Debug, Clone, Copy)]
pub struct Spectrum {
    pub bass: f32,
    pub mid: f32,
    pub treble: f32,
}

pub struct AudioEngine {
    pub time: f32,
}

impl AudioEngine {
    pub fn new() -> Self {
        Self { time: 0.0 }
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;
    }

    pub fn get_spectrum(&self) -> Spectrum {
        // Simulate a "Track" with some loops
        let t = self.time;

        // Bass: Kick drum pattern (every 1.0s) + Sub bass
        // Kick is a sharp spike decaying quickly
        let beat = t % 1.0;
        let kick = if beat < 0.2 { (-10.0 * beat).exp() } else { 0.0 }; // Simple impulse
        let sub = (t * 0.5).sin() * 0.5 + 0.5; // Slow undulating sub
        let bass = (kick + sub).clamp(0.0, 1.0);

        // Mid: Melody / Synth
        // Sine wave at 3Hz modulated by another sine
        let mid = ((t * 3.0).sin() * (t * 0.5).cos()).abs();

        // Treble: Hi-hats (every 0.25s) + Noise
        let hat_beat = t % 0.25;
        let hat = (-20.0 * hat_beat).exp();
        let noise = (t * 123.456).sin().fract().abs(); // Pseudo-random
        let treble = (hat * 0.8 + noise * 0.2).clamp(0.0, 1.0);

        Spectrum { bass, mid, treble }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_spectrum_range() {
        let mut engine = AudioEngine::new();
        for _ in 0..100 {
            engine.update(0.1);
            let s = engine.get_spectrum();
            assert!(s.bass >= 0.0 && s.bass <= 1.0, "Bass out of range: {}", s.bass);
            assert!(s.mid >= 0.0 && s.mid <= 1.0, "Mid out of range: {}", s.mid);
            assert!(s.treble >= 0.0 && s.treble <= 1.0, "Treble out of range: {}", s.treble);
        }
    }
}
