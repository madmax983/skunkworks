use num_complex::Complex;
use rustfft::FftPlanner;

pub struct Hologram {
    pub data: Vec<Complex<f64>>,
}

impl Hologram {
    pub fn from_grid(grid: &[f64], width: usize, height: usize) -> Self {
        let mut data: Vec<Complex<f64>> = grid.iter().map(|&val| Complex::new(val, 0.0)).collect();

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

        // Shift zero-frequency component to center
        let mut shifted_data = vec![Complex::new(0.0, 0.0); width * height];
        let half_width = width / 2;
        let half_height = height / 2;
        for y in 0..height {
            for x in 0..width {
                let shifted_x = (x + half_width) % width;
                let shifted_y = (y + half_height) % height;
                shifted_data[shifted_y * width + shifted_x] = data[y * width + x];
            }
        }

        Self { data: shifted_data }
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
            temp[x * height + y] = data[y * width + x];
        }
    }
    data.copy_from_slice(&temp);
}
