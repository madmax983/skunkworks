use glam::Vec4;
use std::f32::consts::PI;

pub struct Mesh4D {
    pub vertices: Vec<Vec4>,
    pub edges: Vec<(usize, usize)>,
}

impl Mesh4D {
    pub fn new() -> Self {
        Self {
            vertices: Vec::new(),
            edges: Vec::new(),
        }
    }
}

pub fn tesseract() -> Mesh4D {
    let mut vertices = Vec::new();
    let mut edges = Vec::new();

    // Generate 16 vertices
    for i in 0..16 {
        let x = if (i & 1) != 0 { 1.0 } else { -1.0 };
        let y = if (i & 2) != 0 { 1.0 } else { -1.0 };
        let z = if (i & 4) != 0 { 1.0 } else { -1.0 };
        let w = if (i & 8) != 0 { 1.0 } else { -1.0 };
        vertices.push(Vec4::new(x, y, z, w));
    }

    // Generate edges
    for i in 0..16 {
        for j in 0..4 {
            let neighbor = i ^ (1 << j);
            if i < neighbor {
                edges.push((i, neighbor));
            }
        }
    }

    Mesh4D { vertices, edges }
}

/// Generates a 4D path representing a 3D signal evolving over time.
/// num_points: Number of steps along W.
/// w_range: (min_w, max_w)
pub fn time_series(num_points: usize, w_range: (f32, f32)) -> Mesh4D {
    let mut vertices = Vec::new();
    let mut edges = Vec::new();

    let (min_w, max_w) = w_range;
    let step = (max_w - min_w) / (num_points as f32 - 1.0);

    for i in 0..num_points {
        let w = min_w + i as f32 * step;

        // Signal functions
        // x = sin(3t)
        // y = cos(2t)
        // z = sin(5t) * cos(t)
        // t is mapped from w, but let's just use w directly or scaled.
        let t = w * PI; // Scale W to radians if W is small

        let x = (3.0 * t).sin();
        let y = (2.0 * t).cos();
        let z = (5.0 * t).sin() * (t).cos();

        vertices.push(Vec4::new(x, y, z, w));

        if i > 0 {
            edges.push((i - 1, i));
        }
    }

    Mesh4D { vertices, edges }
}

pub fn combine(meshes: Vec<Mesh4D>) -> Mesh4D {
    let mut combined = Mesh4D::new();
    let mut offset = 0;

    for mesh in meshes {
        combined.vertices.extend(mesh.vertices);
        for (start, end) in mesh.edges {
            combined.edges.push((start + offset, end + offset));
        }
        offset = combined.vertices.len();
    }

    combined
}
