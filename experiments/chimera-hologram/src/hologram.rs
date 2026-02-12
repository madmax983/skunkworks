use chimera_lang::prelude::*;
use num_complex::Complex;
use rand::Rng;
use rustfft::FftPlanner;
use strum::IntoEnumIterator;

/// A genome stored in the Frequency Domain as a holographic interference pattern.
pub struct HolographicDna {
    /// The complex coefficients of the hologram.
    pub coefficients: Vec<Complex<f64>>,
}

impl HolographicDna {
    /// Creates a new random holographic genome.
    pub fn new_random(size: usize) -> Self {
        let mut rng = rand::thread_rng();
        let coefficients: Vec<Complex<f64>> = (0..size)
            .map(|_| {
                // Random magnitude and phase
                let re = rng.gen_range(-1.0..1.0);
                let im = rng.gen_range(-1.0..1.0);
                Complex::new(re, im)
            })
            .collect();
        Self { coefficients }
    }

    /// Decodes the hologram into a sequence of OpCodes based on a reference beam angle (phase shift).
    ///
    /// The phase shift simulates looking at the hologram from a different angle.
    /// This changes the interference pattern, resulting in different instructions.
    pub fn decode(&self, phase_shift: f64) -> Vec<OpCode> {
        let size = self.coefficients.len();
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_inverse(size);

        // Apply phase shift and prepare buffer
        // Shift factor: e^(i * phase) = cos(phase) + i*sin(phase)
        let shift = Complex::from_polar(1.0, phase_shift);

        // We need to convert num_complex::Complex to rustfft::num_complex::Complex if they are different versions,
        // but typically they are compatible or the same type alias.
        // Let's assume they are compatible.
        let mut buffer: Vec<Complex<f64>> = self.coefficients
            .iter()
            .map(|c| c * shift)
            .collect();

        // Perform Inverse FFT
        fft.process(&mut buffer);

        // Map the signal (real part) to OpCodes
        // We normalize the signal to map to the opcode range.
        let opcodes: Vec<OpCode> = OpCode::iter().collect();
        let opcode_count = opcodes.len();

        // Scaling factor to ensure we hit higher index opcodes
        // FFT magnitude scales with sqrt(N) * input_magnitude approx.
        // If N=64, sqrt(N)=8. Input ~0.5. Result ~4.
        // We want indices up to ~150. So we need a multiplier of ~40.
        // Let's pick 100.0 to be safe and wrap around.
        let scale = 100.0;

        buffer
            .iter()
            .map(|c| {
                // Use the real part of the signal (constructive/destructive interference result)
                // This is the "Holographic Projection" onto the Real axis.
                let val = c.re;

                // Map magnitude to index using modulo.
                let index = ((val.abs() * scale) as usize) % opcode_count;
                opcodes[index].clone()
            })
            .collect()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decode_deterministic() {
        let dna = HolographicDna::new_random(16);
        let ops1 = dna.decode(0.0);
        let ops2 = dna.decode(0.0);
        assert_eq!(ops1, ops2);
    }

    #[test]
    fn test_phase_shift_changes_phenotype() {
        let dna = HolographicDna::new_random(16);
        let ops1 = dna.decode(0.0);
        let ops2 = dna.decode(std::f64::consts::PI / 2.0);

        // It is extremely unlikely that a random hologram produces the exact same code
        // when phase shifted by 90 degrees, unless it's empty.
        assert_ne!(ops1, ops2);
    }
}
