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

    pub fn set_density(&mut self, data: &[f32]) {
        for (i, &v) in data.iter().enumerate() {
            if i < self.data.len() {
                self.data[i] = Complex::new(v as f64, 0.0);
            }
        }
    }

    pub fn process_fft(&mut self) {
        let width = self.width;
        let height = self.height;

        let mut planner = FftPlanner::new();
        let fft_x = planner.plan_fft_forward(width);
        let fft_y = planner.plan_fft_forward(height);

        for y in 0..height {
            let start = y * width;
            let end = start + width;
            fft_x.process(&mut self.data[start..end]);
        }

        transpose(&mut self.data, width, height);

        for x in 0..width {
            let start = x * height;
            let end = start + height;
            fft_y.process(&mut self.data[start..end]);
        }

        transpose(&mut self.data, height, width);
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

        data.iter().map(|c| c.norm()).collect()
    }
}

fn transpose<T: Copy>(data: &mut [T], width: usize, height: usize) {
    let mut temp = Vec::with_capacity(width * height);
    for x in 0..width {
        for y in 0..height {
            temp.push(data[y * width + x]);
        }
    }
    data.copy_from_slice(&temp);
}
