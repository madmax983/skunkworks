use num_complex::Complex;
use rustfft::FftPlanner;
use std::f64::consts::PI;

pub struct Hologram {
    pub width: usize,
    pub height: usize,
    pub data: Vec<Complex<f64>>,
}

impl Hologram {
    #[allow(dead_code)]
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![Complex::new(0.0, 0.0); width * height],
        }
    }

    pub fn from_grid(
        width: usize,
        height: usize,
        grid: &[f64],
        rec_shift_x: f64,
        rec_shift_y: f64,
    ) -> Self {
        // Apply Reference Beam (Recording Angle)
        // We simulate recording with a reference beam at a specific angle.
        // This modulates the object wave in the spatial domain.
        // When we take the FFT, this shifts the spectrum away from DC.
        let rec_angle_x = rec_shift_x * (2.0 * PI / width as f64);
        let rec_angle_y = rec_shift_y * (2.0 * PI / height as f64);

        let mut data: Vec<Complex<f64>> = grid
            .iter()
            .enumerate()
            .map(|(i, &val)| {
                let x = (i % width) as f64;
                let y = (i / width) as f64;
                // Modulation: exp(i * (kx*x + ky*y))
                let phase = rec_angle_x * x + rec_angle_y * y;
                let r = Complex::new(phase.cos(), phase.sin());
                Complex::new(val, 0.0) * r
            })
            .collect();

        // 2D FFT
        let mut planner = FftPlanner::new();
        let fft_x = planner.plan_fft_forward(width);
        let fft_y = planner.plan_fft_forward(height);

        // FFT Rows
        for y in 0..height {
            let start = y * width;
            let end = start + width;
            fft_x.process(&mut data[start..end]);
        }

        // Transpose
        transpose(&mut data, width, height);

        // FFT Columns (which are now rows in memory)
        for x in 0..width {
            let start = x * height;
            let end = start + height;
            fft_y.process(&mut data[start..end]);
        }

        // Transpose back
        transpose(&mut data, height, width);

        Self {
            width,
            height,
            data,
        }
    }

    pub fn reconstruct(&self, shift_x_bins: isize, shift_y_bins: isize) -> Vec<f64> {
        let width = self.width;
        let height = self.height;

        // Circular Shift in Frequency Domain
        // This simulates changing the reconstruction angle to match the recording angle.
        // If shift matches recording shift (-20, -10), we move the spectrum back to DC.

        let mut shifted_data = vec![Complex::new(0.0, 0.0); width * height];

        for y in 0..height {
            for x in 0..width {
                let src_x = (x as isize - shift_x_bins).rem_euclid(width as isize) as usize;
                let src_y = (y as isize - shift_y_bins).rem_euclid(height as isize) as usize;
                shifted_data[y * width + x] = self.data[src_y * width + src_x];
            }
        }

        // Inverse 2D FFT
        let mut planner = FftPlanner::new();
        let fft_x = planner.plan_fft_inverse(width);
        let fft_y = planner.plan_fft_inverse(height);

        let mut data = shifted_data;

        // IFFT Rows
        for y in 0..height {
            let start = y * width;
            let end = start + width;
            fft_x.process(&mut data[start..end]);
        }

        transpose(&mut data, width, height);

        // IFFT Columns
        for x in 0..width {
            let start = x * height;
            let end = start + height;
            fft_y.process(&mut data[start..end]);
        }

        transpose(&mut data, height, width);

        // Return magnitude normalized
        let scale = 1.0 / (width * height) as f64;
        data.iter().map(|c| c.norm() * scale).collect()
    }

    #[allow(dead_code)]
    pub fn get_magnitude(&self) -> Vec<f64> {
        // Log-scale magnitude for better visualization of spectrum
        self.data.iter().map(|c| (c.norm() + 1.0).ln()).collect()
    }
}

fn transpose(data: &mut [Complex<f64>], width: usize, height: usize) {
    let mut temp = vec![Complex::new(0.0, 0.0); width * height];
    for y in 0..height {
        for x in 0..width {
            // Original: [y][x] -> [y * width + x]
            // Transposed: [x][y] -> [x * height + y]
            temp[x * height + y] = data[y * width + x];
        }
    }
    data.copy_from_slice(&temp);
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_hologram_roundtrip() {
        let mut grid = vec![0.0; 64 * 32];
        grid[10] = 1.0;
        let h = Hologram::from_grid(64, 32, &grid, -20.0, -10.0);

        // Reconstruct at correct angle (-20, -10)
        let recon = h.reconstruct(-20, -10);

        // Check if we have peaks
        let max_val = recon.iter().cloned().fold(0.0_f64, f64::max);
        assert!(max_val > 0.0, "Max value too low: {}", max_val);
    }
}
