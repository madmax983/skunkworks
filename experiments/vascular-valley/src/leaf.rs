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
                let v = (y as f32 - cy) / scale; // y is inverted in screen space usually, but here it's just a grid

                // Leaf Shape Formula: Simple Ovate
                // r = 1.0 - 0.3 * |sin(theta)| produces a slight pinch?
                // Let's use implicit equation for a simple lens/vesica piscis shape?
                // Or: x^2 + (y/1.5)^2 <= 1.0 (Ellipse)
                // Let's use a "Teardrop" shape:
                // x = cos(t) * (1 - sin(t))
                // y = sin(t)

                // Let's stick to a simple modulated ellipse for now.
                // dist from center
                let r = (u * u + v * v).sqrt();
                let theta = v.atan2(u);

                // Polar shape: r_max = 1.0 - 0.5 * |sin(theta)| gives a peanut.
                // Let's try: r_max = (1.0 - |v|) * 0.8 for width?

                // Classic Leaf Shape approximation:
                // x = r * (1 - |y|)
                // Let's just use: |u| < (1.0 - v*v).sqrt() * (1.0 - 0.3*v)
                // A distorted circle.

                // Using distance from (0,0) with a modulation on angle.
                // r_boundary = 1.0 + 0.2 * cos(3.0 * theta + 0.5);
                // No, that's a clover.

                // Let's use the "Betel leaf" implicit:
                // x^2 + y^2 - |x| * y = ...

                // Simpler: Just an ellipse for the first version.
                // u^2 + (v/1.5)^2 < 1.0
                let is_inside = (u * u + (v / 1.5).powi(2)) < 1.0;

                // Use theta to avoid warning
                let _ = theta;

                let idx = y * self.width + x;
                self.mask[idx] = is_inside;

                if is_inside {
                    // Initial Heightmap
                    // 1. General slope: High in center (midrib), low at edges.
                    // d = distance from centerline (u=0)
                    // h = 1.0 - |u|

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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_leaf_shape() {
        let mut leaf = LeafMap::new(100, 100);
        leaf.generate_shape();

        // Center should be inside
        assert!(leaf.is_inside(50, 50));

        // Corner should be outside
        assert!(!leaf.is_inside(0, 0));
        assert!(!leaf.is_inside(99, 99));
    }
}
