use crate::glyph::Glyph;
use ::rand::Rng;
use image::{ImageBuffer, Rgb};
use macroquad::prelude::*;

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
        // Ensure at least 1 row
        let len = payload.len().max(1) as u32;
        let height_glyphs = len.div_ceil(width_glyphs);
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
        let mut rng = ::rand::thread_rng();

        // Fill with "Void" (deep space)
        for pixel in img.pixels_mut() {
            *pixel = Rgb([5, 5, 15]); // Very dark blue
        }

        // Add Salt Noise (background stars)
        let num_stars = (self.width * self.height) / 50;
        for _ in 0..num_stars {
            let x = rng.gen_range(0..self.width);
            let y = rng.gen_range(0..self.height);
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

            glyph.render(x, y, &mut img);
        }

        img
    }

    pub fn generate_texture(&self) -> Texture2D {
        let buffer = self.generate();
        let width = buffer.width() as u16;
        let height = buffer.height() as u16;

        let mut mq_image = Image {
            width,
            height,
            bytes: vec![0; width as usize * height as usize * 4],
        };

        for (x, y, pixel) in buffer.enumerate_pixels() {
            let [r, g, b] = pixel.0;
            mq_image.set_pixel(x, y, Color::from_rgba(r, g, b, 255));
        }

        Texture2D::from_image(&mq_image)
    }
}
