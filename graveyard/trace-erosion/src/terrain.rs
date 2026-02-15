use crate::graph::CallGraph;
use macroquad::prelude::*;

pub const GRID_SIZE: usize = 128;
pub const WORLD_SIZE: f32 = 400.0; // World ranges from -200 to 200

pub struct Terrain {
    pub heightmap: Vec<f32>,
    pub width: usize,
    pub height: usize,
}

impl Terrain {
    pub fn new() -> Self {
        let size = GRID_SIZE * GRID_SIZE;
        Self {
            heightmap: vec![0.0; size],
            width: GRID_SIZE,
            height: GRID_SIZE,
        }
    }

    pub fn uplift(&mut self, _graph: &CallGraph) {
        // Initial terrain shape: A cone centered at 0,0 (Root)
        for y in 0..self.height {
            for x in 0..self.width {
                let world_pos = self.grid_to_world(x, y);
                let dist = world_pos.length();

                // Base height: High in center, low at edges
                let base_height = (50.0 - (dist / 5.0)).max(0.0);

                // Add some noise (simple sin waves)
                let noise = (world_pos.x * 0.1).sin() * 2.0 + (world_pos.y * 0.1).cos() * 2.0;

                let idx = y * self.width + x;
                self.heightmap[idx] = (base_height + noise).max(0.0);
            }
        }
    }

    pub fn erode(&mut self, graph: &CallGraph) {
        // Use macroquad::rand globally
        use macroquad::rand;

        for edge in &graph.edges {
            let start_node = &graph.nodes[edge.source];
            let end_node = &graph.nodes[edge.target];

            // Number of "droplets" / carving strength proportional to call weight
            // Weight ranges 10..1000 usually.
            let flow_strength = edge.weight * 0.05;

            let start_pos = start_node.pos;
            let end_pos = end_node.pos;
            let dist = start_pos.distance(end_pos);
            let steps = (dist * 1.5) as usize; // 1.5 steps per unit

            if steps == 0 {
                continue;
            }

            // Meander parameters
            let freq = rand::gen_range(0.05, 0.15);
            let phase = rand::gen_range(0.0, std::f32::consts::PI * 2.0);
            let amp = rand::gen_range(5.0, 15.0);

            for i in 0..=steps {
                let t = i as f32 / steps as f32;
                let mut pos = start_pos.lerp(end_pos, t);

                // Add meander perpendicular to direction
                let dir = (end_pos - start_pos).normalize_or_zero();
                let perp = vec2(-dir.y, dir.x);

                // Sine wave offset with dampening at ends
                let offset = (t * dist * freq + phase).sin() * amp * (1.0 - (t - 0.5).abs() * 2.0);

                pos += perp * offset;

                // Carve
                self.carve_at(pos, flow_strength);
            }
        }
    }

    fn carve_at(&mut self, pos: Vec2, amount: f32) {
        let (x, y) = self.world_to_grid(pos);
        if x >= self.width || y >= self.height {
            return;
        }

        // Simple 3x3 kernel carving for wider rivers
        for dy in -1..=1 {
            for dx in -1..=1 {
                let nx = x as isize + dx;
                let ny = y as isize + dy;

                if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                    let idx = ny as usize * self.width + nx as usize;
                    // Gaussian-ish falloff
                    let dist_sq = (dx * dx + dy * dy) as f32;
                    let factor = (-dist_sq / 2.0).exp();

                    self.heightmap[idx] -= amount * factor * 0.1;
                }
            }
        }
    }

    fn grid_to_world(&self, x: usize, y: usize) -> Vec2 {
        let x_f = (x as f32 / self.width as f32) * WORLD_SIZE - (WORLD_SIZE / 2.0);
        let y_f = (y as f32 / self.height as f32) * WORLD_SIZE - (WORLD_SIZE / 2.0);
        vec2(x_f, y_f)
    }

    fn world_to_grid(&self, pos: Vec2) -> (usize, usize) {
        let x_f = (pos.x + WORLD_SIZE / 2.0) / WORLD_SIZE * self.width as f32;
        let y_f = (pos.y + WORLD_SIZE / 2.0) / WORLD_SIZE * self.height as f32;
        (x_f as usize, y_f as usize)
    }

    pub fn get_mesh(&self) -> Mesh {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        for y in 0..self.height - 1 {
            for x in 0..self.width - 1 {
                let idx = y * self.width + x;

                let p1 = self.grid_to_world(x, y);
                let h1 = self.heightmap[idx];

                let p2 = self.grid_to_world(x + 1, y);
                let h2 = self.heightmap[idx + 1];

                let p3 = self.grid_to_world(x + 1, y + 1);
                let h3 = self.heightmap[idx + 1 + self.width];

                let p4 = self.grid_to_world(x, y + 1);
                let h4 = self.heightmap[idx + self.width];

                // Color based on height
                let c1 = self.get_color(h1);
                let c2 = self.get_color(h2);
                let c3 = self.get_color(h3);
                let c4 = self.get_color(h4);

                let v_start = vertices.len() as u16;

                // Normal is required in macroquad 0.4. We use up vector for simplicity.
                let normal = vec4(0., 1., 0., 1.);

                // Use .into() for colors as required by macroquad 0.4
                vertices.push(Vertex {
                    position: vec3(p1.x, h1, p1.y),
                    uv: vec2(0., 0.),
                    color: c1.into(),
                    normal,
                });
                vertices.push(Vertex {
                    position: vec3(p2.x, h2, p2.y),
                    uv: vec2(1., 0.),
                    color: c2.into(),
                    normal,
                });
                vertices.push(Vertex {
                    position: vec3(p3.x, h3, p3.y),
                    uv: vec2(1., 1.),
                    color: c3.into(),
                    normal,
                });
                vertices.push(Vertex {
                    position: vec3(p4.x, h4, p4.y),
                    uv: vec2(0., 1.),
                    color: c4.into(),
                    normal,
                });

                indices.push(v_start);
                indices.push(v_start + 1);
                indices.push(v_start + 2);

                indices.push(v_start);
                indices.push(v_start + 2);
                indices.push(v_start + 3);
            }
        }

        Mesh {
            vertices,
            indices,
            texture: None,
        }
    }

    fn get_color(&self, h: f32) -> Color {
        if h < 0.0 {
            BLUE // "Water" / Deep Erosion
        } else if h < 15.0 {
            Color::new(0.6, 0.4, 0.2, 1.0) // Canyon walls
        } else if h < 35.0 {
            GREEN // Grass
        } else {
            WHITE // Peaks
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use crate::graph::CallGraph;

    #[test]
    fn test_uplift() {
        let graph = CallGraph::generate_random();
        let mut terrain = Terrain::new();
        terrain.uplift(&graph);
        // Center should be high
        let center_idx = (terrain.height / 2) * terrain.width + (terrain.width / 2);
        // Height is 50 - dist/5. center dist is 0. So 50.
        // But noise might lower it slightly.
        assert!(terrain.heightmap[center_idx] > 40.0);
    }

    #[test]
    fn test_erode() {
        let graph = CallGraph::generate_random();
        let mut terrain = Terrain::new();
        terrain.uplift(&graph);
        let initial_height = terrain.heightmap.iter().sum::<f32>();

        terrain.erode(&graph);
        let eroded_height = terrain.heightmap.iter().sum::<f32>();

        assert!(eroded_height < initial_height);
    }
}
