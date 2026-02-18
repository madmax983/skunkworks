use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};

pub struct Painter {
    pub pos: Vec2,
    pub vel: Vec2,
    pub color: Color,
    pub size: f32,
    noise_offset: f32,
}

impl Painter {
    pub fn new(x: f32, y: f32, color: Color) -> Self {
        Self {
            pos: vec2(x, y),
            vel: vec2(0.0, 0.0),
            color,
            size: 5.0,
            noise_offset: ::rand::random::<f32>() * 1000.0,
        }
    }

    pub fn update(&mut self, dt: f32, energy: f32, noise: &Perlin, bounds: Vec2) {
        // Perlin noise for wandering
        let nx = noise.get([self.noise_offset as f64, 0.0]) as f32;
        let ny = noise.get([self.noise_offset as f64, 100.0]) as f32;

        // Energy affects speed and randomness
        let speed = 50.0 + energy * 300.0;
        let force = vec2(nx, ny) * speed;

        self.vel += force * dt;
        // Dampen velocity
        self.vel *= 0.95;

        self.pos += self.vel * dt;

        // Wrap around
        if self.pos.x < 0.0 { self.pos.x = bounds.x; }
        if self.pos.x > bounds.x { self.pos.x = 0.0; }
        if self.pos.y < 0.0 { self.pos.y = bounds.y; }
        if self.pos.y > bounds.y { self.pos.y = 0.0; }

        self.noise_offset += dt * (0.5 + energy * 2.0);
        // Modulate size by energy
        self.size = 2.0 + energy * 20.0;
    }
}

pub struct FluidCanvas {
    pub width: usize,
    pub height: usize,
    pub pixels: Vec<u8>, // RGBA
}

impl FluidCanvas {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            pixels: vec![0; width * height * 4],
        }
    }

    pub fn paint(&mut self, x: f32, y: f32, radius: f32, color: Color) {
        let ix = x as i32;
        let iy = y as i32;
        let r = radius as i32;
        let r2 = r * r;

        // Pre-calculate color bytes
        let cr = (color.r * 255.0) as u8;
        let cg = (color.g * 255.0) as u8;
        let cb = (color.b * 255.0) as u8;
        let ca = color.a;

        for dy in -r..=r {
            for dx in -r..=r {
                if dx*dx + dy*dy <= r2 {
                    let nx = ix + dx;
                    let ny = iy + dy;
                    if nx >= 0 && nx < self.width as i32 && ny >= 0 && ny < self.height as i32 {
                        let idx = ((ny * self.width as i32 + nx) * 4) as usize;

                        // Blend
                        let old_r = self.pixels[idx];
                        let old_g = self.pixels[idx+1];
                        let old_b = self.pixels[idx+2];

                        let new_r = (old_r as f32 * (1.0 - ca) + cr as f32 * ca) as u8;
                        let new_g = (old_g as f32 * (1.0 - ca) + cg as f32 * ca) as u8;
                        let new_b = (old_b as f32 * (1.0 - ca) + cb as f32 * ca) as u8;

                        self.pixels[idx] = new_r;
                        self.pixels[idx+1] = new_g;
                        self.pixels[idx+2] = new_b;
                        self.pixels[idx+3] = 255; // Always opaque output
                    }
                }
            }
        }
    }

    // Simple box blur to simulate diffusion/mixing
    pub fn diffuse(&mut self) {
        let w = self.width;
        let h = self.height;
        // We create a copy for the source.
        // Optimization: Double buffering could avoid allocation, but cloning Vec<u8> is fast enough for small grids.
        let source = self.pixels.clone();

        for y in 1..h-1 {
            for x in 1..w-1 {
                let idx = (y * w + x) * 4;

                let up = ((y-1)*w + x) * 4;
                let down = ((y+1)*w + x) * 4;
                let left = (y*w + x-1) * 4;
                let right = (y*w + x+1) * 4;

                // R
                let r = (source[up] as u16 + source[down] as u16 + source[left] as u16 + source[right] as u16) >> 2;
                // G
                let g = (source[up+1] as u16 + source[down+1] as u16 + source[left+1] as u16 + source[right+1] as u16) >> 2;
                // B
                let b = (source[up+2] as u16 + source[down+2] as u16 + source[left+2] as u16 + source[right+2] as u16) >> 2;

                self.pixels[idx] = r as u8;
                self.pixels[idx+1] = g as u8;
                self.pixels[idx+2] = b as u8;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_canvas_bounds() {
        let mut canvas = FluidCanvas::new(100, 100);
        // Paint outside bounds should not panic
        canvas.paint(-10.0, -10.0, 5.0, RED);
        canvas.paint(110.0, 110.0, 5.0, RED);

        // Paint inside
        canvas.paint(50.0, 50.0, 10.0, Color::new(1.0, 0.0, 0.0, 1.0));
        let center_idx = (50 * 100 + 50) * 4;
        println!("Pixel at center: {:?}", &canvas.pixels[center_idx..center_idx+4]);
        assert!(canvas.pixels[center_idx] > 250); // Red channel should be near 255
        assert_eq!(canvas.pixels[center_idx+1], 0); // Green channel
    }
}
