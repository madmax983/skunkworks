use image::{Rgba, RgbaImage};
use rand::{Rng, SeedableRng};
use rand_chacha::ChaCha20Rng;
use sha2::{Digest, Sha256};

pub struct CircuitGenerator {
    width: u32,
    height: u32,
    bg_color: Rgba<u8>,
    trace_color: Rgba<u8>,
    pad_color: Rgba<u8>,
    hole_color: Rgba<u8>,
}

impl CircuitGenerator {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            bg_color: Rgba([0, 68, 0, 255]),     // Dark Green
            trace_color: Rgba([0, 170, 0, 255]), // Light Green
            pad_color: Rgba([255, 215, 0, 255]), // Gold
            hole_color: Rgba([0, 0, 0, 255]),    // Black
        }
    }

    pub fn generate(&self, hash_seed: &str) -> (RgbaImage, Vec<(u32, u32)>) {
        let mut hasher = Sha256::new();
        hasher.update(hash_seed.as_bytes());
        let result = hasher.finalize();
        let seed: [u8; 32] = result.into();
        let mut rng = ChaCha20Rng::from_seed(seed);

        let mut img = RgbaImage::from_pixel(self.width, self.height, self.bg_color);
        let mut pads = Vec::new();

        let num_pads = rng.gen_range(32..64);
        let min_dist_sq = (6 * 2 + 8) * (6 * 2 + 8); // radius=6 + buffer=4 -> distance 20 -> 400

        // Generate Pads
        for _ in 0..num_pads {
            for _ in 0..100 {
                // Max attempts per pad
                let x = rng.gen_range(20..self.width - 20);
                let y = rng.gen_range(20..self.height - 20);

                let mut collision = false;
                for &(px, py) in &pads {
                    let dx = x as i32 - px as i32;
                    let dy = y as i32 - py as i32;
                    if dx * dx + dy * dy < min_dist_sq {
                        collision = true;
                        break;
                    }
                }

                if !collision {
                    pads.push((x, y));
                    break;
                }
            }
        }

        // Draw Traces first (so pads cover them)
        // Connect each pad to the nearest neighbor or random neighbor
        for i in 0..pads.len() {
            let start = pads[i];
            let end_idx = rng.gen_range(0..pads.len());
            if i == end_idx {
                continue;
            }
            let end = pads[end_idx];

            self.draw_trace(&mut img, start, end);
        }

        // Draw Pads
        for &(x, y) in &pads {
            self.draw_pad(&mut img, x, y);
        }

        (img, pads)
    }

    fn draw_trace(&self, img: &mut RgbaImage, start: (u32, u32), end: (u32, u32)) {
        // Draw L-shaped trace
        // Horizontal then Vertical or Vertical then Horizontal
        let (x1, y1) = start;
        let (x2, y2) = end;

        // Simple Manhattan routing: (x1, y1) -> (x2, y1) -> (x2, y2)
        self.draw_line(img, x1, y1, x2, y1, 3);
        self.draw_line(img, x2, y1, x2, y2, 3);
    }

    fn draw_line(&self, img: &mut RgbaImage, x0: u32, y0: u32, x1: u32, y1: u32, thickness: i32) {
        // Bresenham's line algorithm modified for thickness
        // For simplicity, just draw horizontal or vertical rectangles since we use Manhattan routing
        if x0 == x1 {
            // Vertical
            let min_y = y0.min(y1);
            let max_y = y0.max(y1);
            for y in min_y..=max_y {
                for t in -thickness / 2..=thickness / 2 {
                    let px = (x0 as i32 + t) as u32;
                    if px < self.width {
                        img.put_pixel(px, y, self.trace_color);
                    }
                }
            }
        } else if y0 == y1 {
            // Horizontal
            let min_x = x0.min(x1);
            let max_x = x0.max(x1);
            for x in min_x..=max_x {
                for t in -thickness / 2..=thickness / 2 {
                    let py = (y0 as i32 + t) as u32;
                    if py < self.height {
                        img.put_pixel(x, py, self.trace_color);
                    }
                }
            }
        } else {
            // Fallback for non-Manhattan segments if any
        }
    }

    fn draw_pad(&self, img: &mut RgbaImage, cx: u32, cy: u32) {
        let radius = 6;
        let hole_radius = 2;

        // Draw Gold Circle
        for y in (cy as i32 - radius)..(cy as i32 + radius + 1) {
            for x in (cx as i32 - radius)..(cx as i32 + radius + 1) {
                if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
                    continue;
                }
                let dx = x - cx as i32;
                let dy = y - cy as i32;
                if dx * dx + dy * dy <= radius * radius {
                    img.put_pixel(x as u32, y as u32, self.pad_color);
                }
            }
        }

        // Draw Black Hole
        for y in (cy as i32 - hole_radius)..(cy as i32 + hole_radius + 1) {
            for x in (cx as i32 - hole_radius)..(cx as i32 + hole_radius + 1) {
                if x < 0 || x >= self.width as i32 || y < 0 || y >= self.height as i32 {
                    continue;
                }
                let dx = x - cx as i32;
                let dy = y - cy as i32;
                if dx * dx + dy * dy <= hole_radius * hole_radius {
                    img.put_pixel(x as u32, y as u32, self.hole_color);
                }
            }
        }
    }
}
