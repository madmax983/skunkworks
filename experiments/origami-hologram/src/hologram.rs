use num_complex::Complex;
use rustfft::FftPlanner;

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
        self.data.fill(Complex::new(0.0, 0.0));
    }

    pub fn reconstruct(&self, shift_x_bins: isize, shift_y_bins: isize) -> Vec<f64> {
        let width = self.width;
        let height = self.height;

        let mut shifted_data = vec![Complex::new(0.0, 0.0); width * height];

        for y in 0..height {
            for x in 0..width {
                let src_x = (x as isize - shift_x_bins).rem_euclid(width as isize) as usize;
                let src_y = (y as isize - shift_y_bins).rem_euclid(height as isize) as usize;
                shifted_data[y * width + x] = self.data[src_y * width + src_x];
            }
        }

        let mut planner = FftPlanner::new();
        let fft_x = planner.plan_fft_inverse(width);
        let fft_y = planner.plan_fft_inverse(height);

        let mut data = shifted_data;

        for y in 0..height {
            let start = y * width;
            let end = start + width;
            fft_x.process(&mut data[start..end]);
        }

        transpose(&mut data, width, height);

        for x in 0..width {
            let start = x * height;
            let end = start + height;
            fft_y.process(&mut data[start..end]);
        }

        transpose(&mut data, height, width);

        let scale = 1.0 / (width * height) as f64;
        data.iter().map(|c| c.norm() * scale).collect()
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
