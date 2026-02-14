use std::f32::consts::PI;

pub struct Synthesizer {
    time: f32,
    pub base_feed: f32,
    pub base_kill: f32,
    pub feed_amp: f32,
    pub kill_amp: f32,
    pub frequency: f32,
}

impl Synthesizer {
    pub fn new() -> Self {
        Self {
            time: 0.0,
            base_feed: 0.055,
            base_kill: 0.062,
            feed_amp: 0.005,
            kill_amp: 0.002,
            frequency: 1.0, // 1 Hz beat
        }
    }

    pub fn update(&mut self, dt: f32) {
        self.time += dt;
    }

    pub fn get_params(&self) -> (f32, f32) {
        let feed = self.base_feed + self.feed_amp * (self.time * self.frequency * 2.0 * PI).sin();
        let kill =
            self.base_kill + self.kill_amp * (self.time * self.frequency * 2.0 * PI * 0.5).cos(); // Different rhythm
        (feed, kill)
    }

    pub fn get_waveform(&self, len: usize) -> Vec<f32> {
        let mut wave = Vec::with_capacity(len);
        for i in 0..len {
            let t = self.time - (len - i) as f32 * 0.01;
            let val = (t * self.frequency * 2.0 * PI).sin();
            wave.push(val);
        }
        wave
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_synthesizer_updates() {
        let mut synth = Synthesizer::new();
        synth.update(1.0);
        assert!(synth.time > 0.0);
    }

    #[test]
    fn test_param_range() {
        let mut synth = Synthesizer::new();
        // Simulate a few seconds
        for _ in 0..100 {
            synth.update(0.1);
            let (f, k) = synth.get_params();
            assert!(f >= synth.base_feed - synth.feed_amp - 0.0001);
            assert!(f <= synth.base_feed + synth.feed_amp + 0.0001);
            assert!(k >= synth.base_kill - synth.kill_amp - 0.0001);
            assert!(k <= synth.base_kill + synth.kill_amp + 0.0001);
        }
    }
}
