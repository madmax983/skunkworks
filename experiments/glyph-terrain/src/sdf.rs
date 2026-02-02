use rusttype::{Font, Scale};

#[derive(Debug, Clone)]
pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub data: Vec<f32>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0.0; width * height],
        }
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        if x >= self.width || y >= self.height {
            0.0
        } else {
            self.data[y * self.width + x]
        }
    }

    pub fn set(&mut self, x: usize, y: usize, v: f32) {
        if x < self.width && y < self.height {
            self.data[y * self.width + x] = v;
        }
    }
}

pub fn generate_glyph_sdf(font: &Font, c: char, size: u32) -> Grid {
    let scale = Scale::uniform(size as f32);
    let glyph = font
        .glyph(c)
        .scaled(scale)
        .positioned(rusttype::point(0.0, 0.0));

    // Determine bounds
    let bb = glyph.pixel_bounding_box().unwrap_or(rusttype::Rect {
        min: rusttype::point(0, 0),
        max: rusttype::point(0, 0),
    });

    if bb.min.x == bb.max.x || bb.min.y == bb.max.y {
        // Empty glyph (e.g. space)
        return Grid::new(size as usize, size as usize);
    }

    let padding = 10;
    let width = (bb.max.x - bb.min.x + padding * 2) as usize;
    let height = (bb.max.y - bb.min.y + padding * 2) as usize;

    let mut mask = Grid::new(width, height);

    // Draw glyph to mask (values 0.0 to 1.0)
    glyph.draw(|x, y, v| {
        let ox = x as i32 + padding;
        let oy = y as i32 + padding;
        if ox >= 0 && oy >= 0 {
             mask.set(ox as usize, oy as usize, v);
        }
    });

    // Compute SDF (Brute force Distance Transform)
    let mut sdf = Grid::new(width, height);

    // Optimization: Collect "inside" and "outside" points
    // Actually, just iterating all pixels against all pixels is O((W*H)^2).
    // For 64x64 = 4096. 4096^2 = 16M ops. Very fast.
    // For 128x128 = 16384. 16384^2 = 268M ops. Acceptable for startup (0.2s).

    // We treat > 0.5 as Inside, <= 0.5 as Outside.

    let mut pixels = Vec::with_capacity(width * height);
    for y in 0..height {
        for x in 0..width {
            pixels.push((x, y, mask.get(x, y) > 0.5));
        }
    }

    let inside_pixels: Vec<_> = pixels.iter().filter(|&&(_, _, i)| i).collect();
    let outside_pixels: Vec<_> = pixels.iter().filter(|&&(_, _, i)| !i).collect();

    // If completely empty or full, return appropriate constant
    if inside_pixels.is_empty() {
        return sdf; // All zero (ground level)
    }

    for y in 0..height {
        for x in 0..width {
            let is_inside = mask.get(x, y) > 0.5;

            let mut min_dist_sq = f32::MAX;

            // If inside, find closest outside. If outside, find closest inside.
            let targets = if is_inside { &outside_pixels } else { &inside_pixels };

            for &&(tx, ty, _) in targets {
                let dx = x as f32 - tx as f32;
                let dy = y as f32 - ty as f32;
                let d2 = dx*dx + dy*dy;
                if d2 < min_dist_sq {
                    min_dist_sq = d2;
                }
            }

            let dist = min_dist_sq.sqrt();
            // We want positive height for inside (mountains), zero or negative for outside.
            // Let's make outside 0 to -dist, inside 0 to +dist.
            // For terrain, we usually just want the mountain.
            // So let's clamp outside to 0? Or maybe a slight depression?
            // Let's keep it signed for flexibility.
            sdf.set(x, y, if is_inside { dist } else { -dist });
        }
    }

    sdf
}

#[cfg(test)]
mod tests {
    use super::*;
    use rusttype::Font;
    use std::fs;

    #[test]
    fn test_sdf_generation() {
        // Load font
        let font_data = fs::read("assets/Roboto-Regular.ttf").expect("Failed to read font");
        let font = Font::try_from_vec(font_data).expect("Error constructing Font");

        let sdf = generate_glyph_sdf(&font, 'O', 32);

        // Find center roughly
        let cx = sdf.width / 2;
        let cy = sdf.height / 2;

        // 'O' should have positive distance in the ring, negative in the hole and outside.
        // Wait, 'O' has a hole.
        // My simple "inside/outside" logic based on fill > 0.5 handles holes correctly because rusttype handles it.
        // So the hole will be "outside" (<= 0.5).

        // Let's check max value (should be positive)
        let mut max_val = -f32::MAX;
        let mut min_val = f32::MAX;

        for v in &sdf.data {
            if *v > max_val { max_val = *v; }
            if *v < min_val { min_val = *v; }
        }

        println!("SDF Range: {} to {}", min_val, max_val);

        assert!(max_val > 0.0, "Should have positive heights (inside glyph)");
        assert!(min_val < 0.0, "Should have negative depths (outside glyph)");
    }
}
