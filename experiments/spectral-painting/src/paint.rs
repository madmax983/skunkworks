use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Pixel {
    pub r: u8,
    pub g: u8,
    pub b: u8,
    pub wetness: f32,
}

impl Default for Pixel {
    fn default() -> Self {
        Self {
            r: 0,
            g: 0,
            b: 0,
            wetness: 0.0,
        }
    }
}

pub struct Canvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<Pixel>,
}

impl Canvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![Pixel::default(); width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Pixel {
        if x >= self.width || y >= self.height {
            return Pixel::default();
        }
        self.pixels[y * self.width + x]
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Pixel> {
        if x >= self.width || y >= self.height {
            return None;
        }
        Some(&mut self.pixels[y * self.width + x])
    }

    /// Adds paint at a specific row (y) with a specific color intensity.
    /// x position is randomized to simulate a spray or broad brush, or can be specific.
    /// For the "Spectral Painting" effect, we probably want to spray across the row or at random x.
    pub fn apply_brush(&mut self, y: usize, intensity: f32, color: (u8, u8, u8)) {
        if y >= self.height {
            return;
        }

        let mut rng = rand::thread_rng();
        // Brush width relative to intensity
        let brush_count = (intensity * 20.0) as usize;

        for _ in 0..brush_count {
            let x = rng.gen_range(0..self.width);
            if let Some(pixel) = self.get_mut(x, y) {
                // Blend new color with existing
                let alpha = intensity.clamp(0.0, 1.0);
                pixel.r = (pixel.r as f32 * (1.0 - alpha) + color.0 as f32 * alpha) as u8;
                pixel.g = (pixel.g as f32 * (1.0 - alpha) + color.1 as f32 * alpha) as u8;
                pixel.b = (pixel.b as f32 * (1.0 - alpha) + color.2 as f32 * alpha) as u8;
                pixel.wetness = (pixel.wetness + alpha).min(1.0);
            }
        }
    }

    pub fn tick_physics(&mut self) {
        let drying_rate = 0.02;
        let gravity_factor = 0.3;
        let _diffusion_factor = 0.1;

        // Create a copy to read from while writing to self (double buffering would be better but this is simple)
        // Actually, for "melting", in-place modification top-to-bottom or bottom-to-top matters.
        // Top-to-bottom: Drips move fast (cascading).
        // Bottom-to-top: Drips move one pixel per tick.
        // Let's do Bottom-to-top to avoid instant waterfall.

        for y in (0..self.height - 1).rev() {
            for x in 0..self.width {
                let pixel = self.pixels[y * self.width + x];

                if pixel.wetness > 0.0 {
                    // Gravity: Blend with pixel below
                    let drip_amount = pixel.wetness * gravity_factor;
                    let idx_below = (y + 1) * self.width + x;

                    let r_src = pixel.r as f32;
                    let g_src = pixel.g as f32;
                    let b_src = pixel.b as f32;

                    // Modify pixel below
                    let pixel_below = &mut self.pixels[idx_below];
                    pixel_below.r = (pixel_below.r as f32 * (1.0 - drip_amount) + r_src * drip_amount) as u8;
                    pixel_below.g = (pixel_below.g as f32 * (1.0 - drip_amount) + g_src * drip_amount) as u8;
                    pixel_below.b = (pixel_below.b as f32 * (1.0 - drip_amount) + b_src * drip_amount) as u8;
                    pixel_below.wetness = (pixel_below.wetness + drip_amount).min(1.0);

                    // Diffusion: Blend with neighbors (left/right)
                    // ... (Simplification: only gravity for now to save perf)
                }
            }
        }

        // Drying pass
        for pixel in &mut self.pixels {
            if pixel.wetness > 0.0 {
                pixel.wetness = (pixel.wetness - drying_rate).max(0.0);
            }
        }
    }
}
