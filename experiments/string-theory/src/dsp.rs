use rand::Rng;

pub struct KarplusStrong {
    buffer: Vec<f32>,
    cursor: usize,
    damping: f32, // Usually close to 0.99 - 0.999
}

impl KarplusStrong {
    pub fn new(freq: f32, sample_rate: f32, damping: f32) -> Self {
        // Frequency check to avoid division by zero or huge buffers
        let safe_freq = freq.clamp(20.0, sample_rate / 2.0);
        let len = (sample_rate / safe_freq).round() as usize;
        // Ensure at least 2 samples
        let len = len.max(2);

        Self {
            buffer: vec![0.0; len],
            cursor: 0,
            damping,
        }
    }

    pub fn pluck(&mut self, intensity: f32) {
        let mut rng = rand::thread_rng();
        for sample in self.buffer.iter_mut() {
            *sample = (rng.gen::<f32>() * 2.0 - 1.0) * intensity;
        }
        // Reset cursor on pluck? Not strictly necessary but clean.
        // Actually, for KS, the noise IS the initial state of the delay line.
        // If we pluck while ringing, we overwrite the string state (hard pluck).
        self.cursor = 0;
    }

    pub fn tick(&mut self) -> f32 {
        let current_val = self.buffer[self.cursor];

        // Karplus-Strong Lowpass filter: y[n] = 0.5 * (y[n-L] + y[n-L-1])
        // We act on the buffer in place.
        // We want to write the new value into the CURRENT slot, effectively "pushing" it L steps into the future (since we circle back).
        // Wait, standard ring buffer implementation:
        // Read at cursor.
        // Read at (cursor + 1) (which is the "previous" sample in the delay line if we view it as moving backwards, or "next" if we view it as moving forward).
        // Actually, the simple algorithm is:
        // val = buffer[i]
        // next_val = buffer[(i+1)%L]
        // new_val = (val + next_val) * 0.5 * decay
        // buffer[i] = new_val
        // i = (i+1)%L
        // return val

        let next_idx = (self.cursor + 1) % self.buffer.len();
        let next_val = self.buffer[next_idx];

        let new_val = (current_val + next_val) * 0.5 * self.damping;

        self.buffer[self.cursor] = new_val;
        self.cursor = next_idx;

        current_val
    }

    // Get current energy (RMS-ish) for visualization
    pub fn energy(&self) -> f32 {
        // Optimization: Don't scan entire buffer every frame.
        // But for visualization, maybe just return the last output or a subsample.
        // Let's iterate, it's fine for small N.
        self.buffer.iter().map(|x| x.abs()).sum::<f32>() / (self.buffer.len() as f32)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decay() {
        let mut string = KarplusStrong::new(440.0, 44100.0, 0.99);
        string.pluck(1.0);

        let initial_energy = string.energy();
        assert!(initial_energy > 0.0, "String should have energy after pluck");

        // Simulate 1000 ticks
        for _ in 0..1000 {
            string.tick();
        }

        let final_energy = string.energy();
        assert!(final_energy < initial_energy, "Energy should decay");
        assert!(final_energy > 0.0, "Energy should not be exactly zero yet");
    }

    #[test]
    fn test_freq_bounds() {
        let string = KarplusStrong::new(0.001, 44100.0, 0.99);
        assert!(string.buffer.len() < 100000, "Should clamp low freq to avoid huge buffer");

        let string_high = KarplusStrong::new(100000.0, 44100.0, 0.99);
        assert!(string_high.buffer.len() >= 2, "Should have at least 2 samples");
    }
}
