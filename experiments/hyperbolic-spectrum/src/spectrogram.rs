use rustfft::{FftPlanner, num_complex::Complex};

pub struct Spectrogram {
    pub fft_size: usize,
    planner: FftPlanner<f32>,
}

impl Spectrogram {
    pub fn new(fft_size: usize) -> Self {
        Self {
            fft_size,
            planner: FftPlanner::new(),
        }
    }

    pub fn analyze_chunk(&mut self, samples: &[f32]) -> Vec<f32> {
        let fft = self.planner.plan_fft_forward(self.fft_size);

        let mut input: Vec<Complex<f32>> = samples.iter()
            .take(self.fft_size)
            .map(|&x| Complex::new(x, 0.0))
            .collect();

        // Zero padding if not enough samples
        if input.len() < self.fft_size {
            input.resize(self.fft_size, Complex::new(0.0, 0.0));
        }

        // Window function (Hanning)
        for (i, val) in input.iter_mut().enumerate() {
            let window = 0.5 * (1.0 - (2.0 * std::f32::consts::PI * i as f32 / (self.fft_size - 1) as f32).cos());
            *val = *val * window;
        }

        fft.process(&mut input);

        // Return magnitude of first half (Nyquist limit)
        input.iter()
            .take(self.fft_size / 2)
            .map(|c| c.norm())
            .collect()
    }
}
