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

    pub fn from_boids(width: usize, height: usize, boids: &[(f64, f64)]) -> Self {
        let mut grid = vec![0.0; width * height];

        // Populate density grid from boid positions
        // Give each boid a slight "glow" / density spread
        for &(bx, by) in boids {
            let cx = bx.round() as isize;
            let cy = by.round() as isize;

            for dy in -1..=1 {
                for dx in -1..=1 {
                    let px = cx + dx;
                    let py = cy + dy;
                    if px >= 0 && px < width as isize && py >= 0 && py < height as isize {
                        // Center is brighter
                        let w = if dx == 0 && dy == 0 { 1.0 } else { 0.5 };
                        grid[(py as usize) * width + (px as usize)] += w;
                    }
                }
            }
        }

        // Apply Reference Beam (Recording Angle)
        // We simulate recording with a reference beam at a specific angle.
        // This modulates the object wave in the spatial domain.
        // When we take the FFT, this shifts the spectrum away from DC.
        let rec_angle_x = 20.0 * (2.0 * PI / width as f64); // Shift by 20 bins
        let rec_angle_y = 10.0 * (2.0 * PI / height as f64); // Shift by 10 bins

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
