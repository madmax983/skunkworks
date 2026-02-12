use rusttype::{Font, Scale, PositionedGlyph, point};
use noise::{NoiseFn, Perlin};

pub struct HeightMap {
    pub width: usize,
    pub height: usize,
    pub data: Vec<f32>,
}

impl HeightMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            data: vec![0.0; width * height],
        }
    }

    pub fn from_text(text: &str, font: &Font, font_size: f32, width: usize, height: usize) -> Self {
        let mut map = Self::new(width, height);

        // 1. Add base terrain noise
        let perlin = Perlin::new(42);
        for y in 0..height {
            for x in 0..width {
                let nx = x as f64 * 0.02;
                let ny = y as f64 * 0.02;
                let n = perlin.get([nx, ny]) as f32; // [-1, 1]

                let idx = y * width + x;
                // Base terrain: gentle rolling hills, height 0 to 5
                map.data[idx] = (n + 1.0) * 2.5;
            }
        }

        // 2. Rasterize text onto the map as mountains
        let scale = Scale::uniform(font_size);
        let v_metrics = font.v_metrics(scale);

        // Center text roughly in the middle of the map
        // We'll just start at some offset for now.
        // A better approach would be to calculate text width first, but let's keep it simple.
        let start_x = 50.0;
        let start_y = height as f32 / 2.0 + v_metrics.descent;

        let glyphs: Vec<PositionedGlyph> = font.layout(text, scale, point(start_x, start_y)).collect();

        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let px = x as i32 + bb.min.x;
                    let py = y as i32 + bb.min.y;
                    if px >= 0 && px < width as i32 && py >= 0 && py < height as i32 {
                        let idx = (py as usize) * width + (px as usize);
                        // v is coverage [0.0, 1.0]
                        // We map this to massive height. Let's say max height is 30.0
                        // We add to existing terrain to make it seamless
                        map.data[idx] += v * 30.0;
                    }
                });
            }
        }

        map
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        if x < self.width && y < self.height {
            self.data[y * self.width + x]
        } else {
            0.0
        }
    }
}
