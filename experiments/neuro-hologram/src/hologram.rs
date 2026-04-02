use num_complex::Complex;
use rustfft::FftPlanner;
use std::f64::consts::PI;

pub struct Hologram {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Complex<f64>>,
}

impl Hologram {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Complex::new(0.0, 0.0); width * height],
        }
    }

    pub fn clear(&mut self) {
        for d in self.data.iter_mut() {
            *d = Complex::new(0.0, 0.0);
        }
    }

    pub fn reconstruct(&self, shift_x_bins: isize, shift_y_bins: isize) -> Vec<f64> {
        let mut planner = FftPlanner::new();
        let fft_row = planner.plan_fft_inverse(self.width);
        let fft_col = planner.plan_fft_inverse(self.height);

        let mut reconstructed = self.data.clone();

        // 1. Shift the frequency domain to emulate changing viewing angle
        let mut shifted = vec![Complex::new(0.0, 0.0); self.width * self.height];
        for y in 0..self.height {
            for x in 0..self.width {
                let sx = (x as isize - shift_x_bins).rem_euclid(self.width as isize) as usize;
                let sy = (y as isize - shift_y_bins).rem_euclid(self.height as isize) as usize;
                shifted[sy * self.width + sx] = reconstructed[y * self.width + x];
            }
        }
        reconstructed = shifted;

        // 2. 2D IFFT
        // Rows
        for y in 0..self.height {
            let start = y * self.width;
            let end = start + self.width;
            fft_row.process(&mut reconstructed[start..end]);
        }

        // Cols (Transpose, FFT, Transpose back for simplicity)
        let mut col_data = vec![Complex::new(0.0, 0.0); self.height];
        for x in 0..self.width {
            for y in 0..self.height {
                col_data[y] = reconstructed[y * self.width + x];
            }
            fft_col.process(&mut col_data);
            for y in 0..self.height {
                reconstructed[y * self.width + x] = col_data[y];
            }
        }

        // 3. Extract Intensity (Magnitude Squared) and normalize
        let mut intensities = vec![0.0; self.width * self.height];
        let mut max_intensity = 0.0_f64;

        for i in 0..reconstructed.len() {
            let magnitude = reconstructed[i].norm();
            let intensity = magnitude * magnitude;
            intensities[i] = intensity;
            if intensity > max_intensity {
                max_intensity = intensity;
            }
        }

        if max_intensity > 0.0 {
            for v in intensities.iter_mut() {
                *v /= max_intensity;
            }
        }

        intensities
    }

    pub fn get_magnitude(&self) -> Vec<f64> {
        let mut mag = vec![0.0; self.width * self.height];
        let mut max = 0.0_f64;
        for i in 0..self.data.len() {
            let m = self.data[i].norm();
            mag[i] = m;
            if m > max {
                max = m;
            }
        }
        if max > 0.0 {
            for v in mag.iter_mut() {
                *v /= max;
            }
        }
        mag
    }
}
