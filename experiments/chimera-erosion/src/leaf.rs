use macroquad::prelude::*;
use noise::{Fbm, NoiseFn, Perlin};

pub struct LeafMap {
    pub width: usize,
    pub height: usize,
    pub heightmap: Vec<f32>,
    pub water: Vec<f32>,
    pub mask: Vec<bool>,
}

impl LeafMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            heightmap: vec![0.0; width * height],
            water: vec![0.0; width * height],
            mask: vec![false; width * height],
        }
    }

    pub fn generate_shape(&mut self) {
        let cx = self.width as f32 / 2.0;
        let cy = self.height as f32 / 2.0;
        let scale = self.height as f32 * 0.45;

        let fbm = Fbm::<Perlin>::new(0);

        for y in 0..self.height {
            for x in 0..self.width {
                let u = (x as f32 - cx) / scale;
                let v = (y as f32 - cy) / scale;

                // Simple Ellipse for now.
                // u^2 + (v/1.5)^2 < 1.0
                let is_inside = (u * u + (v / 1.5).powi(2)) < 1.0;

                let idx = y * self.width + x;
                self.mask[idx] = is_inside;

                if is_inside {
                    let r = (u * u + v * v).sqrt();
                    // Initial Heightmap
                    // Cone shape modulated by noise
                    let base_height = 1.0 - r.min(1.0);

                    // Add some noise for terrain variety
                    let noise_val = fbm.get([u as f64 * 3.0, v as f64 * 3.0]) as f32;

                    self.heightmap[idx] = base_height + noise_val * 0.2;
                } else {
                    self.heightmap[idx] = 0.0;
                }
            }
        }
    }

    pub fn is_inside(&self, x: usize, y: usize) -> bool {
        if x >= self.width || y >= self.height {
            return false;
        }
        self.mask[y * self.width + x]
    }
}
