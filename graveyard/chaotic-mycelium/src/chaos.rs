use macroquad::prelude::*;
use rayon::prelude::*;

pub struct ChaosSubstrate {
    pub width: usize,
    pub height: usize,
    pub lyapunov: Vec<f32>, // Store Lyapunov exponents
    pub texture: Texture2D,
    pub image_data: Vec<u8>,
}

impl ChaosSubstrate {
    pub fn new(width: usize, height: usize) -> Self {
        let lyapunov = vec![0.0; width * height];
        let image_data = vec![0u8; width * height * 4];
        let texture = Texture2D::from_image(&Image {
            width: width as u16,
            height: height as u16,
            bytes: image_data.clone(),
        });

        Self {
            width,
            height,
            lyapunov,
            texture,
            image_data,
        }
    }

    pub fn generate(&mut self, seq: &str, a_range: (f32, f32), b_range: (f32, f32)) {
        let width = self.width;
        let height = self.height;
        let seq_bytes = seq.as_bytes();
        let seq_len = seq_bytes.len();

        // Compute rows in parallel
        let rows: Vec<(usize, Vec<f32>, Vec<u8>)> = (0..height)
            .into_par_iter()
            .map(|y| {
                let mut row_lyapunov = vec![0.0; width];
                let mut row_pixels = vec![0u8; width * 4];

                // Map y to b parameter
                let b = b_range.0 + (b_range.1 - b_range.0) * (y as f32 / height as f32);

                for x in 0..width {
                    // Map x to a parameter
                    let a = a_range.0 + (a_range.1 - a_range.0) * (x as f32 / width as f32);

                    let mut x_val = 0.5;
                    let mut sum_log_deriv = 0.0;
                    let warmup = 100;
                    let steps = 400; // Increase for better quality

                    // Warmup
                    for i in 0..warmup {
                        let r = if seq_bytes[i % seq_len] == b'A' { a } else { b };
                        x_val = r * x_val * (1.0 - x_val);
                    }

                    // Calculate Lyapunov
                    for i in 0..steps {
                        let r = if seq_bytes[i % seq_len] == b'A' { a } else { b };
                        x_val = r * x_val * (1.0 - x_val);

                        let deriv = (r * (1.0 - 2.0 * x_val)).abs();
                        if deriv > 1e-9 {
                            sum_log_deriv += deriv.ln();
                        } else {
                            sum_log_deriv += -10.0; // Clamp low value
                        }
                    }

                    let lambda = sum_log_deriv / steps as f32;
                    row_lyapunov[x] = lambda;

                    // Color mapping
                    // Negative lambda (Stable) -> Blue/Cyan/Black
                    // Positive lambda (Chaotic) -> Yellow/Red/White
                    let color = if lambda < 0.0 {
                        // Stable: Shades of Blue/Deep Purple
                        let intensity = (-lambda * 0.5).min(1.0);
                        Color::new(0.0, 0.1 * intensity, 0.2 + 0.4 * intensity, 1.0)
                    } else {
                        // Chaotic: Shades of Orange/Red
                        let intensity = (lambda * 0.5).min(1.0);
                        Color::new(0.2 + 0.8 * intensity, 0.1 * intensity, 0.0, 1.0)
                    };

                    row_pixels[x * 4 + 0] = (color.r * 255.0) as u8;
                    row_pixels[x * 4 + 1] = (color.g * 255.0) as u8;
                    row_pixels[x * 4 + 2] = (color.b * 255.0) as u8;
                    row_pixels[x * 4 + 3] = 255;
                }

                (y, row_lyapunov, row_pixels)
            })
            .collect();

        // Write back to main struct buffers
        for (y, row_lyapunov, row_pixels) in rows {
            let start_idx = y * width;
            let end_idx = start_idx + width;
            self.lyapunov[start_idx..end_idx].copy_from_slice(&row_lyapunov);

            let pixel_start = y * width * 4;
            let pixel_end = pixel_start + width * 4;
            self.image_data[pixel_start..pixel_end].copy_from_slice(&row_pixels);
        }

        self.texture.update(&Image {
            width: width as u16,
            height: height as u16,
            bytes: self.image_data.clone(),
        });
    }

    pub fn get_cost(&self, x: i32, y: i32) -> f32 {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return f32::INFINITY;
        }
        let idx = (y as usize) * self.width + (x as usize);
        let lambda = self.lyapunov[idx];

        // Cost function:
        // Fungus prefers stability (negative lambda).
        // Map lambda range [-2.0, 1.0] to cost [1.0, 100.0]

        if lambda < 0.0 {
            // Stable region: Low cost
            // lambda -2.0 -> cost 1.0
            // lambda 0.0 -> cost 5.0
            1.0 + (lambda + 2.0).max(0.0) * 2.0
        } else {
            // Chaotic region: High cost
            // lambda 0.0 -> cost 5.0
            // lambda 1.0 -> cost 50.0
            5.0 + lambda * 45.0
        }
    }

    pub fn draw(&self) {
        draw_texture_ex(
            &self.texture,
            0.0,
            0.0,
            WHITE,
            DrawTextureParams {
                dest_size: Some(vec2(screen_width(), screen_height())),
                ..Default::default()
            },
        );
    }
}
