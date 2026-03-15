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

    pub fn from_density(width: usize, height: usize, grid: &[f64]) -> Self {
        let mut planner = FftPlanner::new();
        let fft = planner.plan_fft_forward(width * height);

        let mut complex_data = vec![Complex::new(0.0, 0.0); width * height];

        let angle_x = 20.0;
        let angle_y = 10.0;
        let kx = 2.0 * PI * angle_x / width as f64;
        let ky = 2.0 * PI * angle_y / height as f64;

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let phase = kx * x as f64 + ky * y as f64;
                let reference = Complex::new(phase.cos(), phase.sin());
                let object = Complex::new(grid[idx], 0.0);

                complex_data[idx] = object * reference;
            }
        }

        fft.process(&mut complex_data);

        Self {
            width,
            height,
            data: complex_data,
        }
    }

    pub fn reconstruct(&self, angle_x: isize, angle_y: isize) -> Vec<f64> {
        let mut planner = FftPlanner::new();
        let ifft = planner.plan_fft_inverse(self.width * self.height);
        let mut complex_data = self.data.clone();

        ifft.process(&mut complex_data);

        let kx = 2.0 * PI * angle_x as f64 / self.width as f64;
        let ky = 2.0 * PI * angle_y as f64 / self.height as f64;

        let mut reconstructed_mag = vec![0.0; self.width * self.height];
        let scale = 1.0 / (self.width * self.height) as f64;

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let phase = kx * x as f64 + ky * y as f64;
                let reference_conj = Complex::new(phase.cos(), -phase.sin());

                let val = complex_data[idx] * scale * reference_conj;
                reconstructed_mag[idx] = val.norm();
            }
        }
        reconstructed_mag
    }

    pub fn get_magnitude(&self) -> Vec<f64> {
        self.data.iter().map(|c| (c.norm() + 1.0).ln()).collect()
    }
}
