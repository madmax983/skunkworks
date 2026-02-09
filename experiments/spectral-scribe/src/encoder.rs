use rustfft::{FftPlanner, num_complex::Complex};
use rand::prelude::*;
use crate::font;
use std::f32::consts::PI;

pub struct EncoderConfig {
    pub sample_rate: u32,
    pub fft_size: usize,
    pub bin_per_pixel: usize,
    pub base_bin: usize,
    pub spacing: usize,
    pub stretch_factor: usize,
}

impl Default for EncoderConfig {
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

pub fn generate_audio(text: &str, config: &EncoderConfig) -> Vec<f32> {
    let grid = font::render_text(text); // 8 rows, W cols
    if grid.is_empty() {
        return Vec::new();
    }
    let width = grid[0].len();

    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_inverse(config.fft_size);

    let mut samples = Vec::new();
    let mut rng = thread_rng();

    // Process each column of the bitmap
    for col in 0..width {
        // Repeat for stretch_factor
        for _ in 0..config.stretch_factor {
            let mut spectrum = vec![Complex::new(0.0, 0.0); config.fft_size];

            // Fill spectrum
            for row in 0..8 {
                if grid[row][col] == 1 {
                    // Row 0 (Top) -> High Freq
                    // Row 7 (Bottom) -> Low Freq
                    // visual_y = 7 - row
                    let visual_y = 7 - row;
                    let start_bin = config.base_bin + visual_y * config.spacing;

                    for k in 0..config.bin_per_pixel {
                        let bin = start_bin + k;
                        if bin < config.fft_size / 2 {
                            let mag = 50.0; // Amplitude
                            let phase = rng.gen::<f32>() * 2.0 * PI;
                            let c = Complex::from_polar(mag, phase);

                            spectrum[bin] = c;
                            // Conjugate symmetry
                            if bin > 0 {
                                spectrum[config.fft_size - bin] = c.conj();
                            }
                        }
                    }
                }
            }

            // IFFT
            fft.process(&mut spectrum);

            // Normalize and append
            let scale = 1.0 / (config.fft_size as f32);
            for c in spectrum {
                samples.push(c.re * scale);
            }
        }
    }

    samples
}

pub fn save_wav(path: &str, samples: &[f32], sample_rate: u32) -> anyhow::Result<()> {
    let spec = hound::WavSpec {
        channels: 1,
        sample_rate,
        bits_per_sample: 16,
        sample_format: hound::SampleFormat::Int,
    };
    let mut writer = hound::WavWriter::create(path, spec)?;

    for &s in samples {
        // Soft clipping
        let amp = (s * i16::MAX as f32).clamp(-i16::MAX as f32, i16::MAX as f32);
        writer.write_sample(amp as i16)?;
    }
    writer.finalize()?;
    Ok(())
}
