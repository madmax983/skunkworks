use num_complex::Complex;
use rustfft::FftPlanner;
use std::f64::consts::PI;

pub struct SpectralField {
    pub width: usize,
    pub height: usize,
    pub spectrum: Vec<Complex<f64>>,
    pub potential: Vec<f64>,
}

impl SpectralField {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            spectrum: vec![Complex::new(0.0, 0.0); width * height],
            potential: vec![0.0; width * height],
        }
    }

    /// Compute FFT of the spatial data (e.g., magnetism density)
    pub fn compute_spectrum(&mut self, spatial_data: &[f64]) {
        let size = self.width * self.height;
        if spatial_data.len() != size {
            return;
        }

        // 1. Prepare Complex Input
        let mut buffer: Vec<Complex<f64>> = spatial_data
            .iter()
            .map(|&val| Complex::new(val, 0.0))
            .collect();

        // 2. 2D FFT
        let mut planner = FftPlanner::new();
        let fft_x = planner.plan_fft_forward(self.width);
        let fft_y = planner.plan_fft_forward(self.height);

        // Rows
        for y in 0..self.height {
            let start = y * self.width;
            let end = start + self.width;
            fft_x.process(&mut buffer[start..end]);
        }

        transpose(&mut buffer, self.width, self.height);

        // Columns
        for x in 0..self.width {
            let start = x * self.height;
            let end = start + self.height;
            fft_y.process(&mut buffer[start..end]);
        }

        transpose(&mut buffer, self.height, self.width);

        self.spectrum = buffer;
    }

    /// Filter spectrum and compute Inverse FFT to get "Spectral Potential"
    pub fn compute_potential(&mut self) {
        let mut buffer = self.spectrum.clone();

        // 1. Filter: Attenuate high frequencies (Low-pass)
        // High frequencies are in the middle of the buffer due to standard FFT layout?
        // Actually, standard FFT layout has DC at 0, Nyquist at N/2.
        // So high frequencies are around width/2, height/2.

        let cx = self.width as f64 / 2.0;
        let cy = self.height as f64 / 2.0;

        for y in 0..self.height {
            for x in 0..self.width {
                // Calculate normalized frequency distance from DC (0,0)
                // We handle wrapping: 0 is DC, width-1 is small negative freq (close to DC).
                // But for simple "distance from DC", we can use min(i, N-i).

                let dx = x.min(self.width - x) as f64;
                let dy = y.min(self.height - y) as f64;
                let dist = (dx * dx + dy * dy).sqrt();

                // "Spectral Resonance": Boost mid-frequencies, suppress high/low?
                // Or just standard low-pass for a smooth potential.
                // Let's do a "Band-pass" to create interesting ripples.
                // Peaks around 10.0 distance.

                let factor = (-((dist - 10.0).powi(2) / 50.0)).exp(); // Gaussian centered at 10

                let idx = y * self.width + x;
                buffer[idx] *= factor * 5.0; // Amplify for force strength
            }
        }

        // 2. Inverse 2D FFT
        let mut planner = FftPlanner::new();
        let ifft_x = planner.plan_fft_inverse(self.width);
        let ifft_y = planner.plan_fft_inverse(self.height);

        // Rows
        for y in 0..self.height {
            let start = y * self.width;
            let end = start + self.width;
            ifft_x.process(&mut buffer[start..end]);
        }

        transpose(&mut buffer, self.width, self.height);

        // Columns
        for x in 0..self.width {
            let start = x * self.height;
            let end = start + self.height;
            ifft_y.process(&mut buffer[start..end]);
        }

        transpose(&mut buffer, self.height, self.width);

        // 3. Store Real Part (Normalized)
        let scale = 1.0 / (self.width * self.height) as f64;
        self.potential = buffer.iter().map(|c| c.re * scale).collect();
    }

    /// Get gradient of the potential field at (x, y)
    pub fn get_gradient(&self, x: f64, y: f64) -> (f64, f64) {
        let gx = x.round() as usize;
        let gy = y.round() as usize;

        if gx > 0 && gx < self.width - 1 && gy > 0 && gy < self.height - 1 {
            let idx = gy * self.width + gx;

            // Central Difference
            let right = self.potential[idx + 1];
            let left = self.potential[idx - 1];
            let up = self.potential[idx + self.width];
            let down = self.potential[idx - self.width];

            ((right - left) * 0.5, (up - down) * 0.5)
        } else {
            (0.0, 0.0)
        }
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
