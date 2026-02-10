use rustfft::{FftPlanner, num_complex::Complex};
use rand::prelude::*;
use std::f32::consts::PI;
use std::io::Cursor;
use crate::font;

pub struct AudioConfig {
    pub sample_rate: u32,
    pub fft_size: usize,
    pub bin_per_pixel: usize,
    pub base_bin: usize,
    pub spacing: usize,
    pub stretch_factor: usize,
}

impl Default for AudioConfig {
    fn default() -> Self {
        Self {
            sample_rate: 44100,
            fft_size: 1024,
            bin_per_pixel: 6,
            base_bin: 100,
            spacing: 20,
            stretch_factor: 4,
        }
    }
}

pub fn generate_audio_from_text(text: &str, config: &AudioConfig) -> Vec<f32> {
    let grid = font::render_text(text); // 8 rows, W cols
    if grid.is_empty() {
        return Vec::new();
    }
    let width = grid[0].len(); // number of columns (width of text in pixels)

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_inverse(config.fft_size);

    let mut samples = Vec::new();
    let mut rng = thread_rng();

    // Process each column of the bitmap
    for col in 0..width {
        // Repeat for stretch_factor
        for _ in 0..config.stretch_factor {
            let mut spectrum = vec![Complex::new(0.0, 0.0); config.fft_size];

            // Fill spectrum based on which pixels are lit in this column
            for row in 0..8 {
                if grid[row][col] == 1 {
                    // Row 0 (Top) -> High Freq (visual y=7)
                    // Row 7 (Bottom) -> Low Freq (visual y=0)
                    // Wait, usually top row is 0. So row 0 is top.
                    // If we want row 0 to be high freq, we can keep it as is.
                    // Let's make row 7 (bottom) be base frequency, and row 0 (top) be higher.
                    let visual_y = 7 - row;
                    let start_bin = config.base_bin + visual_y * config.spacing;

                    for k in 0..config.bin_per_pixel {
                        let bin = start_bin + k;
                        if bin < config.fft_size / 2 {
                            let mag = 50.0; // Amplitude
                            let phase = rng.gen::<f32>() * 2.0 * PI;
                            let c = Complex::from_polar(mag, phase);

                            spectrum[bin] = c;
                            // Conjugate symmetry for real-valued output
                            if bin > 0 {
                                spectrum[config.fft_size - bin] = c.conj();
                            }
                        }
                    }
                }
            }

            // IFFT
            fft.process(&mut spectrum);

            // Normalize and append real parts
            let scale = 1.0 / (config.fft_size as f32);
            for c in spectrum {
                samples.push(c.re * scale);
            }
        }
    }

    samples
}

pub fn create_wav_buffer(samples: &[f32], sample_rate: u32) -> anyhow::Result<Vec<u8>> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };

    let mut cursor = Cursor::new(Vec::new());
    {
        let mut writer = hound::WavWriter::new(&mut cursor, spec)?;
        for &s in samples {
            // Soft clipping
            let amp = (s * i16::MAX as f32).clamp(-i16::MAX as f32, i16::MAX as f32);
            writer.write_sample(amp as i16)?;
        }
        writer.finalize()?;
    }

    Ok(cursor.into_inner())
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_generate_audio() {
        let config = AudioConfig::default();
        let samples = generate_audio_from_text("TEST", &config);
        assert!(!samples.is_empty());
        // 4 chars * 8 columns * stretch_factor 4 * fft_size 1024 / fft_size scale?
        // Wait, stretch_factor loops. For each col, for each stretch, we push samples.
        // Each loop pushes `fft_size` samples.
        // Width = 4 * 8 = 32.
        // Total samples = 32 * 4 * 1024 = 131072.
        assert_eq!(samples.len(), 32 * 4 * 1024);
    }
}
