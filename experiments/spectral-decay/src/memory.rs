use num_complex::Complex;
use rustfft::{FftPlanner, FftDirection};

#[derive(Clone)]
pub struct SpectralMemory {
    pub width: u32,
    pub height: u32,
    pub buffer: Vec<Complex<f32>>,
}

impl SpectralMemory {
    pub fn new(width: u32, height: u32) -> Self {
        let size = (width * height) as usize;
        Self {
            width,
            height,
            buffer: vec![Complex::new(0.0, 0.0); size],
        }
    }

    pub fn ingest(&mut self, data: &[u8]) {
        if data.len() != self.buffer.len() {
            panic!("Data size mismatch");
        }

        // 1. Convert to Complex
        for (i, &pixel) in data.iter().enumerate() {
            self.buffer[i] = Complex::new(pixel as f32, 0.0);
        }

        self.perform_fft(FftDirection::Forward);
    }

    pub fn recall(&self) -> Vec<u8> {
        // Clone buffer to perform inverse transform
        let mut temp_memory = self.clone();
        temp_memory.perform_fft(FftDirection::Inverse);

        let normalization = (self.width * self.height) as f32;

        temp_memory.buffer.iter().map(|c| {
            // Normalize (divide by N)
            let val = c.re / normalization;
            val.clamp(0.0, 255.0) as u8
        }).collect()
    }

    fn perform_fft(&mut self, direction: FftDirection) {
        let mut planner = FftPlanner::new();

        // 1. FFT Rows
        let width = self.width as usize;
        let fft_row = planner.plan_fft(width, direction);
        let scratch_len = fft_row.get_inplace_scratch_len();
        let mut scratch = vec![Complex::new(0.0, 0.0); scratch_len];

        for row in self.buffer.chunks_exact_mut(width) {
            fft_row.process_with_scratch(row, &mut scratch);
        }

        // 2. Transpose
        self.transpose();

        // 3. FFT Columns (now rows because transposed)
        let width = self.width as usize; // width is now old height
        let fft_col = planner.plan_fft(width, direction);
        let scratch_len = fft_col.get_inplace_scratch_len();
        let mut scratch = vec![Complex::new(0.0, 0.0); scratch_len];

        for col in self.buffer.chunks_exact_mut(width) {
            fft_col.process_with_scratch(col, &mut scratch);
        }

        // 4. Transpose back
        self.transpose();
    }

    fn transpose(&mut self) {
        let width = self.width as usize;
        let height = self.height as usize;
        let mut new_buffer = vec![Complex::new(0.0, 0.0); width * height];
        for y in 0..height {
            for x in 0..width {
                new_buffer[x * height + y] = self.buffer[y * width + x];
            }
        }
        self.buffer = new_buffer;
        // Swap dimensions
        std::mem::swap(&mut self.width, &mut self.height);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fft_roundtrip() {
        let width = 8;
        let height = 8;
        let mut memory = SpectralMemory::new(width, height);

        let mut data = Vec::with_capacity((width * height) as usize);
        for i in 0..(width * height) {
            data.push(((i * 10) % 255) as u8);
        }

        memory.ingest(&data);
        let recalled = memory.recall();

        assert_eq!(data.len(), recalled.len());

        for i in 0..data.len() {
            let diff = (data[i] as i16 - recalled[i] as i16).abs();
            // Allow off-by-one due to float rounding
            assert!(diff <= 1, "Pixel {} mismatch: expected {}, got {}", i, data[i], recalled[i]);
        }
    }
}
