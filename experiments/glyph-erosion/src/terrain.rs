use macroquad::prelude::*;
use ::rand::Rng;
use rusttype::{Font, Point, Scale};

#[derive(Clone)]
pub struct ErosionParams {
    pub inertia: f32,          // How much previous direction matters (0.0-1.0)
    pub capacity_factor: f32,  // Multiplier for sediment capacity
    pub min_slope: f32,        // Minimum slope to carry sediment
    pub erosion_rate: f32,     // How fast soil is removed
    pub deposition_rate: f32,  // How fast sediment is dropped
    pub evaporation_rate: f32, // How fast water evaporates
    pub gravity: f32,          // Gravity constant
    pub max_steps: usize,      // Max steps per droplet
    pub initial_water: f32,
    pub initial_vel: f32,
}

impl Default for ErosionParams {
    fn default() -> Self {
        Self {
            inertia: 0.1,
            capacity_factor: 8.0,
            min_slope: 0.05,
            erosion_rate: 0.1,
            deposition_rate: 0.1,
            evaporation_rate: 0.02,
            gravity: 4.0,
            max_steps: 64,
            initial_water: 1.0,
            initial_vel: 1.0,
        }
    }
}

pub struct HeightMap {
    pub heights: Vec<f32>,
    pub width: usize,
    pub height: usize,
    pub original_heights: Vec<f32>, // To track changes if needed
}

impl HeightMap {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            heights: vec![0.0; width * height],
            original_heights: vec![0.0; width * height],
            width,
            height,
        }
    }

    pub fn from_text(font_data: &[u8], text: &str, font_size: f32, padding: usize) -> Self {
        let font = Font::try_from_bytes(font_data).expect("Error constructing Font");
        let scale = Scale::uniform(font_size);
        let v_metrics = font.v_metrics(scale);

        let start = Point { x: padding as f32, y: v_metrics.ascent + padding as f32 };
        let glyphs: Vec<_> = font.layout(text, scale, start).collect();

        // Calculate bounds
        let max_x = glyphs.iter()
            .filter_map(|g| g.pixel_bounding_box())
            .map(|bb| bb.max.x)
            .max()
            .unwrap_or(100) as usize + padding;

        let max_y = glyphs.iter()
            .filter_map(|g| g.pixel_bounding_box())
            .map(|bb| bb.max.y)
            .max()
            .unwrap_or(100) as usize + padding;

        let width = max_x;
        let height = max_y;

        let mut map = Self::new(width, height);

        // Rasterize
        for glyph in glyphs {
            if let Some(bb) = glyph.pixel_bounding_box() {
                glyph.draw(|x, y, v| {
                    let gx = x as i32 + bb.min.x;
                    let gy = y as i32 + bb.min.y;

                    if gx >= 0 && gy >= 0 && gx < width as i32 && gy < height as i32 {
                        let idx = (gy as usize) * width + (gx as usize);
                        // v is coverage 0.0 to 1.0
                        // Make letters high plateaus
                        let h = v * 20.0;
                        if map.heights[idx] < h {
                            map.heights[idx] = h;
                        }
                    }
                });
            }
        }

        // Add some base noise
        let mut rng = ::rand::thread_rng();
        for h in map.heights.iter_mut() {
            if *h < 0.1 {
                *h += rng.gen_range(0.0..0.5); // Rough ground
            } else {
                 *h += rng.gen_range(0.0..1.0); // Rough top
            }
        }

        map.original_heights = map.heights.clone();
        map
    }

    fn get_gradient(&self, x: f32, y: f32) -> Vec2 {
        let ix = x as usize;
        let iy = y as usize;

        if ix == 0 || ix >= self.width - 1 || iy == 0 || iy >= self.height - 1 {
            return vec2(0.0, 0.0);
        }

        let idx = iy * self.width + ix;

        // Simple gradient
        let dx = (self.heights[idx + 1] - self.heights[idx - 1]) * 0.5;
        let dy = (self.heights[idx + self.width] - self.heights[idx - self.width]) * 0.5;

        vec2(dx, dy)
    }

    pub fn erode(&mut self, iterations: usize, params: &ErosionParams) {
        let mut rng = ::rand::thread_rng();
        let w = self.width as f32;
        let h = self.height as f32;

        for _ in 0..iterations {
            let start_x = rng.gen_range(1.0..w - 1.0);
            let start_y = rng.gen_range(1.0..h - 1.0);

            let mut pos = vec2(start_x, start_y);
            let mut dir = vec2(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize_or_zero();
            let mut vel = params.initial_vel;
            let mut water = params.initial_water;
            let mut sediment = 0.0;

            for _step in 0..params.max_steps {
                let ix = pos.x as usize;
                let iy = pos.y as usize;
                let idx = iy * self.width + ix;

                // 1. Calculate Gradient
                let grad = self.get_gradient(pos.x, pos.y);

                // 2. Update Direction
                // New direction is combination of inertia and gradient (downhill)
                // Gradient points uphill, so we want -grad
                let mut new_dir = dir * params.inertia - grad * (1.0 - params.inertia);

                // Normalize
                let len = new_dir.length();
                if len > 0.0 {
                    new_dir /= len;
                } else {
                    new_dir = vec2(rng.gen_range(-1.0..1.0), rng.gen_range(-1.0..1.0)).normalize();
                }
                dir = new_dir;

                // 3. Move
                let new_pos = pos + dir;

                // Bounds check
                if new_pos.x < 1.0 || new_pos.x >= w - 1.0 || new_pos.y < 1.0 || new_pos.y >= h - 1.0 {
                    break;
                }

                // 4. Calculate Height Difference
                let old_height = self.heights[idx];
                let new_idx = (new_pos.y as usize) * self.width + (new_pos.x as usize);
                let new_height = self.heights[new_idx];

                let height_diff = new_height - old_height;

                // 5. Carry/Deposit Sediment
                // Capacity is higher if moving fast and lots of water, and slope is steep (height_diff is negative)
                let slope = -height_diff; // Positive if going downhill
                let capacity = slope.max(params.min_slope) * vel * water * params.capacity_factor;

                if sediment > capacity {
                    // Deposit
                    let amount = (sediment - capacity) * params.deposition_rate;
                    self.heights[idx] += amount; // Deposit at old position to fill up
                    sediment -= amount;
                } else {
                    // Erode
                    // Can't erode more than height diff (don't dig a pit deeper than where we are going)
                    let amount = ((capacity - sediment) * params.erosion_rate).min(-height_diff);
                    if amount > 0.0 {
                        self.heights[idx] -= amount;
                        sediment += amount;
                    }
                }

                // 6. Evaporate
                vel = (vel * vel + height_diff * params.gravity).abs().sqrt();
                water *= 1.0 - params.evaporation_rate;

                pos = new_pos;

                if water < 0.01 {
                    break;
                }
            }
        }
    }

    pub fn to_mesh(&self) -> Mesh {
        let mut vertices = Vec::with_capacity(self.width * self.height);
        let mut indices = Vec::with_capacity((self.width - 1) * (self.height - 1) * 6);

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let h = self.heights[idx];

                // Color mapping
                let color = if h < 0.5 {
                    Color::new(0.2, 0.4, 0.8, 1.0) // Water/Base
                } else if h < 2.0 {
                     Color::new(0.76, 0.7, 0.5, 1.0) // Sand
                } else if h < 10.0 {
                    Color::new(0.2, 0.6, 0.2, 1.0) // Grass
                } else if h < 18.0 {
                    Color::new(0.5, 0.5, 0.5, 1.0) // Rock
                } else {
                    Color::new(0.9, 0.9, 0.95, 1.0) // Snow
                };

                vertices.push(Vertex {
                    position: vec3(x as f32, h, y as f32),
                    uv: vec2(x as f32 / self.width as f32, y as f32 / self.height as f32),
                    color: color.into(),
                    normal: vec4(0.0, 1.0, 0.0, 0.0), // Simplified normal
                });
            }
        }

        for y in 0..self.height - 1 {
            for x in 0..self.width - 1 {
                let i = (y * self.width + x) as u16;
                let next_row = ((y + 1) * self.width) as u16;

                // Tri 1
                indices.push(i);
                indices.push(i + 1);
                indices.push(next_row + x as u16);

                // Tri 2
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

    #[test]
    fn test_heightmap_creation() {
        let map = HeightMap::new(10, 10);
        assert_eq!(map.width, 10);
        assert_eq!(map.height, 10);
        assert_eq!(map.heights.len(), 100);
    }
}
