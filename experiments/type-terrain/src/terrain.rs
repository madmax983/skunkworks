use crate::sdf::generate_sdf;
use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};
use rusttype::{Font, Point, PositionedGlyph, Scale};

pub struct FontTerrain {
    pub heightmap: Vec<f32>,
    pub width: usize,
    pub height: usize,
    pub max_height: f32,
}

impl FontTerrain {
    pub fn new(font_data: &[u8], text: &str, font_size: f32) -> Self {
        let font = Font::try_from_bytes(font_data).expect("Error constructing Font");
        let scale = Scale::uniform(font_size);
        let v_metrics = font.v_metrics(scale);

        let start = Point {
            x: 0.0,
            y: v_metrics.ascent,
        };
        let glyphs: Vec<PositionedGlyph> = font.layout(text, scale, start).collect();

        if glyphs.is_empty() {
             return Self { heightmap: vec![], width: 0, height: 0, max_height: 0.0 };
        }

        // Calculate bounds
        let min_x = glyphs.iter().filter_map(|g| g.pixel_bounding_box()).map(|bb| bb.min.x).min().unwrap_or(0);
        let max_x = glyphs.iter().filter_map(|g| g.pixel_bounding_box()).map(|bb| bb.max.x).max().unwrap_or(100);
        let min_y = glyphs.iter().filter_map(|g| g.pixel_bounding_box()).map(|bb| bb.min.y).min().unwrap_or(0);
        let max_y = glyphs.iter().filter_map(|g| g.pixel_bounding_box()).map(|bb| bb.max.y).max().unwrap_or(100);

        let padding = 20;
        let width = (max_x - min_x + padding * 2) as usize;
        let height = (max_y - min_y + padding * 2) as usize;

        // binary grid for SDF
        let mut grid = vec![false; width * height];

        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let gx = (x as i32 + bb.min.x - min_x + padding) as usize;
                    let gy = (y as i32 + bb.min.y - min_y + padding) as usize;
                    if gx < width && gy < height {
                        if v > 0.3 { // Threshold
                            grid[gy * width + gx] = true;
                        }
                    }
                });
            }
        }

        // Generate SDF
        let mut sdf = generate_sdf(&grid, width, height);

        // Apply Noise
        let perlin = Perlin::new(1);
        let mut max_h = 0.0;

        for y in 0..height {
            for x in 0..width {
                let idx = y * width + x;
                let dist = sdf[idx];

                if dist > 0.0 {
                    let nx = x as f64 * 0.1;
                    let ny = y as f64 * 0.1;
                    let noise_val = perlin.get([nx, ny]) as f32;

                    // Height is base distance + noise
                    // Scale distance to make mountains taller
                    let height_val = dist * 1.5 + noise_val * 2.0;
                    sdf[idx] = height_val.max(0.0);
                } else {
                    sdf[idx] = 0.0;
                }

                if sdf[idx] > max_h {
                    max_h = sdf[idx];
                }
            }
        }

        Self {
            heightmap: sdf,
            width,
            height,
            max_height: max_h,
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
                let normalized_height = if self.max_height > 0.0 { height / self.max_height } else { 0.0 };

                // Color based on height
                let color = if height <= 0.1 {
                     Color::new(0.0, 0.0, 0.0, 1.0) // Abyss
                } else if normalized_height < 0.2 {
                    Color::new(0.2, 0.8, 0.2, 1.0) // Grass
                } else if normalized_height < 0.5 {
                    Color::new(0.5, 0.5, 0.5, 1.0) // Rock
                } else {
                    Color::new(1.0, 1.0, 1.0, 1.0) // Snow
                };

                // Emphasize outlines
                let final_color = if height > 0.0 && height < 1.0 {
                    Color::new(1.0, 0.0, 0.0, 1.0) // Red Coastline
                } else {
                    color
                };

                vertices.push(Vertex {
                    position: vec3(x as f32, height, y as f32),
                    uv: vec2(x as f32 / w as f32, y as f32 / h as f32),
                    color: final_color.into(),
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
                indices.push(next_row + x as u16);
                indices.push(i + 1);

                // Triangle 2
                indices.push(i + 1);
                indices.push(next_row + x as u16);
                indices.push(next_row + x as u16 + 1);
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
        // Try to find the font file in a few common locations
        let possible_paths = [
            "assets/font.ttf",
            "experiments/type-terrain/assets/font.ttf",
            "../assets/font.ttf",
        ];

        let mut font_data = None;
        for path in possible_paths {
            if let Ok(bytes) = fs::read(path) {
                font_data = Some(bytes);
                break;
            }
        }

        if let Some(data) = font_data {
             let terrain = FontTerrain::new(&data, "A", 50.0);
             assert!(!terrain.heightmap.is_empty());
             assert!(terrain.width > 0);
        } else {
            // Skip test if font not found
            println!("Font not found, skipping test");
        }
    }
}
