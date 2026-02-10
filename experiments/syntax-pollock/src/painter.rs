use image::{Rgb, RgbImage};
use rand::Rng;

pub struct Canvas {
    pub image: RgbImage,
    width: u32,
    height: u32,
}

impl Canvas {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            image: RgbImage::new(width, height),
            width,
            height,
        }
    }

    pub fn splash(&mut self, x: u32, y: u32, color: [u8; 3], size: u32) {
        let radius = size as i32;
        let cx = x as i32;
        let cy = y as i32;
        // Simple circle
        for dy in -radius..=radius {
            for dx in -radius..=radius {
                if dx*dx + dy*dy <= radius*radius {
                    let px = cx + dx;
                    let py = cy + dy;
                    if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
                         self.image.put_pixel(px as u32, py as u32, Rgb(color));
                    }
                }
            }
        }

        // Add some "splatter" droplets
        let mut rng = rand::thread_rng();
        for _ in 0..size {
             let dx = rng.gen_range(-((size * 2) as i32)..=(size * 2) as i32);
             let dy = rng.gen_range(-((size * 2) as i32)..=(size * 2) as i32);
             let px = cx + dx;
             let py = cy + dy;
             if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
                 self.image.put_pixel(px as u32, py as u32, Rgb(color));
             }
        }
    }

    pub fn stroke(&mut self, start: (u32, u32), end: (u32, u32), color: [u8; 3], width: u32) {
        // Simple line drawing (Bresenham-ish)
        let x0 = start.0 as i32;
        let y0 = start.1 as i32;
        let x1 = end.0 as i32;
        let y1 = end.1 as i32;

        let dx = (x1 - x0).abs();
        let dy = -(y1 - y0).abs();
        let sx = if x0 < x1 { 1 } else { -1 };
        let sy = if y0 < y1 { 1 } else { -1 };
        let mut err = dx + dy;

        let mut x = x0;
        let mut y = y0;

        loop {
            // Draw a brush tip at (x, y) with size `width`
            let r = width as i32 / 2;
             for dy_b in -r..=r {
                for dx_b in -r..=r {
                     if dx_b*dx_b + dy_b*dy_b <= r*r {
                         let px = x + dx_b;
                         let py = y + dy_b;
                         if px >= 0 && px < self.width as i32 && py >= 0 && py < self.height as i32 {
                             self.image.put_pixel(px as u32, py as u32, Rgb(color));
                         }
                     }
                }
            }

            if x == x1 && y == y1 { break; }
            let e2 = 2 * err;
            if e2 >= dy { err += dy; x += sx; }
            if e2 <= dx { err += dx; y += sy; }
        }
    }

    pub fn drip(&mut self, x: u32, y: u32, length: u32, color: [u8; 3]) {
        self.stroke((x, y), (x, y + length), color, 2);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_splash() {
        let mut canvas = Canvas::new(100, 100);
        let color = [255, 0, 0];
        canvas.splash(50, 50, color, 10);
        // Check center pixel is colored
        assert_eq!(canvas.image.get_pixel(50, 50).0, color);
    }
}
