use image::{ImageBuffer, Rgb};

const OFFSETS: &[(i32, i32)] = &[
    (0, -2),  // Top
    (1, -1),  // Top-Right
    (2, 0),   // Right
    (1, 1),   // Bottom-Right
    (0, 2),   // Bottom
    (-1, 1),  // Bottom-Left
    (-2, 0),  // Left
    (-1, -1), // Top-Left
];

pub struct Glyph {
    pub byte: u8,
}

impl Glyph {
    pub fn new(byte: u8) -> Self {
        Self { byte }
    }

    pub fn render(&self, center_x: u32, center_y: u32, img: &mut ImageBuffer<Rgb<u8>, Vec<u8>>) {
        let w = img.width() as i32;
        let h = img.height() as i32;
        let cx = center_x as i32;
        let cy = center_y as i32;

        // Draw Core
        if cx >= 0 && cx < w && cy >= 0 && cy < h {
            img.put_pixel(center_x, center_y, Rgb([255, 255, 255]));
        }

        for (i, (dx, dy)) in OFFSETS.iter().enumerate() {
            if (self.byte >> i) & 1 == 1 {
                let px = cx + dx;
                let py = cy + dy;
                if px >= 0 && px < w && py >= 0 && py < h {
                    // Ray color slightly dimmer/blue
                    img.put_pixel(px as u32, py as u32, Rgb([200, 200, 240]));
                }
            }
        }
    }

    pub fn decode(center_x: u32, center_y: u32, img: &ImageBuffer<Rgb<u8>, Vec<u8>>) -> u8 {
        let w = img.width() as i32;
        let h = img.height() as i32;
        let cx = center_x as i32;
        let cy = center_y as i32;

        let mut byte = 0;

        for (i, (dx, dy)) in OFFSETS.iter().enumerate() {
            let px = cx + dx;
            let py = cy + dy;

            if px >= 0 && px < w && py >= 0 && py < h {
                let pixel = img.get_pixel(px as u32, py as u32);
                // Simple threshold check.
                // If sum of channels > threshold
                let brightness = pixel[0] as u16 + pixel[1] as u16 + pixel[2] as u16;
                if brightness > 300 {
                    // 100 per channel avg
                    byte |= 1 << i;
                }
            }
        }
        byte
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use image::ImageBuffer;

    #[test]
    fn test_render_decode() {
        let mut img = ImageBuffer::new(10, 10);
        let byte = 0b10101010; // 170
        let glyph = Glyph::new(byte);

        // Render at 5,5
        glyph.render(5, 5, &mut img);

        // Decode at 5,5
        let decoded = Glyph::decode(5, 5, &img);

        assert_eq!(byte, decoded, "Decoded byte should match encoded byte");
    }
}
