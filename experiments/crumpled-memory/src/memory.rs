use image::{Rgba, RgbaImage};
use rand::Rng;

pub struct Memory {
    pub width: u32,
    pub height: u32,
    pub perceived: RgbaImage,
}

impl Memory {
    pub fn new(width: u32, height: u32) -> Self {
        let mut ground_truth = RgbaImage::new(width, height);

        // Generate a test pattern: Gradient + Shapes
        for y in 0..height {
            for x in 0..width {
                let r = ((x as f32 / width as f32) * 255.0) as u8;
                let g = ((y as f32 / height as f32) * 255.0) as u8;
                let b = (((x + y) as f32 / (width + height) as f32) * 255.0) as u8;
                ground_truth.put_pixel(x, y, Rgba([r, g, b, 255]));
            }
        }

        // Add some shapes
        let center_x = width / 2;
        let center_y = height / 2;
        let radius = width.min(height) / 4;

        for y in 0..height {
            for x in 0..width {
                let dx = x as i32 - center_x as i32;
                let dy = y as i32 - center_y as i32;
                if dx * dx + dy * dy < (radius * radius) as i32 {
                    let p = ground_truth.get_pixel_mut(x, y);
                    p[0] = 255 - p[0];
                    p[1] = 255 - p[1];
                    p[2] = 255 - p[2];
                }
            }
        }

        let perceived = ground_truth.clone();

        Self {
            width,
            height,
            perceived,
        }
    }
}

impl Memory {
    pub fn erode(&mut self) {
        let mut rng = rand::thread_rng();
        let width = self.width;
        let height = self.height;

        // Iterate over a random subset of pixels to simulate sparse decay
        for _ in 0..2000 {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            let pixel = self.perceived.get_pixel_mut(x, y);

            // Drift towards grey or fade
            for c in 0..3 {
                let val = pixel[c] as f32;
                let new_val = if rng.gen_bool(0.5) {
                    val * 0.99
                } else {
                    val * 0.99 + 127.0 * 0.01
                };

                let noise = rng.gen_range(-2.0..2.0);
                pixel[c] = (new_val + noise).clamp(0.0, 255.0) as u8;
            }
        }
    }

    pub fn stress(&mut self, x: u32, y: u32, amount: f32) {
        if x >= self.width || y >= self.height {
            return;
        }

        let mut rng = rand::thread_rng();
        // Probability of damage proportional to amount
        // If amount is high (e.g. 1.0), almost certain damage
        // We scale it down so it's not instant destruction
        if rng.gen::<f32>() > amount * 0.5 {
            return;
        }

        let pixel = self.perceived.get_pixel_mut(x, y);

        // Damage: Wear and Tear.
        // Scratches (white) or Cracks (black) or Color Shift
        let mode = rng.gen_range(0..3);
        match mode {
            0 => {
                // Fade to white (Crease mark)
                for c in 0..3 {
                    pixel[c] = pixel[c].saturating_add((amount * 100.0) as u8);
                }
            }
            1 => {
                // Fade to black (Deep crack)
                for c in 0..3 {
                    pixel[c] = pixel[c].saturating_sub((amount * 100.0) as u8);
                }
            }
            _ => {
                // Chromatic aberration / Noise
                for c in 0..3 {
                    let noise = rng.gen_range(-50.0..50.0) * amount;
                    pixel[c] = (pixel[c] as f32 + noise).clamp(0.0, 255.0) as u8;
                }
            }
        }
    }

    pub fn stress_line(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, amount: f32) {
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
                // Apply stress with some thickness?
                // For now just the line
                self.stress(x0 as u32, y0 as u32, amount);

                // Add some thickness randomness
                let mut rng = rand::thread_rng();
                if rng.gen_bool(0.3) {
                    self.stress(
                        (x0 + 1).min(self.width as i32 - 1) as u32,
                        y0 as u32,
                        amount * 0.5,
                    );
                    self.stress(x0.saturating_sub(1) as u32, y0 as u32, amount * 0.5);
                    self.stress(
                        x0 as u32,
                        (y0 + 1).min(self.height as i32 - 1) as u32,
                        amount * 0.5,
                    );
                    self.stress(x0 as u32, y0.saturating_sub(1) as u32, amount * 0.5);
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
