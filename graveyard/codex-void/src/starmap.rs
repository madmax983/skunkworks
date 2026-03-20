use crate::glyph::Glyph;
use image::{ImageBuffer, Rgb};
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

    pub fn generate(&self) -> ImageBuffer<Rgb<u8>, Vec<u8>> {
        let mut img = ImageBuffer::new(self.width, self.height);
        let mut rng = rand::thread_rng();

        // Fill with "Void" (deep space)
        for pixel in img.pixels_mut() {
            *pixel = Rgb([5, 5, 15]); // Very dark blue
        }

        // Add Salt Noise (background stars)
        // Ensure background stars are distinct from data stars (dimmer or single pixel)
        let num_stars = (self.width * self.height) / 50;
        for _ in 0..num_stars {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
            // Random dim brightness
            let b = rng.gen_range(50..150);
            img.put_pixel(x, y, Rgb([b, b, b + 20]));
        }

        // Render Glyphs
        let cols = self.width / self.spacing;

        for (i, glyph) in self.glyphs.iter().enumerate() {
            let row = (i as u32) / cols;
            let col = (i as u32) % cols;

            let x = col * self.spacing + (self.spacing / 2);
            let y = row * self.spacing + (self.spacing / 2);

            // We use the imported render logic
            // But wait, render function is part of Glyph impl in glyph.rs
            // I need to import it.
            // Wait, Glyph::render takes &mut img.
            // But I cannot borrow img mutably in a loop if something else borrows it?
            // Here it's fine.

            // However, Glyph::render puts pixels.
            // I need to make sure Glyph::render uses `put_pixel` on the img.
            // The method signature in `glyph.rs` was:
            // pub fn render(&self, center_x: u32, center_y: u32, img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>)

            glyph.render(x, y, &mut img);
        }

        img
    }
}
