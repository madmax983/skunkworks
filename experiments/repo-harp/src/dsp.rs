use rand::Rng;

#[derive(Debug, Clone)]
pub struct KarplusStrong {
    buffer: Vec<f32>,
    index: usize,
    feedback: f32,
}

impl KarplusStrong {
    pub fn new(freq: f32, sample_rate: f32) -> Self {
        let period = sample_rate / freq;
        let len = period.round() as usize;
        // Ensure at least length 1 to avoid panic
        let len = len.max(1);

        Self {
            buffer: vec![0.0; len],
            index: 0,
            feedback: 0.996, // Slightly less than 1.0 for decay
        }
    }

    pub fn pluck(&mut self) {
        let mut rng = rand::thread_rng();
        for sample in self.buffer.iter_mut() {
            *sample = rng.gen_range(-1.0..=1.0);
        }
    }

    pub fn tick(&mut self) -> f32 {
        let len = self.buffer.len();
        let current_val = self.buffer[self.index];
        let next_index = (self.index + 1) % len;
        let next_val = self.buffer[next_index];

        // Low-pass filter + Decay
        let new_val = 0.5 * (current_val + next_val) * self.feedback;

        // Update buffer
        self.buffer[self.index] = new_val;

        // Advance index
        self.index = next_index;

        current_val
    }

    pub fn current_value(&self) -> f32 {
        if self.buffer.is_empty() { 0.0 } else { self.buffer[self.index] }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_karplus_strong_decay() {
        let sample_rate = 44100.0;
        let freq = 440.0;
        let mut string = KarplusStrong::new(freq, sample_rate);

        string.pluck();

        // Measure initial energy
        let mut initial_energy = 0.0;
        for _ in 0..100 {
            initial_energy += string.tick().abs();
        }

        // Run for a while
        for _ in 0..5000 {
            string.tick();
        }

        // Measure final energy
        let mut final_energy = 0.0;
        for _ in 0..100 {
            final_energy += string.tick().abs();
        }

        assert!(initial_energy > 0.0, "Initial energy should be > 0 after pluck");
        assert!(final_energy < initial_energy, "Energy should decay over time");

        // Also ensure it didn't blow up (NaN check or Infinity)
        assert!(final_energy.is_finite());
    }
}
