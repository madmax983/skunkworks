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

    pub fn from_grid(grid: &[f64], width: usize, height: usize) -> Self {
        let rec_angle_x = 20.0 * (2.0 * PI / width as f64);
        let rec_angle_y = 10.0 * (2.0 * PI / height as f64);

        let mut data: Vec<Complex<f64>> = grid
            .iter()
            .enumerate()
            .map(|(i, &val)| {
                let x = (i % width) as f64;
                let y = (i / width) as f64;
                let phase = rec_angle_x * x + rec_angle_y * y;
                Complex::new(val * phase.cos(), val * phase.sin())
            })
            .collect();

        Self::fft2d(&mut data, width, height, rustfft::FftDirection::Forward);

        Self {
            width,
            height,
            data,
        }
    }

    pub fn get_magnitude(&self) -> Vec<f64> {
        self.data.iter().map(|c| c.norm().ln_1p()).collect()
    }

    pub fn reconstruct(&self, angle_x: isize, angle_y: isize) -> Vec<f64> {
        let mut recon = self.data.clone();

        Self::fft2d(
            &mut recon,
            self.width,
            self.height,
            rustfft::FftDirection::Inverse,
        );

        let phase_x = angle_x as f64 * (2.0 * PI / self.width as f64);
        let phase_y = angle_y as f64 * (2.0 * PI / self.height as f64);

        recon
            .iter()
            .enumerate()
            .map(|(i, c)| {
                let x = (i % self.width) as f64;
                let y = (i / self.width) as f64;
                let correction = phase_x * x + phase_y * y;
                let rot = Complex::new(correction.cos(), correction.sin());
                (c * rot).re.max(0.0) // Return real part, clamped to 0
            })
            .collect()
    }

    fn fft2d(
        data: &mut [Complex<f64>],
        width: usize,
        height: usize,
        direction: rustfft::FftDirection,
    ) {
        let mut planner = FftPlanner::new();
        let fft_row = planner.plan_fft(width, direction);
        let fft_col = planner.plan_fft(height, direction);

        // Process rows
        for y in 0..height {
            let row = &mut data[y * width..(y + 1) * width];
            fft_row.process(row);
        }

        // Transpose
        let mut transposed = vec![Complex::new(0.0, 0.0); width * height];
        for y in 0..height {
            for x in 0..width {
                transposed[x * height + y] = data[y * width + x];
            }
        }

        // Process columns
        for x in 0..width {
            let col = &mut transposed[x * height..(x + 1) * height];
            fft_col.process(col);
        }

        // Transpose back
        for x in 0..width {
            for y in 0..height {
                data[y * width + x] = transposed[x * height + y];
            }
        }

        if direction == rustfft::FftDirection::Inverse {
            let scale = 1.0 / (width * height) as f64;
            for c in data.iter_mut() {
                *c *= scale;
            }
        }
    }
}
