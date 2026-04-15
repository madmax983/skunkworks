//! Audio encoding engine.
//!
//! Provides tools to convert string text into a spectrogram footprint, which is
//! then synthesized into raw audio samples via Inverse Fast Fourier Transform (IFFT).

use crate::font;
use rand::prelude::*;
use rustfft::{num_complex::Complex, FftPlanner};
use std::f32::consts::PI;

/// Configuration parameters for embedding text into the audio spectrum.
///
/// Controls the resolution, frequency band placement, and duration of the encoded signal.
///
/// # Examples
///
/// ```
/// use spectral_scribe::encoder::EncoderConfig;
///
/// let config = EncoderConfig::default();
/// assert_eq!(config.sample_rate, 44100);
/// assert_eq!(config.fft_size, 1024);
/// ```
pub struct EncoderConfig {
    /// Audio sample rate in Hz (e.g. 44100).
    pub sample_rate: u32,
    /// Number of bins for the FFT. Higher sizes offer higher frequency resolution.
    pub fft_size: usize,
    /// Number of adjacent frequency bins used to represent a single pixel in height.
    pub bin_per_pixel: usize,
    /// The starting bin index for the lowest frequency of the text rendering.
    pub base_bin: usize,
    /// Frequency bin distance between vertical pixel rows.
    pub spacing: usize,
    /// Number of audio frames a single horizontal pixel column is repeated (duration).
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

/// Synthesizes raw audio samples from an input string.
///
/// The string is converted into an 8-bit height bitmap and written into
/// the frequency spectrum via IFFT according to the `EncoderConfig`.
///
/// # Examples
///
/// ```
/// use spectral_scribe::encoder::{EncoderConfig, generate_audio};
///
/// let config = EncoderConfig::default();
/// let samples = generate_audio("Hello", &config);
/// assert!(!samples.is_empty());
/// ```
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
            for (row_idx, row_vec) in grid.iter().enumerate().take(8) {
                if row_vec[col] == 1 {
                    // Row 0 (Top) -> High Freq
                    // Row 7 (Bottom) -> Low Freq
                    // visual_y = 7 - row_idx
                    let visual_y = 7 - row_idx;
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

/// Saves raw audio samples as a single-channel, 16-bit WAV file on disk.
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
