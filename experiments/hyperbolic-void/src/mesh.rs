use bytemuck::{Pod, Zeroable};
use wgpu::util::DeviceExt;

#[repr(C)]
#[derive(Copy, Clone, Debug, Pod, Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub color: [f32; 3],
}

pub struct Mesh {
    pub vertex_buffer: wgpu::Buffer,
    pub index_buffer: wgpu::Buffer,
    pub num_indices: u32,
}

impl Mesh {
    pub fn new_cube(device: &wgpu::Device, subdivisions: u32) -> Self {
        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Helper to push vertex
        let mut push_vert = |x, y, z, r, g, b| {
            let idx = vertices.len() as u32;
            vertices.push(Vertex {
                position: [x, y, z],
                color: [r, g, b],
            });
            idx
        };

        // Size of the cube in Poincaré coordinates (near origin)
        let s = 0.2;

        // Function to create a grid for a face
        let mut create_face = |axis: usize, dir: f32, color: [f32;3]| {
            for i in 0..subdivisions {
                for j in 0..subdivisions {
                    let u0 = (i as f32 / subdivisions as f32) * 2.0 - 1.0;
                    let u1 = ((i + 1) as f32 / subdivisions as f32) * 2.0 - 1.0;
                    let v0 = (j as f32 / subdivisions as f32) * 2.0 - 1.0;
                    let v1 = ((j + 1) as f32 / subdivisions as f32) * 2.0 - 1.0;

                    let mut p = [[0.0;3]; 4];
                    // Quad corners
                    let quad_uv = [(u0, v0), (u1, v0), (u1, v1), (u0, v1)];

                    for (k, (u, v)) in quad_uv.iter().enumerate() {
                        let mut pos = [0.0; 3];
                        match axis {
                            0 => { pos[0] = dir * s; pos[1] = u * s; pos[2] = v * s; } // X face
                            1 => { pos[0] = u * s; pos[1] = dir * s; pos[2] = v * s; } // Y face
                            2 => { pos[0] = u * s; pos[1] = v * s; pos[2] = dir * s; } // Z face
                            _ => {}
                        }
                        p[k] = pos;
                    }

                    let i0 = push_vert(p[0][0], p[0][1], p[0][2], color[0], color[1], color[2]);
                    let i1 = push_vert(p[1][0], p[1][1], p[1][2], color[0], color[1], color[2]);
                    let i2 = push_vert(p[2][0], p[2][1], p[2][2], color[0], color[1], color[2]);
                    let i3 = push_vert(p[3][0], p[3][1], p[3][2], color[0], color[1], color[2]);

                    indices.extend_from_slice(&[i0, i1, i2, i2, i3, i0]);
                }
            }
        };

        create_face(0, 1.0, [1.0, 0.0, 0.0]); // +X Red
        create_face(0, -1.0, [0.5, 0.0, 0.0]); // -X Dark Red
        create_face(1, 1.0, [0.0, 1.0, 0.0]); // +Y Green
        create_face(1, -1.0, [0.0, 0.5, 0.0]); // -Y Dark Green
        create_face(2, 1.0, [0.0, 0.0, 1.0]); // +Z Blue
        create_face(2, -1.0, [0.0, 0.0, 0.5]); // -Z Dark Blue

        let vertex_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Vertex Buffer"),
            contents: bytemuck::cast_slice(&vertices),
            usage: wgpu::BufferUsages::VERTEX,
        });

        let index_buffer = device.create_buffer_init(&wgpu::util::BufferInitDescriptor {
            label: Some("Index Buffer"),
            contents: bytemuck::cast_slice(&indices),
            usage: wgpu::BufferUsages::INDEX,
        });

        Self {
            vertex_buffer,
            index_buffer,
            num_indices: indices.len() as u32,
        }
    }
}
