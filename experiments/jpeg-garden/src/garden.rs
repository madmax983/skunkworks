use crate::dct::{dct_2d, idct_2d};
use image::{Rgba, RgbaImage};
use rand::Rng;

#[derive(Clone, Copy)]
pub struct Block {
    pub y: [f32; 64],
    pub cb: [f32; 64],
    pub cr: [f32; 64],
}

impl Block {
    pub fn new() -> Self {
        Self {
            y: [0.0; 64],
            cb: [0.0; 64],
            cr: [0.0; 64],
        }
    }

    pub fn distance(&self, other: &Block) -> f32 {
        let mut sum = 0.0;
        for i in 0..64 {
            sum += (self.y[i] - other.y[i]).abs();
            sum += (self.cb[i] - other.cb[i]).abs();
            sum += (self.cr[i] - other.cr[i]).abs();
        }
        sum
    }
}

pub struct EntropyMetrics {
    pub total_error: f32,
    pub max_block_error: f32,
}

pub struct Garden {
    pub width_blocks: u32,
    pub height_blocks: u32,
    pub blocks: Vec<Block>,
    pub original_blocks: Vec<Block>,
}

impl Garden {
    pub fn new(width_blocks: u32, height_blocks: u32) -> Self {
        let count = (width_blocks * height_blocks) as usize;
        Self {
            width_blocks,
            height_blocks,
            blocks: vec![Block::new(); count],
            original_blocks: vec![Block::new(); count],
        }
    }

    pub fn from_image(img: &RgbaImage) -> Self {
        let (w, h) = img.dimensions();
        let width_blocks = w / 8;
        let height_blocks = h / 8;
        let mut garden = Garden::new(width_blocks, height_blocks);

        for by in 0..height_blocks {
            for bx in 0..width_blocks {
                let mut y_block = [0.0; 64];
                let mut cb_block = [0.0; 64];
                let mut cr_block = [0.0; 64];

                for y in 0..8 {
                    for x in 0..8 {
                        let px = img.get_pixel(bx * 8 + x, by * 8 + y);
                        let r = px[0] as f32;
                        let g = px[1] as f32;
                        let b = px[2] as f32;

                        let y_val = 0.299 * r + 0.587 * g + 0.114 * b - 128.0;
                        let cb_val = -0.1687 * r - 0.3313 * g + 0.5 * b;
                        let cr_val = 0.5 * r - 0.4187 * g - 0.0813 * b;

                        y_block[(y * 8 + x) as usize] = y_val;
                        cb_block[(y * 8 + x) as usize] = cb_val;
                        cr_block[(y * 8 + x) as usize] = cr_val;
                    }
                }

                let idx = (by * width_blocks + bx) as usize;
                let block = Block {
                    y: dct_2d(&y_block),
                    cb: dct_2d(&cb_block),
                    cr: dct_2d(&cr_block),
                };
                garden.blocks[idx] = block;
                garden.original_blocks[idx] = block;
            }
        }
        garden
    }

    pub fn to_image(&self) -> RgbaImage {
        let w = self.width_blocks * 8;
        let h = self.height_blocks * 8;
        let mut img = RgbaImage::new(w, h);

        for by in 0..self.height_blocks {
            for bx in 0..self.width_blocks {
                let idx = (by * self.width_blocks + bx) as usize;
                let block = &self.blocks[idx];

                let y_spatial = idct_2d(&block.y);
                let cb_spatial = idct_2d(&block.cb);
                let cr_spatial = idct_2d(&block.cr);

                for y in 0..8 {
                    for x in 0..8 {
                        let i = (y * 8 + x) as usize;
                        let y_val = y_spatial[i] + 128.0;
                        let cb_val = cb_spatial[i];
                        let cr_val = cr_spatial[i];

                        let r = (y_val + 1.402 * cr_val).clamp(0.0, 255.0) as u8;
                        let g =
                            (y_val - 0.34414 * cb_val - 0.71414 * cr_val).clamp(0.0, 255.0) as u8;
                        let b = (y_val + 1.772 * cb_val).clamp(0.0, 255.0) as u8;

                        img.put_pixel(bx * 8 + x, by * 8 + y, Rgba([r, g, b, 255]));
                    }
                }
            }
        }
        img
    }

    pub fn get_entropy_metrics(&self) -> EntropyMetrics {
        let mut total_error = 0.0;
        let mut max_block_error = 0.0;

        for (b, ob) in self.blocks.iter().zip(self.original_blocks.iter()) {
            let err = b.distance(ob);
            total_error += err;
            if err > max_block_error {
                max_block_error = err;
            }
        }

        EntropyMetrics {
            total_error,
            max_block_error,
        }
    }

    pub fn tick(&mut self) {
        let mut rng = rand::thread_rng();
        let total_blocks = self.blocks.len();

        if total_blocks == 0 {
            return;
        }

        for _ in 0..5 {
            let idx = rng.gen_range(0..total_blocks);
            let block = &mut self.blocks[idx];

            let high_freq_idx = rng.gen_range(10..64);
            if rng.gen_bool(0.3) {
                block.y[high_freq_idx] = 0.0;
                block.cb[high_freq_idx] = 0.0;
                block.cr[high_freq_idx] = 0.0;
            }

            if rng.gen_bool(0.2) {
                let noise_idx = rng.gen_range(0..64);
                let noise = rng.gen_range(-10.0..10.0);
                block.y[noise_idx] += noise;
            }

            if rng.gen_bool(0.05) {
                block.cb[0] += rng.gen_range(-5.0..5.0);
                block.cr[0] += rng.gen_range(-5.0..5.0);
            }

            if rng.gen_bool(0.05) {
                let idx = rng.gen_range(32..64);
                block.y[idx] *= 1.1;
            }
        }
    }
}
