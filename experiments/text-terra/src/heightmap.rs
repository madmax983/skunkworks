use noise::{NoiseFn, Perlin};
use rusttype::{point, Font, Scale, PositionedGlyph};

pub struct HeightMap {
    pub width: u32,
    pub height: u32,
    pub data: Vec<f32>,
}

impl HeightMap {
    pub fn new(width: u32, height: u32) -> Self {
        Self {
            width,
            height,
            data: vec![0.0; (width * height) as usize],
        }
    }

    pub fn get(&self, x: u32, y: u32) -> f32 {
        if x >= self.width || y >= self.height {
            return 0.0;
        }
        self.data[(y * self.width + x) as usize]
    }

    pub fn set(&mut self, x: u32, y: u32, val: f32) {
        if x >= self.width || y >= self.height {
            return;
        }
        self.data[(y * self.width + x) as usize] = val;
    }
}

pub fn generate_text_heightmap(
    text: &str,
    font_data: &[u8],
    width: u32,
    height: u32,
) -> HeightMap {
    let mut map = HeightMap::new(width, height);

    // 1. Generate Noise Terrain
    let perlin = Perlin::new(1);
    let scale = 0.05; // Zoom level for noise

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 * scale;
            let ny = y as f64 * scale;
            let noise_val = perlin.get([nx, ny]);
            // Normalize roughly to 0.0 - 5.0
            let terrain_height = (noise_val + 1.0) * 2.5;
            map.set(x, y, terrain_height as f32);
        }
    }

    // 2. Rasterize Text
    let font = Font::try_from_bytes(font_data).expect("Error constructing Font");

    // Scale text to fit roughly in the middle
    // Let's assume the text should take up about 80% of width
    let font_scale_val = (width as f32) * 0.25; // Heuristic
    let scale = Scale::uniform(font_scale_val);
    let v_metrics = font.v_metrics(scale);

    let offset = point(width as f32 * 0.1, height as f32 / 2.0 + v_metrics.ascent / 2.0);

    let glyphs: Vec<PositionedGlyph> = font.layout(text, scale, offset).collect();

    for glyph in glyphs {
        if let Some(bb) = glyph.pixel_bounding_box() {
            glyph.draw(|x, y, v| {
                let gx = x as i32 + bb.min.x;
                let gy = y as i32 + bb.min.y;

                if gx >= 0 && gx < width as i32 && gy >= 0 && gy < height as i32 {
                    let current_h = map.get(gx as u32, gy as u32);
                    // Text should be significantly higher, e.g., +20.0 * coverage
                    let text_h = v * 20.0;
                    // We blend or stack? Let's stack.
                    map.set(gx as u32, gy as u32, current_h + text_h);
                }
            });
        }
    }

    map
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_heightmap_generation() {
        // Use include_bytes! to ensure the font is found relative to this file
        let font_data = include_bytes!("../assets/DejaVuSans.ttf");

        let width = 100;
        let height = 100;
        let map = generate_text_heightmap("A", font_data, width, height);

        assert_eq!(map.width, width);
        assert_eq!(map.height, height);

        // Check if we have some variation (noise)
        let mut min = f32::MAX;
        let mut max = f32::MIN;
        let mut has_high_peak = false; // Text should create a high peak

        for val in &map.data {
            if *val < min { min = *val; }
            if *val > max { max = *val; }
            if *val > 10.0 { has_high_peak = true; }
        }

        assert!(max > min, "Map should have variation");
        assert!(has_high_peak, "Should have text peaks (value > 10.0)");
    }
}
