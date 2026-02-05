use rustfft::{FftPlanner, num_complex::Complex};
use std::f32::consts::PI;

pub struct SignalGenerator {
    sample_rate: u32,
    phase: f32,
    time: f32,
}

impl SignalGenerator {
    pub fn new(sample_rate: u32) -> Self {
        Self {
            sample_rate,
            phase: 0.0,
            time: 0.0,
        }
    }

    pub fn generate_chunk(&mut self, buffer: &mut [f32]) {
        for sample in buffer.iter_mut() {
            // Complex signal generation for "painting"
            // Base drone
            let t = self.time;

            // LFO for frequency sweep (Slow rise and fall)
            // 0.1 Hz LFO
            let lfo = (t * 0.2 * 2.0 * PI).sin();

            // Map LFO to frequency range 100Hz - 2000Hz
            let freq = 200.0 + 1800.0 * (0.5 + 0.5 * lfo);

            // Sine wave at current frequency
            // Note: strict FM synthesis requires integrating frequency, but for slow changes this is "ok" enough for visual art
            // To be correct: phase += freq / sample_rate
            self.phase += freq / self.sample_rate as f32;
            if self.phase > 1.0 { self.phase -= 1.0; }

            let signal = (self.phase * 2.0 * PI).sin();

            // Add harmonics (High frequency detail)
            let harmonic = (self.phase * 5.0 * 2.0 * PI).sin() * 0.3;

            // Add sub-bass (steady 50Hz)
            let sub_phase = (t * 50.0 * 2.0 * PI).rem_euclid(2.0 * PI);
            let sub = sub_phase.sin() * 0.5;

            // White noise for texture (Brush strokes)
            let noise = (rand::random::<f32>() - 0.5) * 0.1;

            *sample = (signal + harmonic + sub + noise).clamp(-1.0, 1.0);

            self.time += 1.0 / self.sample_rate as f32;
        }
    }
}

pub struct SpectralAnalyzer {
    planner: FftPlanner<f32>,
    fft_size: usize,
    scratch: Vec<Complex<f32>>,
}

impl SpectralAnalyzer {
    pub fn new(fft_size: usize) -> Self {
        Self {
            planner: FftPlanner::new(),
            fft_size,
            scratch: vec![Complex { re: 0.0, im: 0.0 }; fft_size],
        }
    }

    pub fn compute_spectrum(&mut self, samples: &[f32], output: &mut [f32]) {
        let fft = self.planner.plan_fft_forward(self.fft_size);

        // Copy to scratch
        for (i, &s) in samples.iter().take(self.fft_size).enumerate() {
            self.scratch[i] = Complex { re: s, im: 0.0 };
        }
        // Zero pad if needed
        if samples.len() < self.fft_size {
             for i in samples.len()..self.fft_size {
                 self.scratch[i] = Complex { re: 0.0, im: 0.0 };
             }
        }

        fft.process(&mut self.scratch);

        // Compute magnitude
        let nyquist = self.fft_size / 2;
        for (i, c) in self.scratch.iter().take(nyquist).enumerate() {
             if i < output.len() {
                 output[i] = c.norm();
             }
        }
    }
}
