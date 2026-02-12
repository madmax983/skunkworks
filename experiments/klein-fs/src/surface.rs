use glam::Vec3;
use std::f32::consts::PI;

const R: f32 = 2.0;

/// Returns a point on the Figure-8 immersion of the Klein bottle.
/// u in [0, 2*PI], v in [0, 2*PI]
/// Returns Vec3 with Y-up orientation (swapping Z and Y from standard mathematical form)
pub fn figure_8_immersion(u: f32, v: f32) -> Vec3 {
    let r = R;
    let cos_u = u.cos();
    let sin_u = u.sin();
    let _cos_v = v.cos();
    let sin_v = v.sin();
    let sin_2v = (2.0 * v).sin();
    let cos_u2 = (u / 2.0).cos();
    let sin_u2 = (u / 2.0).sin();

    // Standard math form (Z-up):
    // x = (r + cos(u/2) sin(v) - sin(u/2) sin(2v)) cos(u)
    // y = (r + cos(u/2) sin(v) - sin(u/2) sin(2v)) sin(u)
    // z = sin(u/2) sin(v) + cos(u/2) sin(2v)

    let x = (r + cos_u2 * sin_v - sin_u2 * sin_2v) * cos_u;
    let y_math = (r + cos_u2 * sin_v - sin_u2 * sin_2v) * sin_u;
    let z_math = sin_u2 * sin_v + cos_u2 * sin_2v;

    // Return Y-up for macroquad/game engines: (x, z, -y) or (x, z, y)
    // Let's use (x, z, y)
    Vec3::new(x, z_math, y_math)
}

/// Generates a wireframe mesh for the Klein bottle.
/// Returns (vertices, indices) for GL_LINES.
pub fn generate_wireframe_mesh(u_res: usize, v_res: usize) -> (Vec<Vec3>, Vec<u16>) {
    let mut vertices = Vec::new();
    let mut indices = Vec::new();

    // Generate vertices including the seam for visual continuity
    for i in 0..=u_res {
        let u = (i as f32 / u_res as f32) * 2.0 * PI;
        for j in 0..=v_res {
            let v = (j as f32 / v_res as f32) * 2.0 * PI;
            vertices.push(figure_8_immersion(u, v));
        }
    }

    let width = (v_res + 1) as u16;

    for i in 0..u_res {
        for j in 0..v_res {
            let curr = i as u16 * width + j as u16;
            let right = curr + 1;
            let down = (i as u16 + 1) * width + j as u16;

            // Horizontal segment
            indices.push(curr);
            indices.push(right);

            // Vertical segment
            indices.push(curr);
            indices.push(down);
        }
    }

    // Close the loop explicitly?
    // The loops above go from 0..u_res and 0..v_res, which covers the grid cells.
    // Since we included the seam vertices (i=u_res and j=v_res), the last iteration of i
    // connects row (u_res-1) to row (u_res), which is visually correct.
    // The "seam" vertices at i=u_res should match i=0 topologically, but geometrically they are the same point.

    (vertices, indices)
}
