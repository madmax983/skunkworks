use crate::memory::Memory;
use rand::Rng;

pub trait Decay {
    fn erode(&mut self);
    fn recall(&mut self, cx: u32, cy: u32, radius: u32);
    fn stress(&mut self, x: u32, y: u32, amount: f32);
    fn stress_line(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, amount: f32);
}

impl Decay for Memory {
    fn erode(&mut self) {
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

    fn recall(&mut self, cx: u32, cy: u32, radius: u32) {
        // Implementation from mnemosyne if needed, but not primary for this hybrid
        // Keeping it for compatibility/completeness
        // let mut rng = rand::thread_rng();
        let x_start = cx.saturating_sub(radius);
        let x_end = (cx + radius).min(self.width);
        let y_start = cy.saturating_sub(radius);
        let y_end = (cy + radius).min(self.height);

        for y in y_start..y_end {
            for x in x_start..x_end {
                let dx = x as i64 - cx as i64;
                let dy = y as i64 - cy as i64;
                if dx*dx + dy*dy > (radius as i64 * radius as i64) {
                    continue;
                }

                // Heal/Refresh logic could go here
                // For now, let's just slightly restore ground truth
                 let gt_pixel = *self.ground_truth.get_pixel(x, y);
                 let p_pixel = self.perceived.get_pixel_mut(x, y);

                 for c in 0..3 {
                     let gt = gt_pixel[c] as f32;
                     let p = p_pixel[c] as f32;
                     p_pixel[c] = (p * 0.9 + gt * 0.1) as u8;
                 }
            }
        }
    }

    fn stress(&mut self, x: u32, y: u32, amount: f32) {
        if x >= self.width || y >= self.height { return; }

        let mut rng = rand::thread_rng();
        // Probability of damage proportional to amount
        // If amount is high (e.g. 1.0), almost certain damage
        // We scale it down so it's not instant destruction
        if rng.gen::<f32>() > amount * 0.5 { return; }

        let pixel = self.perceived.get_pixel_mut(x, y);

        // Damage: Wear and Tear.
        // Scratches (white) or Cracks (black) or Color Shift
        let mode = rng.gen_range(0..3);
        match mode {
            0 => { // Fade to white (Crease mark)
                for c in 0..3 {
                    pixel[c] = pixel[c].saturating_add((amount * 100.0) as u8);
                }
            },
            1 => { // Fade to black (Deep crack)
                for c in 0..3 {
                    pixel[c] = pixel[c].saturating_sub((amount * 100.0) as u8);
                }
            },
            _ => { // Chromatic aberration / Noise
                 for c in 0..3 {
                    let noise = rng.gen_range(-50.0..50.0) * amount;
                    pixel[c] = (pixel[c] as f32 + noise).clamp(0.0, 255.0) as u8;
                 }
            }
        }
    }

    fn stress_line(&mut self, x0: u32, y0: u32, x1: u32, y1: u32, amount: f32) {
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
                     self.stress((x0 + 1).min(self.width as i32 - 1) as u32, y0 as u32, amount * 0.5);
                     self.stress(x0.saturating_sub(1) as u32, y0 as u32, amount * 0.5);
                     self.stress(x0 as u32, (y0 + 1).min(self.height as i32 - 1) as u32, amount * 0.5);
                     self.stress(x0 as u32, y0.saturating_sub(1) as u32, amount * 0.5);
                }
            }
            if x0 == x1 && y0 == y1 { break; }
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
