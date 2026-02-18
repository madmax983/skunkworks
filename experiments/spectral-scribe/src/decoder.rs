use font8x8::{UnicodeFonts, BASIC_FONTS};
use rustfft::{num_complex::Complex, FftPlanner};

pub struct DecoderConfig {
    pub fft_size: usize,
    pub bin_per_pixel: usize,
    pub base_bin: usize,
    pub spacing: usize,
    pub stretch_factor: usize,
    pub threshold: f32,
}

impl Default for DecoderConfig {
    fn default() -> Self {
        Self {
            fft_size: 1024,
            bin_per_pixel: 6,
            base_bin: 100,
            spacing: 20,
            stretch_factor: 4,
            threshold: 25.0, // Expected magnitude ~50.0
        }
    }
}

#[allow(dead_code)]
pub fn load_wav(path: &str) -> anyhow::Result<Vec<f32>> {
    let mut reader = hound::WavReader::open(path)?;
    // Check spec? Assume matches for now.
    let samples: Vec<f32> = reader
        .samples::<i16>()
        .map(|s| s.map(|x| x as f32 / i16::MAX as f32))
        .collect::<Result<_, _>>()?;
    Ok(samples)
}

pub fn audio_to_spectrogram(samples: &[f32], config: &DecoderConfig) -> Vec<Vec<f32>> {
    let mut planner = FftPlanner::new();
    let fft = planner.plan_fft_forward(config.fft_size);

    let mut spectrogram = Vec::new();

    for chunk in samples.chunks(config.fft_size) {
        if chunk.len() < config.fft_size {
            break;
        }

        let mut buffer: Vec<Complex<f32>> = chunk.iter().map(|&s| Complex::new(s, 0.0)).collect();

        fft.process(&mut buffer);

        // Compute magnitude of first half
        let magnitude: Vec<f32> = buffer
            .iter()
            .take(config.fft_size / 2)
            .map(|c| c.norm())
            .collect();

        spectrogram.push(magnitude);
    }
    spectrogram
}

pub fn recover_text(spectrogram: &[Vec<f32>], config: &DecoderConfig) -> String {
    let num_frames = spectrogram.len();
    if num_frames == 0 {
        return String::new();
    }

    let width = num_frames / config.stretch_factor;
    if width == 0 {
        return String::new();
    }

    // Recover grid
    let mut recovered_grid = vec![vec![0u8; width]; 8];

    for col in 0..width {
        let start_frame = col * config.stretch_factor;
        let end_frame = start_frame + config.stretch_factor;

        for row in 0..8 {
            let visual_y = 7 - row;
            let start_bin = config.base_bin + visual_y * config.spacing;

            let mut sum_mag = 0.0;
            let mut count = 0;

            for frame_idx in start_frame..end_frame {
                if frame_idx >= spectrogram.len() {
                    break;
                }
                let frame = &spectrogram[frame_idx];

                for k in 0..config.bin_per_pixel {
                    let bin = start_bin + k;
                    if bin < frame.len() {
                        sum_mag += frame[bin];
                        count += 1;
                    }
                }
            }

            let avg_mag = if count > 0 {
                sum_mag / count as f32
            } else {
                0.0
            };

            // Threshold
            // avg_mag is average per BIN.
            // If encoder put 50.0 per bin, we expect ~50.0 here.
            if avg_mag > config.threshold {
                recovered_grid[row][col] = 1;
            }
        }
    }

    // Grid to Text
    let num_chars = width / 8;
    let mut result = String::new();

    // Extended charset for searching
    let candidates = " ABCDEFGHIJKLMNOPQRSTUVWXYZabcdefghijklmnopqrstuvwxyz0123456789.,!?:;\"'()[]{}<>-+=_@#$%^&*|/\\`~\n";

    for i in 0..num_chars {
        let mut glyph = [0u8; 8];
        for row in 0..8 {
            let mut byte = 0u8;
            for col_rel in 0..8 {
                let col_abs = i * 8 + col_rel;
                if recovered_grid[row][col_abs] == 1 {
                    let bit = 7 - col_rel;
                    byte |= 1 << bit;
                }
            }
            glyph[row] = byte;
        }

        let mut found = false;
        for c in candidates.chars() {
            if let Some(g) = BASIC_FONTS.get(c) {
                if g == glyph {
                    result.push(c);
                    found = true;
                    break;
                }
            }
        }

        if !found {
            // Check for empty glyph (space) if not covered
            if glyph == [0; 8] {
                result.push(' ');
            } else {
                result.push('?');
            }
        }
    }

    result
}
