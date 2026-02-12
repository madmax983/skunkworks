use crate::platter::Platter;
use rand::Rng;

pub struct DecayEngine {
    pub temperature: f64, // Probability of a byte being affected per update
}

impl DecayEngine {
    pub fn new() -> Self {
        Self { temperature: 0.0001 }
    }

    pub fn apply_entropy(&self, platter: &mut Platter) {
        let mut rng = rand::thread_rng();

        if self.temperature <= 0.0 {
            return;
        }

        // Scale by capacity to determine number of flips
        let num_flips = (platter.capacity as f64 * self.temperature).ceil() as usize;

        for _ in 0..num_flips {
            let byte_idx = rng.gen_range(0..platter.capacity);
            let bit_idx = rng.gen_range(0..8);
            platter.flip_bit(byte_idx, bit_idx);
        }
    }
}
