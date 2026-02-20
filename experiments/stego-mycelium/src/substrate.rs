use macroquad::prelude::*;

pub struct StegoSubstrate {
    pub width: usize,
    pub height: usize,
    pub texture: Texture2D,
    pub image_data: Image,
}

impl StegoSubstrate {
    pub fn new(width: usize, height: usize) -> Self {
        let mut image_data = Image::gen_image_color(width as u16, height as u16, BLACK);

        // Generate a smooth gradient, but ensure LSBs are 0
        for y in 0..height {
            for x in 0..width {
                // Gradient
                let r = (x as f32 / width as f32 * 254.0) as u8;
                let g = (y as f32 / height as f32 * 254.0) as u8;
                let b = ((x + y) as f32 / (width + height) as f32 * 254.0) as u8;

                // Zero out LSBs
                let r = r & !1;
                let g = g & !1;
                let b = b & !1;

                image_data.set_pixel(x as u32, y as u32, Color::from_rgba(r, g, b, 255));
            }
        }

        let texture = Texture2D::from_image(&image_data);
        texture.set_filter(FilterMode::Nearest);

        Self {
            width,
            height,
            texture,
            image_data,
        }
    }

    pub fn embed_message(&mut self, message: &str) {
        let bytes = message.as_bytes();

        let mut x = 10;
        let mut base_y = self.height / 2;

        for byte in bytes.iter() {
            if x > self.width - 30 {
                x = 10;
                base_y = (base_y + 40) % (self.height - 20);
            }

            // Calculate a position
            let y = (base_y as f32 + (x as f32 * 0.05).sin() * 15.0) as usize;
            let y = y.clamp(1, self.height - 2);

            for b in 0..8 {
                let bit = (byte >> b) & 1;

                let px = x;
                let py = y + (b as usize % 2); // Stagger slightly

                if px < self.width && py < self.height {
                    let p = self.image_data.get_pixel(px as u32, py as u32);
                    let mut r = (p.r * 255.0) as u8;
                    // Force LSB to 1 if bit is 1.
                    // If bit is 0, we leave it as 0 (which matches cover).
                    // BUT for the fungus to follow the path, we need a signal.
                    // What if we invert the LSB for data?
                    // i.e. Data 0 -> LSB 1, Data 1 -> LSB 0?
                    // No, simpler:
                    // If bit is 1, set LSB 1.
                    // If bit is 0, set LSB 1? No that destroys data.

                    // Let's encode using 2 bits?
                    // Or let's just accept that 0s are gaps.
                    // But to ensure connectivity, maybe we add "marker" pixels around data?

                    // Helper: Use Blue channel for "Marker" (always 1 for data pixels)
                    // and Red channel LSB for actual data bit.
                    // The fungus follows Blue channel.

                    // Set Red LSB to data bit
                    r = (r & !1) | bit;

                    // Set Blue LSB to 1 (Marker)
                    let mut b_val = (p.b * 255.0) as u8;
                    b_val |= 1;

                    self.image_data.set_pixel(
                        px as u32,
                        py as u32,
                        Color::from_rgba(r, (p.g * 255.0) as u8, b_val, 255),
                    );
                }
                x += 1;
            }
            x += 2; // Space
        }

        self.texture.update(&self.image_data);
    }

    pub fn get_cost(&self, x: i32, y: i32) -> f32 {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return f32::INFINITY;
        }

        let p = self.image_data.get_pixel(x as u32, y as u32);
        // Check Blue LSB for "Marker"
        let b = (p.b * 255.0) as u8;
        if (b & 1) == 1 {
            1.0 // Data path
        } else {
            20.0 // Empty
        }
    }

    pub fn get_bit(&self, x: i32, y: i32) -> Option<u8> {
        if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
            return None;
        }
        let p = self.image_data.get_pixel(x as u32, y as u32);
        // Check Blue LSB for Marker
        let b = (p.b * 255.0) as u8;
        if (b & 1) == 0 {
            return None; // Not a data pixel
        }

        // Extract Data from Red LSB
        let r = (p.r * 255.0) as u8;
        Some(r & 1)
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
