use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};
use rusttype::{Font, Point, PositionedGlyph, Scale};

pub struct FontTerrain {
    pub heightmap: Vec<f32>,
    pub width: usize,
    pub height: usize,
    pub sediment_map: Vec<f32>, // Track sediment for coloring
}

impl FontTerrain {
    pub fn new(font_data: &[u8], text: &str, font_size: f32) -> Self {
        let font = Font::try_from_bytes(font_data).expect("Error constructing Font");
        let scale = Scale::uniform(font_size);
        let v_metrics = font.v_metrics(scale);

        // Layout the glyphs to find bounds
        let start = Point {
            x: 0.0,
            y: v_metrics.ascent,
        };
        let glyphs: Vec<PositionedGlyph> = font.layout(text, scale, start).collect();

        if glyphs.is_empty() {
            return Self {
                heightmap: vec![],
                width: 0,
                height: 0,
                sediment_map: vec![],
            };
        }

        // Calculate bounds with significant padding for terrain context
        let min_x = glyphs
            .iter()
            .filter_map(|g| g.pixel_bounding_box())
            .map(|bb| bb.min.x)
            .min()
            .unwrap_or(0);
        let max_x = glyphs
            .iter()
            .filter_map(|g| g.pixel_bounding_box())
            .map(|bb| bb.max.x)
            .max()
            .unwrap_or(100);
        let min_y = glyphs
            .iter()
            .filter_map(|g| g.pixel_bounding_box())
            .map(|bb| bb.min.y)
            .min()
            .unwrap_or(0);
        let max_y = glyphs
            .iter()
            .filter_map(|g| g.pixel_bounding_box())
            .map(|bb| bb.max.y)
            .max()
            .unwrap_or(100);

        let padding = 60;
        let width = (max_x - min_x + padding * 2) as usize;
        let height = (max_y - min_y + padding * 2) as usize;
        let mut heightmap = vec![0.0; width * height];

        // Rasterize with "SDF-like" blurring
        // First pass: Direct rasterization
        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let gx = (x as i32 + bb.min.x - min_x + padding as i32) as usize;
                    let gy = (y as i32 + bb.min.y - min_y + padding as i32) as usize;
                    if gx < width && gy < height {
                        let idx = gy * width + gx;
                        // Initial height based on coverage
                        if heightmap[idx] < v {
                            heightmap[idx] = v;
                        }
                    }
                });
            }
        }

        // Apply extensive blurring to create slopes
        // 5 passes of box blur to approximate Gaussian
        for _ in 0..5 {
            let mut next_map = heightmap.clone();
            for y in 1..height - 1 {
                for x in 1..width - 1 {
                    let mut sum = 0.0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            sum += heightmap[((y as i32 + dy) as usize) * width + ((x as i32 + dx) as usize)];
                        }
                    }
                    next_map[y * width + x] = sum / 9.0;
                }
            }
            heightmap = next_map;
        }

        // Apply Perlin Noise to everything
        let perlin = Perlin::new(1);
        for y in 0..height {
            for x in 0..width {
                let nx = x as f64 * 0.05;
                let ny = y as f64 * 0.05;
                let n = perlin.get([nx, ny]) as f32; // -1 to 1

                let idx = y * width + x;

                // Base terrain noise
                let base_noise = (n + 1.0) * 0.1;

                // Add noise to mountains, scaled by height so peaks are rugged but valleys are smoother
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
            // Spawn a droplet at a random location
            let mut x = rand::gen_range(1.0, (w - 2) as f32);
            let mut y = rand::gen_range(1.0, (h - 2) as f32);

            let mut dir_x: f32 = 0.0;
            let mut dir_y: f32 = 0.0;
            let mut speed: f32 = 1.0;
            let mut water: f32 = 1.0;
            let mut sediment: f32 = 0.0;

            // Parameters
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

                // Calculate gradient
                // Simple gradient based on neighbors
                let height = self.heightmap[idx];
                let h_l = self.heightmap[idx - 1];
                let h_r = self.heightmap[idx + 1];
                let h_u = self.heightmap[idx - w as usize];
                let h_d = self.heightmap[idx + w as usize];

                let gx = h_l - h_r;
                let gy = h_u - h_d;

                // Normalize gradient
                let _len = (gx * gx + gy * gy).sqrt().max(min_slope);

                // Update direction with inertia
                dir_x = dir_x * inertia - gx * (1.0 - inertia); // Move DOWNHILL (negative gradient)
                dir_y = dir_y * inertia - gy * (1.0 - inertia);

                // Normalize direction
                let dir_len = (dir_x * dir_x + dir_y * dir_y).sqrt().max(min_slope);
                dir_x /= dir_len;
                dir_y /= dir_len;

                // Update position
                x += dir_x;
                y += dir_y;

                // Check bounds
                if x < 1.0 || x >= (w - 1) as f32 || y < 1.0 || y >= (h - 1) as f32 {
                    break;
                }

                // New height at new position
                let new_idx = (y as i32 * w + x as i32) as usize;
                let new_height = self.heightmap[new_idx];
                let diff = height - new_height; // Positive if went downhill

                // Calculate carrying capacity
                let capacity = diff.max(min_slope) * speed * water * capacity_factor;

                // Erosion or Deposition
                if diff > 0.0 {
                    // Moving downhill
                    if sediment > capacity {
                        // Deposit
                        let amount = (sediment - capacity) * deposition_rate;
                        self.heightmap[idx] += amount; // Deposit at OLD position usually, or new? usually old or interpolated. Let's do simple: deposit at current cell before moving completely
                        self.sediment_map[idx] += amount;
                        sediment -= amount;
                    } else {
                        // Erode
                        let amount = (capacity - sediment) * erosion_rate;
                        let amount = amount.min(diff); // Don't dig a hole deeper than the move
                        self.heightmap[idx] -= amount;
                        sediment += amount;
                    }
                } else {
                    // Moving uphill (can happen due to inertia) -> Deposit everything
                    let amount = sediment.min(-diff); // Fill the hole
                     self.heightmap[idx] += amount;
                     self.sediment_map[idx] += amount;
                     sediment -= amount;
                }

                // Update speed and water
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

                // Tangents
                // T_x = (2, 0, h_r - h_l) (scaling x by 1 unit step)
                // T_z = (0, 2, h_d - h_u)
                // Normal = T_z x T_x

                let _tx = vec3(2.0, h_r - h_l, 0.0); // Wait, y is up in my mesh generator? Yes.
                // In my mesh generator: position = (x, height, y) -> so y is UP.
                // So terrain grid is XZ plane.

                // Vector along X axis: (2, h_r - h_l, 0)
                // Vector along Z axis: (0, h_d - h_u, 2)

                // Normal = Cross product
                // | i  j  k |
                // | 0  dy 2 |
                // | 2  dx 0 |

                // i: 0 - 2*dx = -2(h_r - h_l)
                // j: 4 - 0    = 4 (This component is Y-up)
                // k: 0 - 2*dy = -2(h_d - h_u)

                let scale = 10.0; // Height scaling factor from mesh gen
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

                // Calculate slope factor (dot product with up vector)
                let slope = normal.dot(vec3(0.0, 1.0, 0.0));

                // Color mapping
                // Deep water / Valley floor
                let mut color = if height < 0.2 {
                    Color::new(0.2, 0.3, 0.4, 1.0) // Dark blueish
                } else if height < 0.5 {
                     Color::new(0.2, 0.5, 0.2, 1.0) // Grass
                } else if height < 1.5 {
                     Color::new(0.5, 0.5, 0.5, 1.0) // Rock
                } else {
                     Color::new(0.9, 0.9, 1.0, 1.0) // Snow
                };

                // Slope influence: steep = rock
                if slope < 0.7 {
                    color = Color::new(0.4, 0.4, 0.4, 1.0);
                }

                // Sediment influence: deposited soil = sand/brown
                if sediment > 0.05 {
                    color = Color::new(0.6, 0.5, 0.3, 1.0);
                }

                vertices.push(Vertex {
                    position: vec3(x as f32, height * 10.0, y as f32),
                    uv: vec2(x as f32 / w as f32, y as f32 / h as f32),
                    color: color.into(),
                    normal: vec4(normal.x, normal.y, normal.z, 0.0), // macroquad Vertex normal is Vec4? or Vec3?
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
        let path = "assets/font.ttf";
        // Create dummy font data if file doesn't exist (CI/CD without assets)
        // But here we rely on the file existing as per previous exploration
        let font_data = match fs::read(path) {
            Ok(data) => data,
            Err(_) => return, // Skip test if asset missing
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

        // Erosion redistributes mass, but some might be lost to evaporation/boundary
        // Just check it didn't crash and potentially changed
        assert!(initial_height != final_height || initial_height == 0.0);
    }
}
