use macroquad::prelude::*;
use rayon::prelude::*;

pub struct Map {
    pub width: usize,
    pub height: usize,
    pub trail: Vec<f32>,
    pub buffer: Vec<f32>,
    pub food: Vec<f32>,
    pub image: Image,
    pub texture: Texture2D,
}

impl Map {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let trail = vec![0.0; size];
        let buffer = vec![0.0; size];
        let food = vec![0.0; size];
        let image = Image::gen_image_color(width as u16, height as u16, BLACK);
        let texture = Texture2D::from_image(&image);

        Self {
            width,
            height,
            trail,
            buffer,
            food,
            image,
            texture,
        }
    }

    pub fn diffuse_and_decay(&mut self, decay_rate: f32) {
        let width = self.width;
        let height = self.height;

        // We write from trail to buffer
        // Parallel iter over buffer

        // We need immutable access to trail and mutable to buffer
        // But self has both. We can split.
        let (trail, buffer) = (&self.trail, &mut self.buffer);

        buffer.par_iter_mut().enumerate().for_each(|(i, val)| {
            let x = i % width;
            let y = i / width;

            let mut sum = 0.0;

            // 3x3 kernel blur
            for dy in -1..=1 {
                for dx in -1..=1 {
                    let nx = x as isize + dx;
                    let ny = y as isize + dy;

                    if nx >= 0 && nx < width as isize && ny >= 0 && ny < height as isize {
                        sum += trail[ny as usize * width + nx as usize];
                    }
                }
            }

            *val = (sum / 9.0) * decay_rate;
        });

        // Swap trail and buffer
        std::mem::swap(&mut self.trail, &mut self.buffer);
    }

    pub fn update_texture(&mut self) {
        let trail = &self.trail;
        let food = &self.food;
        let bytes = &mut self.image.bytes;

        bytes.par_chunks_mut(4).enumerate().for_each(|(i, pixel)| {
            if i < trail.len() {
                let t = trail[i];
                let f = food[i];

                let trail_val = (t * 5.0).min(1.0); // Boost visibility
                let food_val = (f * 1.0).min(1.0);

                // Colors:
                // Trail: Cyan/Blue (0, 255, 255)
                // Food: Orange/Red (255, 100, 0)

                let r = (food_val * 255.0) as u8;
                let g = ((trail_val * 0.8 + food_val * 0.4) * 255.0).min(255.0) as u8;
                let b = ((trail_val * 1.0) * 255.0).min(255.0) as u8;

                pixel[0] = r;
                pixel[1] = g;
                pixel[2] = b;
                pixel[3] = 255;
            }
        });

        self.texture.update(&self.image);
    }
}
