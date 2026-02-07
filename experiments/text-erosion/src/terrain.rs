use macroquad::prelude::*;
use noise::{NoiseFn, Perlin};

pub fn generate_mesh(width: usize, height: usize, sdf: &[f32], time: f64) -> Mesh {
    assert!(width * height < 65536, "Mesh too large for u16 indices. Reduce resolution.");

    let mut vertices = Vec::with_capacity(width * height);
    let mut indices = Vec::with_capacity((width - 1) * (height - 1) * 6);

    let perlin = Perlin::new(1);

    // 1. Generate Heightmap
    let mut heights = vec![0.0; width * height];

    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let dist = sdf[idx];

            // Base height from SDF
            let mut h = if dist > 0.0 {
                // Inside text: Mountain
                dist * 2.0 + 5.0
            } else {
                // Outside text: Low terrain
                // Smooth falloff from boundary?
                // dist is negative.
                // Let's make it -2.0.
                -2.0
            };

            // Apply Noise
            // We use 3 layers of noise for detail
            let scale1 = 0.05;
            let scale2 = 0.1;

            // Use 3D noise with time for animation
            let n1 = perlin.get([x as f64 * scale1, y as f64 * scale1, time * 0.2]);
            let n2 = perlin.get([x as f64 * scale2, y as f64 * scale2, time * 0.5]);

            let noise_val = n1 * 4.0 + n2 * 2.0;

            h += noise_val as f32;

            heights[idx] = h;
        }
    }

    // 2. Generate Vertices with Normals
    for y in 0..height {
        for x in 0..width {
            let idx = y * width + x;
            let h = heights[idx];

            // Calculate Normal using central differences where possible
            let h_l = if x > 0 { heights[idx - 1] } else { h };
            let h_r = if x < width - 1 { heights[idx + 1] } else { h };
            let h_u = if y > 0 { heights[idx - width] } else { h };
            let h_d = if y < height - 1 { heights[idx + width] } else { h };

            // Vector along X: (2, h_r - h_l, 0) - wait, X step is 1?
            // If grid step is 1.0.
            // Tangent X: (2.0, h_r - h_l, 0.0) -> Normalized?
            // Tangent Z: (0.0, h_d - h_u, 2.0)
            // Normal = Cross(Z, X) = (-dy_z * dx_y, ..., ...)
            // Simplified: (-dh/dx, 1, -dh/dz).

            let dh_dx = (h_r - h_l) / 2.0;
            let dh_dz = (h_d - h_u) / 2.0;

            let normal = vec3(-dh_dx, 1.0, -dh_dz).normalize();

            // Color mapping
            let slope = normal.y; // 1.0 is flat up

            let color = if h < -5.0 {
                BLUE
            } else if h < 0.0 {
                Color::new(0.2, 0.6, 0.2, 1.0) // Grass
            } else if h < 10.0 {
                if slope > 0.8 { Color::new(0.4, 0.8, 0.4, 1.0) } else { Color::new(0.5, 0.4, 0.3, 1.0) } // Green or Dirt
            } else if h < 20.0 {
                if slope > 0.6 { DARKGRAY } else { GRAY } // Rock
            } else {
                WHITE // Snow
            };

            // Position: Center the mesh around (0,0)
            let px = (x as f32) - (width as f32 / 2.0);
            let pz = (y as f32) - (height as f32 / 2.0);

            vertices.push(Vertex {
                position: vec3(px, h, pz),
                uv: vec2(x as f32 / width as f32, y as f32 / height as f32),
                color: color.into(),
                normal: vec4(normal.x, normal.y, normal.z, 0.0), // Vec4 required
            });
        }
    }

    // 3. Generate Indices
    for y in 0..height - 1 {
        for x in 0..width - 1 {
            let i = (y * width + x) as u16;
            let next_row = ((y + 1) * width + x) as u16;

            // Triangle 1
            indices.push(i);
            indices.push(next_row);
            indices.push(i + 1);

            // Triangle 2
            indices.push(i + 1);
            indices.push(next_row);
            indices.push(next_row + 1);
        }
    }

    Mesh {
        vertices,
        indices,
        texture: None,
    }
}
