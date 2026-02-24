use macroquad::prelude::*;

pub struct LogisticMap {
    pub texture: Texture2D,
    pub min_r: f32,
    pub max_r: f32,
}

impl LogisticMap {
    pub fn new(width: u16, height: u16) -> Self {
        let mut image = Image::gen_image_color(width, height, BLACK);

        let min_r = 2.8;
        let max_r = 4.0;

        for px in 0..width {
            let r = min_r + (px as f32 / width as f32) * (max_r - min_r);
            let mut x = 0.5;

            // Settle
            for _ in 0..100 {
                x = r * x * (1.0 - x);
            }

            // Draw
            for _ in 0..200 {
                x = r * x * (1.0 - x);
                let py = ((1.0 - x) * (height as f32 - 1.0)) as u32;
                if py < height as u32 {
                    // Accumulate brightness
                    let existing = image.get_pixel(px as u32, py);
                    let brightness = (existing.g + 0.1).min(1.0);
                    image.set_pixel(px as u32, py, Color::new(0.0, brightness, existing.b + 0.05, 1.0));
                }
            }
        }

        let texture = Texture2D::from_image(&image);
        Self { texture, min_r, max_r }
    }

    /// Calculate distance to the nearest attractor point for a given (r, x)
    pub fn distance_to_attractor(&self, r: f32, x: f32) -> f32 {
        if r < self.min_r || r > self.max_r {
            return 1.0; // Out of bounds is chaos/void
        }

        let mut curr = 0.5;
        // Settle
        for _ in 0..100 {
            curr = r * curr * (1.0 - curr);
        }

        let mut min_dist = 1.0f32;
        // Check orbit
        for _ in 0..100 {
            curr = r * curr * (1.0 - curr);
            let dist = (curr - x).abs();
            if dist < min_dist {
                min_dist = dist;
            }
        }
        min_dist
    }
}
