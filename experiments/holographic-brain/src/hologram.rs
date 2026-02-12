use num_complex::Complex;
use rustfft::FftPlanner;
use std::f64::consts::PI;

pub struct HolographicMemory {
    pub width: usize,
    pub height: usize,
    pub memory: Vec<Complex<f64>>, // Accumulated interference pattern (Frequency Domain? No, usually spatial)
    // Wait, in digital holography, we usually record the hologram in the spatial domain (CCD).
    // The hologram IS the interference pattern.
    // H(x,y) = |O(x,y) + R(x,y)|^2
    // But here we want to store it in frequency domain?
    // h(u,v) = FFT(H(x,y))?
    // Let's stick to the `hologram-text` approach which stores `data` as Complex<f64> which is the FFT.
    // In `hologram-text`, `from_text` does:
    // 1. Create spatial grid.
    // 2. Modulate with Reference Beam (phase shift).
    // 3. FFT.
    // So `data` is the FFT of (Object * Reference).
    // This is basically storing the Object spectrum shifted by the carrier frequency.

    // So `memory` here will be the sum of these FFTs.
}

impl HolographicMemory {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            memory: vec![Complex::new(0.0, 0.0); width * height],
        }
    }

    pub fn clear(&mut self) {
        for val in &mut self.memory {
            *val = Complex::new(0.0, 0.0);
        }
    }

    // Records an object wave (neuron firing pattern) into the hologram.
    // We assume a fixed Reference Beam R.
    // We are storing the Fourier Transform of the Object wave modulated by R.
    // F{ O * R } = F{O} * F{R}. Since R is a plane wave, F{R} is a delta function.
    // So we are just shifting the spectrum of O.
    pub fn record(&mut self, object_grid: &[f64], alpha: f64) {
        if object_grid.len() != self.width * self.height {
            return;
        }

        // 1. Modulate Object with Reference Beam
        // This effectively shifts the spectrum.
        let rec_angle_x = 20.0 * (2.0 * PI / self.width as f64);
        let rec_angle_y = 10.0 * (2.0 * PI / self.height as f64);

        let mut data: Vec<Complex<f64>> = object_grid
            .iter()
            .enumerate()
            .map(|(i, &val)| {
                let x = (i % self.width) as f64;
                let y = (i / self.width) as f64;
                let phase = rec_angle_x * x + rec_angle_y * y;
                let r = Complex::new(phase.cos(), phase.sin());
                // Object * Reference * Alpha
                Complex::new(val * alpha, 0.0) * r
            })
            .collect();

        // 2. FFT
        let mut planner = FftPlanner::new();
        let fft_x = planner.plan_fft_forward(self.width);
        let fft_y = planner.plan_fft_forward(self.height);

        // Rows
        for y in 0..self.height {
            let start = y * self.width;
            let end = start + self.width;
            fft_x.process(&mut data[start..end]);
        }
        transpose(&mut data, self.width, self.height);
        // Cols
        for x in 0..self.width {
            let start = x * self.height;
            let end = start + self.height;
            fft_y.process(&mut data[start..end]);
        }
        transpose(&mut data, self.height, self.width);

        // 3. Accumulate into Memory
        // We use a simple addition (superposition).
        // Normalize? Maybe slightly to prevent overflow, but f64 is large.
        // Let's just add.
        for (m, d) in self.memory.iter_mut().zip(data.iter()) {
            *m = *m + *d;
        }
    }

    // Reconstructs the object wave from the hologram.
    pub fn reconstruct(&self) -> Vec<f64> {
        let width = self.width;
        let height = self.height;

        // 1. Demodulate in Frequency Domain (Shift back)
        // Since we recorded at (+20, +10), the spectrum is centered there.
        // We want to shift it back to DC (-20, -10).
        let shift_x = -20;
        let shift_y = -10;

        let mut shifted_data = vec![Complex::new(0.0, 0.0); width * height];

        for y in 0..height {
            for x in 0..width {
                let src_x = (x as isize - shift_x).rem_euclid(width as isize) as usize;
                let src_y = (y as isize - shift_y).rem_euclid(height as isize) as usize;
                shifted_data[y * width + x] = self.memory[src_y * width + src_x];
            }
        }

        // 2. IFFT
        let mut planner = FftPlanner::new();
        let fft_x = planner.plan_fft_inverse(width);
        let fft_y = planner.plan_fft_inverse(height);

        let mut data = shifted_data;

        // Rows
        for y in 0..height {
            let start = y * width;
            let end = start + width;
            fft_x.process(&mut data[start..end]);
        }
        transpose(&mut data, width, height);
        // Cols
        for x in 0..width {
            let start = x * height;
            let end = start + height;
            fft_y.process(&mut data[start..end]);
        }
        transpose(&mut data, height, width);

        // 3. Magnitude
        let scale = 1.0 / (width * height) as f64;
        data.iter().map(|c| c.norm() * scale).collect()
    }

    pub fn get_magnitude_spectrum(&self) -> Vec<f64> {
        self.memory.iter().map(|c| (c.norm() + 1.0).ln()).collect()
    }
}

fn transpose(data: &mut [Complex<f64>], width: usize, height: usize) {
    let mut temp = vec![Complex::new(0.0, 0.0); width * height];
    for y in 0..height {
        for x in 0..width {
            temp[x * height + y] = data[y * width + x];
        }
    }
    data.copy_from_slice(&temp);
}
