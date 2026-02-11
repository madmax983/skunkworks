use ab_glyph::{FontRef, Font, PxScale, point, ScaleFont};
use image::{GrayImage, Luma};

pub struct TextGenerator {
    font: FontRef<'static>,
    pub width: u32,
    pub height: u32,
}

impl TextGenerator {
    pub fn new(width: u32, height: u32) -> Self {
        // Embed the font to avoid file IO issues at runtime
        let font_data = include_bytes!("../assets/font.ttf");
        let font = FontRef::try_from_slice(font_data).expect("Error loading font");

        Self {
            font,
            width,
            height,
        }
    }

    pub fn generate_texture(&self, text: &str) -> Vec<u8> {
        let mut image = GrayImage::new(self.width, self.height);

        let scale = PxScale { x: 40.0, y: 40.0 };
        let scaled_font = self.font.as_scaled(scale);

        let mut x = 20.0;
        let mut y = 40.0; // Baseline

        for c in text.chars() {
            if c == '\n' {
                x = 20.0;
                y += 45.0; // Line height
                continue;
            }

            let mut glyph = scaled_font.scaled_glyph(c);
            glyph.position = point(x, y);
            let advance = scaled_font.h_advance(glyph.id);

            let outlined = scaled_font.outline_glyph(glyph);

            if let Some(outlined) = outlined {
                let bounds = outlined.px_bounds();
                outlined.draw(|gx, gy, c| {
                    let px = gx + bounds.min.x as u32;
                    let py = gy + bounds.min.y as u32;

                    if px < self.width && py < self.height {
                        let pixel = image.get_pixel_mut(px, py);
                        // Additive blending for overlap
                        let new_val = pixel.0[0].saturating_add((c * 255.0) as u8);
                        *pixel = Luma([new_val]);
                    }
                });
            }
            x += advance;

            if x > self.width as f32 - 40.0 {
                x = 20.0;
                y += 45.0;
            }
        }

        image.into_raw()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_text_generation() {
        let gen = TextGenerator::new(100, 100);
        let buffer = gen.generate_texture("A");
        assert_eq!(buffer.len(), 100 * 100);
        // "A" should result in some non-zero pixels
        assert!(buffer.iter().any(|&x| x > 0));
    }
}
