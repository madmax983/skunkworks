use image::{ImageBuffer, Rgb};

// Fixed DNA: Radial Encoding Offsets (Radius 2)
// 8 rays at 45 degree intervals
const OFFSETS: [(i32, i32); 8] = [
    (2, 0),   // 0: East
    (2, 2),   // 1: South-East
    (0, 2),   // 2: South
    (-2, 2),  // 3: South-West
    (-2, 0),  // 4: West
    (-2, -2), // 5: North-West
    (0, -2),  // 6: North
    (2, -2),  // 7: North-East
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
        let mut img = ImageBuffer::new(20, 20); // Increased size for radius 2
        let byte = 0b10101010; // 170
        let glyph = Glyph::new(byte);

        // Render at 10,10
        glyph.render(10, 10, &mut img);

        // Decode at 10,10
        let decoded = Glyph::decode(10, 10, &img);

        assert_eq!(byte, decoded, "Decoded byte should match encoded byte");
    }
}
