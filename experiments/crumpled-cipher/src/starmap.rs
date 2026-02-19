use crate::glyph::Glyph;
use image::{ImageBuffer, Rgba};
use rand::Rng;

pub const SPACING: u32 = 12;

pub struct StarMap {
    glyphs: Vec<Glyph>,
    pub width: u32,
    pub height: u32,
    pub spacing: u32,
}

impl StarMap {
    pub fn new(payload: &[u8], width_glyphs: u32) -> Self {
        let glyphs: Vec<Glyph> = payload.iter().map(|&b| Glyph::new(b)).collect();
        // Calculate height
        let height_glyphs = (payload.len() as u32 + width_glyphs - 1) / width_glyphs;
        let spacing = SPACING;

        Self {
            glyphs,
            width: width_glyphs * spacing,
            height: height_glyphs * spacing,
            spacing,
        }
    }

    pub fn generate(&self) -> ImageBuffer<Rgba<u8>, Vec<u8>> {
        let mut img = ImageBuffer::new(self.width, self.height);
        let mut rng = rand::thread_rng();

        // Fill with "Void" (deep space) - Black/Dark Blue
        for pixel in img.pixels_mut() {
            *pixel = Rgba([5, 5, 15, 255]);
        }

        // Add Salt Noise (background stars)
        let num_stars = (self.width * self.height) / 50;
        for _ in 0..num_stars {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            // Random dim brightness
            let b = rng.gen_range(50..150);
            img.put_pixel(x, y, Rgba([b, b, b + 20, 255]));
        }

        // Render Glyphs
        let cols = self.width / self.spacing;

        for (i, glyph) in self.glyphs.iter().enumerate() {
            let row = (i as u32) / cols;
            let col = (i as u32) % cols;

            let x = col * self.spacing + (self.spacing / 2);
            let y = row * self.spacing + (self.spacing / 2);

            glyph.render(x, y, &mut img);
        }

        img
    }
}
