use rusttype::{Font, Scale, Point, PositionedGlyph};
use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};

pub struct FontTerrain {
    pub heightmap: Vec<f32>,
    pub width: usize,
    pub height: usize,
}

impl FontTerrain {
    pub fn new(font_data: &[u8], text: &str, font_size: f32) -> Self {
        let font = Font::try_from_bytes(font_data).expect("Error constructing Font");
        let scale = Scale::uniform(font_size);
        let v_metrics = font.v_metrics(scale);

        // Layout the glyphs to find bounds
        let start = Point { x: 0.0, y: v_metrics.ascent };
        let glyphs: Vec<PositionedGlyph> = font.layout(text, scale, start).collect();

        if glyphs.is_empty() {
             return Self {
                heightmap: vec![],
                width: 0,
                height: 0,
            };
        }

        // Calculate bounds
        let min_x = glyphs.iter().filter_map(|g| g.pixel_bounding_box()).map(|bb| bb.min.x).min().unwrap_or(0);
        let max_x = glyphs.iter().filter_map(|g| g.pixel_bounding_box()).map(|bb| bb.max.x).max().unwrap_or(100);
        let min_y = glyphs.iter().filter_map(|g| g.pixel_bounding_box()).map(|bb| bb.min.y).min().unwrap_or(0);
        let max_y = glyphs.iter().filter_map(|g| g.pixel_bounding_box()).map(|bb| bb.max.y).max().unwrap_or(100);

        let width = (max_x - min_x + 20) as usize; // Padding
        let height = (max_y - min_y + 20) as usize;
        let mut heightmap = vec![0.0; width * height];

        // Rasterize
        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let gx = (x as i32 + bb.min.x - min_x + 10) as usize;
                    let gy = (y as i32 + bb.min.y - min_y + 10) as usize;
                    if gx < width && gy < height {
                        let idx = gy * width + gx;
                        // Use the coverage value (v) as height
                        if heightmap[idx] < v {
                            heightmap[idx] = v;
                        }
                    }
                });
            }
        }

        // Apply a simple blur/spread to make it terrain-like
        // A simple box blur
        let mut blurred = heightmap.clone();
        for y in 1..height-1 {
            for x in 1..width-1 {
                let idx = y * width + x;
                if heightmap[idx] > 0.0 {
                    blurred[idx] = heightmap[idx];
                } else {
                    // Spread neighbors
                    let mut sum = 0.0;
                    let mut count = 0.0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let val = heightmap[((y as i32 + dy) as usize) * width + ((x as i32 + dx) as usize)];
                            if val > 0.0 {
                                sum += val;
                                count += 1.0;
                            }
                        }
                    }
                    if count > 0.0 {
                        blurred[idx] = sum / count * 0.5; // Decay
                    }
                }
            }
        }
        heightmap = blurred;

        // Apply Noise
        let perlin = Perlin::new(1);
        for y in 0..height {
            for x in 0..width {
                let nx = x as f64 * 0.1;
                let ny = y as f64 * 0.1;
                let n = perlin.get([nx, ny]) as f32;
                let idx = y * width + x;

                // Base terrain + Glyph mountains
                let base_height = (n + 1.0) * 0.2; // 0.0 to 0.4
                heightmap[idx] = heightmap[idx] * 2.0 + base_height;
            }
        }

        Self {
            heightmap,
            width,
            height,
        }
    }

    pub fn to_mesh(&self) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let w = self.width;
        let h = self.height;

        for y in 0..h {
            for x in 0..w {
                let height = self.heightmap[y * w + x];

                // Color based on height
                let color = if height > 0.5 {
                    Color::new(0.8, 0.8, 0.9, 1.0) // Snow
                } else if height > 0.2 {
                    Color::new(0.4, 0.6, 0.4, 1.0) // Grass
                } else {
                    Color::new(0.3, 0.3, 0.3, 1.0) // Rock/Dirt
                };

                vertices.push(Vertex {
                    position: vec3(x as f32, height * 10.0, y as f32),
                    uv: vec2(x as f32 / w as f32, y as f32 / h as f32),
                    color: color.into(),
                    normal: vec4(0.0, 1.0, 0.0, 0.0),
                });
            }
        }

        for y in 0..h - 1 {
            for x in 0..w - 1 {
                let i = (y * w + x) as u16;
                let next_row = ((y + 1) * w) as u16;

                // Triangle 1
                indices.push(i);
                indices.push(i + 1);
                indices.push(next_row + x as u16);

                // Triangle 2
                indices.push(i + 1);
                indices.push(next_row + x as u16 + 1);
                indices.push(next_row + x as u16);
            }
        }

        Mesh {
            vertices,
            indices,
            texture: None,
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::fs;

    #[test]
    fn test_font_loading_and_heightmap_generation() {
        // Load the font file.
        let path = "assets/font.ttf";
        let font_data = fs::read(path).expect(&format!("Failed to read font file at {}", path));

        let terrain = FontTerrain::new(&font_data, "A", 50.0);

        assert!(!terrain.heightmap.is_empty(), "Heightmap should not be empty");
        assert!(terrain.width > 0, "Width should be > 0");
        assert!(terrain.height > 0, "Height should be > 0");

        // Check if there is some height
        let max_h = terrain.heightmap.iter().cloned().fold(0./0., f32::max);
        assert!(max_h > 0.0, "Max height should be > 0");
    }

    #[test]
    fn test_to_mesh() {
        let path = "assets/font.ttf";
        if let Ok(font_data) = fs::read(path) {
            let terrain = FontTerrain::new(&font_data, "A", 50.0);
            let mesh = terrain.to_mesh();
            assert!(!mesh.vertices.is_empty());
            assert!(!mesh.indices.is_empty());
        }
    }
}
