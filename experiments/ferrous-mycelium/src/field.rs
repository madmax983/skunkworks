use macroquad::prelude::*;

pub struct MagneticField {
    pub width: usize,
    pub height: usize,
    pub grid: Vec<f32>,
    pub texture: Texture2D,
    image: Image,
}

impl MagneticField {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let grid = vec![0.5; size];
        let image = Image::gen_image_color(width as u16, height as u16, BLACK);
        let texture = Texture2D::from_image(&image);

        Self {
            width,
            height,
            grid,
            texture,
            image,
        }
    }

    pub fn get_magnetism(&self, x: usize, y: usize) -> f32 {
        if x < self.width && y < self.height {
            self.grid[y * self.width + x]
        } else {
            0.5
        }
    }

    // Sample with bilinear interpolation for smoother gradients?
    // For now, nearest neighbor is fine.
    pub fn sample(&self, x: f32, y: f32) -> f32 {
        let ix = x.clamp(0.0, (self.width - 1) as f32) as usize;
        let iy = y.clamp(0.0, (self.height - 1) as f32) as usize;
        self.get_magnetism(ix, iy)
    }

    pub fn magnetize(&mut self, x: usize, y: usize, amount: f32) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.grid[idx] = (self.grid[idx] + amount).clamp(0.0, 1.0);
        }
    }

    pub fn decay(&mut self, rate: f32) {
        for val in &mut self.grid {
            // Decay towards 0.5
            *val = (*val - 0.5) * rate + 0.5;
        }
    }

    pub fn update_texture(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let val = self.grid[y * self.width + x];
                // Color mapping:
                // 0.0 (North) -> Blue
                // 0.5 (Neutral) -> Black/Transparent
                // 1.0 (South) -> Red

                let color = if val < 0.45 {
                    // Blue intensity: (0.5 - val) * 2.0
                    Color::new(0.0, 0.0, (0.5 - val) * 2.0, 1.0)
                } else if val > 0.55 {
                    // Red intensity: (val - 0.5) * 2.0
                    Color::new((val - 0.5) * 2.0, 0.0, 0.0, 1.0)
                } else {
                    BLACK
                };

                self.image.set_pixel(x as u32, y as u32, color);
            }
        }
        self.texture.update(&self.image);
    }

    pub fn draw(&self) {
        draw_texture_ex(&self.texture, 0.0, 0.0, WHITE, DrawTextureParams {
            dest_size: Some(vec2(screen_width(), screen_height())),
            ..Default::default()
        });
    }
}
