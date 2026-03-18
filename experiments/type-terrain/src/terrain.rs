use ab_glyph::{Font, FontRef, PxScale, ScaleFont};
use glyph_brush_layout::{GlyphPositioner, Layout, SectionGeometry, SectionText};
use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};

pub struct FontTerrain {
    pub heightmap: Vec<f32>,
    pub width: usize,
    pub height: usize,
    pub sediment_map: Vec<f32>,
}

impl FontTerrain {
    pub fn new(font_data: &[u8], text: &str, font_size: f32) -> Self {
        let font: FontRef<'_> =
            FontRef::try_from_slice(font_data).expect("Error constructing Font");
        let scale = PxScale::from(font_size);
        let scaled_font = font.as_scaled(scale);

        let layout = Layout::default();
        let glyphs = layout.calculate_glyphs(
            &[&font],
            &SectionGeometry {
                screen_position: (0.0, scaled_font.ascent()),
                bounds: (f32::INFINITY, f32::INFINITY),
            },
            &[SectionText {
                text,
                scale,
                font_id: glyph_brush_layout::FontId(0),
            }],
        );

        if glyphs.is_empty() {
            return Self {
                heightmap: vec![],
                width: 0,
                height: 0,
                sediment_map: vec![],
            };
        }

        let mut min_x = f32::INFINITY;
        let mut max_x = f32::NEG_INFINITY;
        let mut min_y = f32::INFINITY;
        let mut max_y = f32::NEG_INFINITY;

        for g in &glyphs {
            let bb = font.glyph_bounds(&g.glyph);
            min_x = min_x.min(bb.min.x);
            max_x = max_x.max(bb.max.x);
            min_y = min_y.min(bb.min.y);
            max_y = max_y.max(bb.max.y);
        }

        let padding = 60;
        let width = (max_x.ceil() - min_x.floor() + padding as f32 * 2.0) as usize;
        let height = (max_y.ceil() - min_y.floor() + padding as f32 * 2.0) as usize;
        let mut heightmap = vec![0.0; width * height];

        for g in glyphs {
            if let Some(outlined) = font.outline_glyph(g.glyph) {
                let px_bounds = outlined.px_bounds();
                outlined.draw(|x, y, v| {
                    let gx = (x as i32 + px_bounds.min.x as i32 - min_x as i32 + padding as i32)
                        as usize;
                    let gy = (y as i32 + px_bounds.min.y as i32 - min_y as i32 + padding as i32)
                        as usize;
                    if gx < width && gy < height {
                        let idx = gy * width + gx;
                        if heightmap[idx] < v {
                            heightmap[idx] = v;
                        }
                    }
                });
            }
        }

        for _ in 0..5 {
            let mut next_map = heightmap.clone();
            for y in 1..height - 1 {
                for x in 1..width - 1 {
                    let mut sum = 0.0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            sum += heightmap
                                [((y as i32 + dy) as usize) * width + ((x as i32 + dx) as usize)];
                        }
                    }
                    next_map[y * width + x] = sum / 9.0;
                }
            }
            heightmap = next_map;
        }

        let perlin = Perlin::new(1);
        for y in 0..height {
            for x in 0..width {
                let nx = x as f64 * 0.05;
                let ny = y as f64 * 0.05;
                let n = perlin.get([nx, ny]) as f32;

                let idx = y * width + x;
                let base_noise = (n + 1.0) * 0.1;
                let mountain_noise = n * 0.2 * heightmap[idx];

                heightmap[idx] = heightmap[idx] * 4.0 + base_noise + mountain_noise;
            }
        }

        Self {
            heightmap,
            width,
            height,
            sediment_map: vec![0.0; width * height],
        }
    }

    pub fn erode(&mut self, iterations: usize) {
        let w = self.width as i32;
        let h = self.height as i32;

        for _ in 0..iterations {
            let mut x = rand::gen_range(1.0, (w - 2) as f32);
            let mut y = rand::gen_range(1.0, (h - 2) as f32);

            let mut dir_x: f32 = 0.0;
            let mut dir_y: f32 = 0.0;
            let mut speed: f32 = 1.0;
            let mut water: f32 = 1.0;
            let mut sediment: f32 = 0.0;

            let inertia = 0.05;
            let capacity_factor = 4.0;
            let deposition_rate = 0.3;
            let erosion_rate = 0.3;
            let evaporation_rate = 0.02;
            let gravity = 4.0;
            let min_slope = 0.0001;
            let max_steps = 30;

            for _step in 0..max_steps {
                let ix = x as i32;
                let iy = y as i32;
                let idx = (iy * w + ix) as usize;

                let height = self.heightmap[idx];
                let h_l = self.heightmap[idx - 1];
                let h_r = self.heightmap[idx + 1];
                let h_u = self.heightmap[idx - w as usize];
                let h_d = self.heightmap[idx + w as usize];

                let gx = h_l - h_r;
                let gy = h_u - h_d;

                let _len = (gx * gx + gy * gy).sqrt().max(min_slope);

                dir_x = dir_x * inertia - gx * (1.0 - inertia);
                dir_y = dir_y * inertia - gy * (1.0 - inertia);

                let dir_len = (dir_x * dir_x + dir_y * dir_y).sqrt().max(min_slope);
                dir_x /= dir_len;
                dir_y /= dir_len;

                x += dir_x;
                y += dir_y;

                if x < 1.0 || x >= (w - 1) as f32 || y < 1.0 || y >= (h - 1) as f32 {
                    break;
                }

                let new_idx = (y as i32 * w + x as i32) as usize;
                let new_height = self.heightmap[new_idx];
                let diff = height - new_height;

                let capacity = diff.max(min_slope) * speed * water * capacity_factor;

                if diff > 0.0 {
                    if sediment > capacity {
                        let amount = (sediment - capacity) * deposition_rate;
                        self.heightmap[idx] += amount;
                        self.sediment_map[idx] += amount;
                        sediment -= amount;
                    } else {
                        let amount = (capacity - sediment) * erosion_rate;
                        let amount = amount.min(diff);
                        self.heightmap[idx] -= amount;
                        sediment += amount;
                    }
                } else {
                    let amount = sediment.min(-diff);
                    self.heightmap[idx] += amount;
                    self.sediment_map[idx] += amount;
                    sediment -= amount;
                }

                speed = (speed * speed + diff * gravity).sqrt();
                water *= 1.0 - evaporation_rate;

                if water < 0.01 {
                    break;
                }
            }
        }
    }

    pub fn calculate_normals(&self) -> Vec<Vec3> {
        let w = self.width;
        let h = self.height;
        let mut normals = vec![vec3(0.0, 1.0, 0.0); w * h];

        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let idx = y * w + x;
                let h_l = self.heightmap[idx - 1];
                let h_r = self.heightmap[idx + 1];
                let h_u = self.heightmap[idx - w];
                let h_d = self.heightmap[idx + w];

                let scale = 10.0;
                let dx = (h_r - h_l) * scale;
                let dz = (h_d - h_u) * scale;

                let normal = vec3(-dx, 2.0, -dz).normalize();
                normals[idx] = normal;
            }
        }
        normals
    }

    pub fn to_mesh(&self) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        let w = self.width;
        let h = self.height;
        let normals = self.calculate_normals();

        for y in 0..h {
            for x in 0..w {
                let idx = y * w + x;
                let height = self.heightmap[idx];
                let sediment = self.sediment_map[idx];
                let normal = normals[idx];

                let slope = normal.dot(vec3(0.0, 1.0, 0.0));

                let mut color = if height < 0.2 {
                    Color::new(0.2, 0.3, 0.4, 1.0)
                } else if height < 0.5 {
                    Color::new(0.2, 0.5, 0.2, 1.0)
                } else if height < 1.5 {
                    Color::new(0.5, 0.5, 0.5, 1.0)
                } else {
                    Color::new(0.9, 0.9, 1.0, 1.0)
                };

                if slope < 0.7 {
                    color = Color::new(0.4, 0.4, 0.4, 1.0);
                }

                if sediment > 0.05 {
                    color = Color::new(0.6, 0.5, 0.3, 1.0);
                }

                vertices.push(Vertex {
                    position: vec3(x as f32, height * 10.0, y as f32),
                    uv: vec2(x as f32 / w as f32, y as f32 / h as f32),
                    color: color.into(),
                    normal: vec4(normal.x, normal.y, normal.z, 0.0),
                });
            }
        }

        for y in 0..h - 1 {
            for x in 0..w - 1 {
                let i = (y * w + x) as u16;
                let next_row = ((y + 1) * w) as u16;

                indices.push(i);
                indices.push(i + 1);
                indices.push(next_row + x as u16);

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
        let path = "assets/font.ttf";
        let font_data = match fs::read(path) {
            Ok(data) => data,
            Err(_) => return,
        };

        let terrain = FontTerrain::new(&font_data, "A", 50.0);

        assert!(terrain.width > 0);
        assert!(terrain.height > 0);
    }

    #[test]
    fn test_erosion_runs() {
        let path = "assets/font.ttf";
        let font_data = match fs::read(path) {
            Ok(data) => data,
            Err(_) => return,
        };

        let mut terrain = FontTerrain::new(&font_data, "Test", 50.0);
        let initial_height = terrain.heightmap.iter().sum::<f32>();

        terrain.erode(100);

        let final_height = terrain.heightmap.iter().sum::<f32>();

        assert!(initial_height != final_height || initial_height == 0.0);
    }
}
