use rustfft::{Fft, FftPlanner, num_complex::Complex};
use std::sync::Arc;
use chimera_lang::prelude::*;

pub struct SpecterSynth {
    pub fft_size: usize,
    pub inverse_fft: Arc<dyn Fft<f32>>,
    pub buffer: Vec<Complex<f32>>,
    pub output_buffer: Vec<f32>,
}

impl SpecterSynth {
    pub fn new(fft_size: usize) -> Self {
        let mut planner = FftPlanner::new();
        let inverse_fft = planner.plan_fft_inverse(fft_size);

        Self {
            fft_size,
            inverse_fft,
            buffer: vec![Complex::new(0.0, 0.0); fft_size],
            output_buffer: vec![0.0; fft_size],
        }
    }

    pub fn grid_to_audio(&mut self, grid: &Vec<Vec<Value>>) -> &[f32] {
        // Clear buffer
        for i in 0..self.fft_size {
            self.buffer[i] = Complex::new(0.0, 0.0);
        }

        // Map grid rows to frequency bins
        // 16 rows.
        // We map them to bins 10, 20, 30... (approx 430Hz steps at 44.1k/1024)
        // Actually 44100 / 1024 = 43Hz per bin.
        // Bin 10 = 430Hz.
        // Bin 20 = 860Hz.

        for (row_idx, row) in grid.iter().enumerate() {
            let mut amplitude = 0.0;
            for cell in row {
                match cell {
                    Value::Int(v) => amplitude += (*v as f32).abs(),
                    Value::Str(s) => amplitude += s.len() as f32,
                    _ => {},
                }
            }

            // Normalize amplitude a bit
            amplitude = (amplitude / 50.0).clamp(0.0, 10.0);

            // Map row_idx to a bin
            let bin = 10 + row_idx * 5;

            if bin < self.fft_size / 2 {
                // Add some phase variation based on column?
                // For now, just magnitude.
                self.buffer[bin] = Complex::new(amplitude, 0.0);
                self.buffer[self.fft_size - bin] = Complex::new(amplitude, 0.0); // Conjugate symmetry for real output
            }
        }

        // Run IFFT
        self.inverse_fft.process(&mut self.buffer);

        // Normalize output
        let scale = 1.0 / (self.fft_size as f32);
        for (i, c) in self.buffer.iter().enumerate() {
            self.output_buffer[i] = c.re * scale;
        }

        &self.output_buffer
    }
}
