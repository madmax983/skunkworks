use font8x8::{UnicodeFonts, BASIC_FONTS};
use num_complex::Complex;
use rustfft::FftPlanner;
use std::f64::consts::PI;

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

    /// Creates a hologram from text, recorded at a specific angle (frequency shift).
    /// rec_angle_x/y are in radians per pixel (spatial frequency).
    pub fn from_text(text: &str, width: usize, height: usize, rec_angle_x: f64, rec_angle_y: f64) -> Self {
        let mut grid = vec![0.0; width * height];

        let mut x_off = 0;
        let mut y_off = 0;

        for c in text.chars() {
            if let Some(glyph) = BASIC_FONTS.get(c) {
                for (y, &row) in glyph.iter().enumerate() {
                    for x in 0..8 {
                        if ((row >> x) & 1) == 1 {
                            let gx = x_off + x;
                            let gy = y_off + y;
                            if gx < width && gy < height {
                                grid[gy * width + gx] = 1.0;
                            }
                        }
                    }
                }
                x_off += 8;
                if x_off >= width {
                    x_off = 0;
                    y_off += 8;
                }
            }
        }

        // Apply Reference Beam (Recording Angle)
        let mut data: Vec<Complex<f64>> = grid
            .iter()
            .enumerate()
            .map(|(i, &val)| {
                let x = (i % width) as f64;
                let y = (i / width) as f64;
                // Modulation: exp(i * (kx*x + ky*y))
                let phase = rec_angle_x * x + rec_angle_y * y;
                let r = Complex::new(phase.cos(), phase.sin());
                Complex::new(val, 0.0) * r
            })
            .collect();

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

        Self {
            width,
            height,
            data,
        }
    }

    pub fn add_layer(&mut self, other: &Hologram) {
        if self.width != other.width || self.height != other.height {
            panic!("Hologram size mismatch");
        }
        for (i, val) in other.data.iter().enumerate() {
            self.data[i] = self.data[i] + val;
        }
    }

    /// Reconstructs the image at a specific viewing angle (shift in frequency bins).
    /// Returns magnitude data.
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

        // Inverse 2D FFT
        let mut planner = FftPlanner::new();
        let fft_x = planner.plan_fft_inverse(width);
        let fft_y = planner.plan_fft_inverse(height);

        let mut data = shifted_data;

        // IFFT Rows
        for y in 0..height {
            let start = y * width;
            let end = start + width;
            fft_x.process(&mut data[start..end]);
        }

        transpose(&mut data, width, height);

        // IFFT Columns
        for x in 0..width {
            let start = x * height;
            let end = start + height;
            fft_y.process(&mut data[start..end]);
        }

        transpose(&mut data, height, width);

        // Return magnitude normalized
        let scale = 1.0 / (width * height) as f64;
        data.iter().map(|c| c.norm() * scale).collect()
    }

    /// Reconstructs a small window around (cx, cy) with size (w, h).
    /// This is an approximation: We do full reconstruction then crop.
    /// Optimization: For a true local reconstruction from a hologram, we'd need
    /// to do a full IFFT anyway because holographic data is non-local.
    /// So full reconstruction is necessary unless we use optical tricks.
    /// For simulation performance, we might cache the reconstruction if the angle doesn't change often.
    /// But organisms change angles independently.
    ///
    /// Wait, full IFFT for every organism is too slow (O(N log N)).
    /// If N=256*256=65k, N log N ~ 1M ops. 100 organisms = 100M ops per frame.
    /// That might be slow in debug mode but okay in release.
    ///
    /// Alternatively, we can assume the "World" has a fixed set of "Channels" (Angles).
    /// And we reconstruct those channels once per frame.
    /// Organisms just tune into one of the pre-computed channels.
    /// This is much faster.
    ///
    /// Let's say we have 4 channels (Food, Mate, Danger, Shelter).
    /// Organisms choose which channel to view.
    pub fn reconstruct_channel(&self, channel_idx: usize, _total_channels: usize) -> Vec<f64> {
         let (sx, sy) = Self::get_channel_shift(channel_idx);
         self.reconstruct(sx, sy)
    }

    pub fn get_channel_shift(channel_idx: usize) -> (isize, isize) {
         let shifts = [
             (0, 0),
             (20, 10),
             (-20, -10),
             (10, -20),
             (-10, 20),
         ];
         shifts[channel_idx % shifts.len()]
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
