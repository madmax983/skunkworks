use image::{ImageBuffer, Rgba};

const OFFSETS: [(i32, i32); 8] = [
    (2, 0), (2, 2), (0, 2), (-2, 2),
    (-2, 0), (-2, -2), (0, -2), (2, -2)
];

pub struct Glyph {
    pub byte: u8,
}

impl Glyph {
    pub fn new(byte: u8) -> Self {
        Self { byte }
    }

    pub fn render(&self, center_x: u32, center_y: u32, img: &mut ImageBuffer<Rgba<u8>, Vec<u8>>) {
        let w = img.width() as i32;
        let h = img.height() as i32;
        let cx = center_x as i32;
        let cy = center_y as i32;

        // Draw Core
        if cx >= 0 && cx < w && cy >= 0 && cy < h {
            img.put_pixel(center_x, center_y, Rgba([255, 255, 255, 255]));
        }

        for (i, (dx, dy)) in OFFSETS.iter().enumerate() {
            if (self.byte >> i) & 1 == 1 {
                let px = cx + dx;
                let py = cy + dy;
                if px >= 0 && px < w && py >= 0 && py < h {
                    // Ray color slightly dimmer/blue
                    img.put_pixel(px as u32, py as u32, Rgba([200, 200, 240, 255]));
                }
            }
        }
    }
}
