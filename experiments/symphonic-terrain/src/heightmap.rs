use noise::{NoiseFn, Perlin};
use ab_glyph::{FontRef, Font, ScaleFont, PxScale};
use glyph_brush_layout::{SectionGeometry, GlyphPositioner, Layout, SectionText};

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

pub fn generate_text_heightmap(text: &str, font_data: &[u8], width: u32, height: u32) -> HeightMap {
    let mut map = HeightMap::new(width, height);

    let perlin = Perlin::new(1);
    let noise_scale = 0.05;

    for y in 0..height {
        for x in 0..width {
            let nx = x as f64 * noise_scale;
            let ny = y as f64 * noise_scale;
            let noise_val = perlin.get([nx, ny]);
            let terrain_height = (noise_val + 1.0) * 2.5;
            map.set(x, y, terrain_height as f32);
        }
    }

    let font = match FontRef::try_from_slice(font_data) {
        Ok(f) => f,
        Err(_) => return map,
    };

    let font_scale_val = (width as f32) * 0.25;
    let px_scale = PxScale::from(font_scale_val);
    let scaled_font = font.as_scaled(px_scale);

    let layout = Layout::default();
    let glyphs = layout.calculate_glyphs(
        &[&font],
        &SectionGeometry {
            screen_position: (width as f32 * 0.1, height as f32 / 2.0 + scaled_font.ascent() / 2.0),
            bounds: (f32::INFINITY, f32::INFINITY),
        },
        &[SectionText {
            text,
            scale: px_scale,
            font_id: glyph_brush_layout::FontId(0),
        }],
    );

    for g in glyphs {
        if let Some(outlined) = font.outline_glyph(g.glyph) {
            let bb = outlined.px_bounds();
            outlined.draw(|x, y, v| {
                let gx = x as i32 + bb.min.x as i32;
                let gy = y as i32 + bb.min.y as i32;

                if gx >= 0 && gx < width as i32 && gy >= 0 && gy < height as i32 {
                    let current_h = map.get(gx as u32, gy as u32);
                    let text_h = v * 20.0;
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
        let font_data = include_bytes!("../assets/DejaVuSans.ttf");

        let width = 100;
        let height = 100;
        let map = generate_text_heightmap("A", font_data, width, height);

        assert_eq!(map.width, width);
        assert_eq!(map.height, height);

        let mut min = f32::MAX;
        let mut max = f32::MIN;
        let mut has_high_peak = false;

        for val in &map.data {
            if *val < min {
                min = *val;
            }
            if *val > max {
                max = *val;
            }
            if *val > 10.0 {
                has_high_peak = true;
            }
        }

        assert!(max > min, "Map should have variation");
        assert!(has_high_peak, "Should have text peaks (value > 10.0)");
    }
}
