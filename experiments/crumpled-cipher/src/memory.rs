use crate::starmap::StarMap;
use image::{Rgba, RgbaImage};
use rand::Rng;

pub struct Memory {
    pub width: u32,
    pub height: u32,
    pub hidden_layer: RgbaImage,
    pub visible_layer: RgbaImage,
}

impl Memory {
    pub fn new(width: u32, height: u32) -> Self {
        // Generate Hidden Layer (StarMap)
        let message = "THE UNIVERSE IS FOLDED. LIGHT EMERGES FROM THE CREASE.";

        // Ensure starmap matches dimensions (resize if needed, but starmap generates based on payload)
        // Actually, starmap generates size based on payload length and width.
        // We want it to fill the texture.

        // Let's generate a starmap and then resize/crop or tile it?
        // Or just force starmap width.
        // StarMap::new takes width_glyphs. width_pixels = width_glyphs * 12.
        let width_glyphs = width / 12;
        let sm = StarMap::new(message.as_bytes(), width_glyphs);
        let hidden_generated = sm.generate();

        // Resize hidden_generated to match target width/height if needed, or just paste it
        let mut hidden_layer = RgbaImage::new(width, height);

        // Fill hidden layer with void
        for p in hidden_layer.pixels_mut() {
            *p = Rgba([5, 5, 15, 255]);
        }

        // Paste generated starmap
        for y in 0..hidden_generated.height().min(height) {
            for x in 0..hidden_generated.width().min(width) {
                hidden_layer.put_pixel(x, y, *hidden_generated.get_pixel(x, y));
            }
        }

        // Visible Layer: Start as Void
        let mut visible_layer = RgbaImage::new(width, height);
        for p in visible_layer.pixels_mut() {
            *p = Rgba([0, 0, 0, 255]); // Pitch black initially
        }

        Self {
            width,
            height,
            hidden_layer,
            visible_layer,
        }
    }
}

impl Memory {
    pub fn reveal(&mut self, x: u32, y: u32, amount: f32) {
        if x >= self.width || y >= self.height {
            return;
        }

        // Probability of reveal proportional to amount
        let mut rng = rand::thread_rng();
        if rng.gen::<f32>() > amount * 0.8 {
            // Slightly easier to reveal than destroy
            return;
        }

        let hidden_pixel = *self.hidden_layer.get_pixel(x, y);
        let visible_pixel = self.visible_layer.get_pixel_mut(x, y);

        // Copy hidden to visible
        // Maybe blend it?
        // visible = hidden * amount + visible * (1-amount)?
        // For now, hard copy to ensure clarity of the cipher
        *visible_pixel = hidden_pixel;

        // Add a "glow" effect by brightening the revealed pixel slightly if it's a star?
        // Or just let the StarMap handle it.
        // If amount is high, maybe make it brighter (overexposed)
        if amount > 0.8 {
            for c in 0..3 {
                visible_pixel[c] = visible_pixel[c].saturating_add(20);
            }
        }
    }

    pub fn reveal_line(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, amount: f32) {
        // Bresenham's algorithm
        let mut x0 = x0 as i32;
        let mut y0 = y0 as i32;
        let x1 = x1 as i32;
        let y1 = y1 as i32;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        loop {
            if x0 >= 0 && x0 < self.width as i32 && y0 >= 0 && y0 < self.height as i32 {
                self.reveal(x0 as u32, y0 as u32, amount);

                // Add thickness/spread randomness
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.4) {
                    self.reveal(
                        (x0 + 1).min(self.width as i32 - 1) as u32,
                        y0 as u32,
                        amount * 0.6,
                    );
                    self.reveal(x0.saturating_sub(1) as u32, y0 as u32, amount * 0.6);
                    self.reveal(
                        x0 as u32,
                        (y0 + 1).min(self.height as i32 - 1) as u32,
                        amount * 0.6,
                    );
                    self.reveal(x0 as u32, y0.saturating_sub(1) as u32, amount * 0.6);
                }
            }
            if x0 == x1 && y0 == y1 {
                break;
            }
            let e2 = 2 * err;
            if e2 >= dy {
                err += dy;
                x0 += sx;
            }
            if e2 <= dx {
                err += dx;
                y0 += sy;
            }
        }
    }
}
