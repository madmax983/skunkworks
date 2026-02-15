use std::f32::consts::PI;
use cgmath::{InnerSpace, Vector3};

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Vertex {
    pub position: [f32; 3],
    pub normal: [f32; 3],
    pub uv: [f32; 2],
}

impl Vertex {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Vertex>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Vertex,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 0,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 1,
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: (std::mem::size_of::<[f32; 3]>() * 2) as wgpu::BufferAddress,
                    shader_location: 2,
                    format: wgpu::VertexFormat::Float32x2,
                },
            ],
        }
    }
}

#[repr(C)]
#[derive(Copy, Clone, Debug, bytemuck::Pod, bytemuck::Zeroable)]
pub struct Instance {
    pub model_pos: [f32; 3],
    pub color: [f32; 4],
}

impl Instance {
    pub fn desc() -> wgpu::VertexBufferLayout<'static> {
        wgpu::VertexBufferLayout {
            array_stride: std::mem::size_of::<Instance>() as wgpu::BufferAddress,
            step_mode: wgpu::VertexStepMode::Instance,
            attributes: &[
                wgpu::VertexAttribute {
                    offset: 0,
                    shader_location: 5, // 0,1,2 used by Vertex. Let's use 5 to be safe or 3?
                    // Usually Vertex takes locations 0..N. Vertex has 0,1,2.
                    // Instance should take 3, 4?
                    format: wgpu::VertexFormat::Float32x3,
                },
                wgpu::VertexAttribute {
                    offset: std::mem::size_of::<[f32; 3]>() as wgpu::BufferAddress,
                    shader_location: 6,
                    format: wgpu::VertexFormat::Float32x4,
                },
            ],
        }
    }
}

/// Parametric equation for a Torus Knot (p, q).
/// t is in [0, 2*PI].
/// Returns the 3D position on the curve.
pub fn torus_knot(p: f32, q: f32, t: f32) -> Vector3<f32> {
    // Standard torus knot on a torus of major radius R=3, minor r=1.5
    let r_major = 3.0;
    let r_minor = 1.0;

    // r = r_major + r_minor * cos(q * t)
    let r = r_major + r_minor * (q * t).cos();

    // x = r * cos(p * t)
    // y = r * sin(p * t)
    // z = r_minor * sin(q * t)

    let x = r * (p * t).cos();
    let y = r * (p * t).sin();
    let z = r_minor * (q * t).sin();

    Vector3::new(x, y, z)
}

/// Generates a tube mesh around the torus knot.
/// tube_radius: Thickness of the tube.
/// u_steps: Segments along the length of the knot.
/// v_steps: Segments around the circumference of the tube.
pub fn generate_knot_tube(p: f32, q: f32, tube_radius: f32, u_steps: u32, v_steps: u32) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for i in 0..=u_steps {
        let t = (i as f32 / u_steps as f32) * 2.0 * PI;
        let t_next = ((i + 1) as f32 / u_steps as f32) * 2.0 * PI;

        // Position on the knot
        let p0 = torus_knot(p, q, t);
        let p1 = torus_knot(p, q, t_next);

        // Tangent vector
        let tangent = (p1 - p0).normalize();

        // Use the torus surface normal as the "Up" vector to avoid twisting
        // The torus center line is the circle of radius R_major in XY plane.
        // The point on that center line closest to p0 is:
        let p0_flat = Vector3::new(p0.x, p0.y, 0.0).normalize() * 3.0; // R_major = 3.0
        let torus_normal = (p0 - p0_flat).normalize();

        // If tangent and torus_normal are parallel (which shouldn't happen for standard knots), fallback
        let normal = if tangent.cross(torus_normal).magnitude2() < 0.001 {
             tangent.cross(Vector3::unit_z()).normalize()
        } else {
             torus_normal
        };

        let binormal = tangent.cross(normal).normalize();
        // Recalculate normal to ensure orthogonality
        let normal = binormal.cross(tangent).normalize();

        for j in 0..=v_steps {
            let theta = (j as f32 / v_steps as f32) * 2.0 * PI;

            // Circle in the plane perpendicular to tangent
            let cos_theta = theta.cos();
            let sin_theta = theta.sin();

            let local_pos = normal * cos_theta * tube_radius + binormal * sin_theta * tube_radius;
            let pos = p0 + local_pos;
            let norm = local_pos.normalize(); // Normal of the surface at this point

            vertices.push(Vertex {
                position: [pos.x, pos.y, pos.z],
                normal: [norm.x, norm.y, norm.z],
                uv: [i as f32 / u_steps as f32, j as f32 / v_steps as f32],
            });
        }
    }

    // Indices
    for i in 0..u_steps {
        for j in 0..v_steps {
            let width = v_steps + 1;

            let i0 = i * width + j;
            let i1 = (i + 1) * width + j;
            let i2 = (i + 1) * width + (j + 1);
            let i3 = i * width + (j + 1);

            indices.push(i0);
            indices.push(i1);
            indices.push(i2);

            indices.push(i0);
            indices.push(i2);
            indices.push(i3);
        }
    }

    (vertices, indices)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_torus_knot_points() {
        let p = 2.0;
        let q = 3.0;
        for i in 0..10 {
            let t = (i as f32 / 10.0) * 2.0 * PI;
            let pos = torus_knot(p, q, t);
            assert!(!pos.x.is_nan());
            assert!(!pos.y.is_nan());
            assert!(!pos.z.is_nan());
        }
    }

    #[test]
    fn test_generate_knot_tube() {
        let (verts, idxs) = generate_knot_tube(2.0, 3.0, 0.5, 10, 4);
        assert!(!verts.is_empty());
        assert!(!idxs.is_empty());
        // vertices count should be (u_steps+1)*(v_steps+1)
        assert_eq!(verts.len(), 11 * 5);
    }
}

/// Generates a UV sphere.
pub fn generate_sphere(radius: f32, u_steps: u32, v_steps: u32) -> (Vec<Vertex>, Vec<u32>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    for i in 0..=u_steps {
        let lat = (i as f32 / u_steps as f32) * PI; // 0 to PI
        let sin_lat = lat.sin();
        let cos_lat = lat.cos();

        for j in 0..=v_steps {
            let lon = (j as f32 / v_steps as f32) * 2.0 * PI; // 0 to 2PI
            let sin_lon = lon.sin();
            let cos_lon = lon.cos();

            let x = sin_lat * cos_lon;
            let y = sin_lat * sin_lon;
            let z = cos_lat;

            let pos = Vector3::new(x, y, z) * radius;
            let normal = Vector3::new(x, y, z); // Unit sphere normal is just position

            vertices.push(Vertex {
                position: [pos.x, pos.y, pos.z],
                normal: [normal.x, normal.y, normal.z],
                uv: [j as f32 / v_steps as f32, i as f32 / u_steps as f32],
            });
        }
    }

    for i in 0..u_steps {
        for j in 0..v_steps {
            let width = v_steps + 1;
            let i0 = i * width + j;
            let i1 = (i + 1) * width + j;
            let i2 = (i + 1) * width + (j + 1);
            let i3 = i * width + (j + 1);

            indices.push(i0);
            indices.push(i1);
            indices.push(i2);

            indices.push(i0);
            indices.push(i2);
            indices.push(i3);
        }
    }

    (vertices, indices)
}
