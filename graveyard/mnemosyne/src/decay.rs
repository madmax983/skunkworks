use crate::memory::Memory;
use rand::Rng;

pub trait Decay {
    fn erode(&mut self);
    fn recall(&mut self, cx: u32, cy: u32, radius: u32);
}

impl Decay for Memory {
    fn erode(&mut self) {
        let mut rng = rand::thread_rng();
        let width = self.width;
        let height = self.height;

        // Iterate over a random subset of pixels to simulate sparse decay
        // Let's pick 1000 pixels per frame to corrupt
        for _ in 0..2000 {
            let x = rng.gen_range(0..width);
            let y = rng.gen_range(0..height);

            let pixel = self.perceived.get_pixel_mut(x, y);

            // Drift towards grey (127) or black (0) depending on channel
            // Simulating loss of signal strength or noise floor rising
            for c in 0..3 {
                let val = pixel[c] as f32;
                // 1% drift towards 0 (fading)
                // 0.5% drift towards 127 (grey noise)
                let new_val = if rng.gen_bool(0.5) {
                    val * 0.99
                } else {
                    val * 0.99 + 127.0 * 0.01
                };

                // Add random noise
                let noise = rng.gen_range(-2.0..2.0);
                pixel[c] = (new_val + noise).clamp(0.0, 255.0) as u8;
            }
        }

        // Occasional heavy glitch: Vertical line drift
        if rng.gen_bool(0.05) {
            let x = rng.gen_range(0..width);
            let _shift = rng.gen_range(-5..5);
            // We can't shift easily in place without a buffer, so just add noise line
            for y in 0..height {
                let p = self.perceived.get_pixel_mut(x, y);
                p[0] = p[0].saturating_add(10);
                p[1] = p[1].saturating_add(10);
                p[2] = p[2].saturating_add(10);
            }
        }
    }

    fn recall(&mut self, cx: u32, cy: u32, radius: u32) {
        let mut rng = rand::thread_rng();

        let x_start = cx.saturating_sub(radius);
        let x_end = (cx + radius).min(self.width);
        let y_start = cy.saturating_sub(radius);
        let y_end = (cy + radius).min(self.height);

        for y in y_start..y_end {
            for x in x_start..x_end {
                // Circular region check
                let dx = x as i64 - cx as i64;
                let dy = y as i64 - cy as i64;
                if dx * dx + dy * dy > (radius as i64 * radius as i64) {
                    continue;
                }

                // Get current values
                let gt_pixel = *self.ground_truth.get_pixel(x, y);
                let p_pixel = *self.perceived.get_pixel(x, y);

                // 1. RECONSOLIDATION: Ground Truth drifts towards the distorted Perceived state.
                // This simulates "remembering the memory", not the event.
                let mut new_gt = gt_pixel;
                for c in 0..3 {
                    let gt_val = gt_pixel[c] as f32;
                    let p_val = p_pixel[c] as f32;
                    // Move GT 5% towards Perceived
                    new_gt[c] = (gt_val * 0.95 + p_val * 0.05).clamp(0.0, 255.0) as u8;
                }
                self.ground_truth.put_pixel(x, y, new_gt);

                // 2. REFRESH: Perceived becomes the new Ground Truth (mostly)
                // But with added quantization artifacts (mental compression)
                let mut new_p = new_gt;

                // Quantization levels decrease over time?
                // Let's just quantize to nearest 32 values
                for c in 0..3 {
                    let val = new_p[c] as f32;
                    // Add some jitter before quantization
                    let jitter = rng.gen_range(-5.0..5.0);
                    // Quantize
                    let q = 16.0;
                    let quantized = ((val + jitter) / q).round() * q;
                    new_p[c] = quantized.clamp(0.0, 255.0) as u8;
                }

                // Occasional spatial jitter (swap with neighbor)
                if rng.gen_bool(0.01) {
                    let nx =
                        (x as i32 + rng.gen_range(-1..2)).clamp(0, (self.width - 1) as i32) as u32;
                    let ny =
                        (y as i32 + rng.gen_range(-1..2)).clamp(0, (self.height - 1) as i32) as u32;
                    // We can't swap easily without reading again, just take the neighbor's value
                    // This is lossy, which is good.
                    let neighbor = *self.ground_truth.get_pixel(nx, ny);
                    new_p = neighbor;
                }

                self.perceived.put_pixel(x, y, new_p);
            }
        }
    }
}
