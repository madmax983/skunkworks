use macroquad::prelude::*;

#[derive(Clone, Copy)]
pub struct MiuraParams {
    pub a: f32,
    pub b: f32,
    pub gamma: f32, // in radians
}

pub struct Vertex {
    pub pos: Vec3,
    pub uv: Vec2,
}

pub struct Mesh {
    pub vertices: Vec<Vertex>,
    pub indices: Vec<u16>,
}

pub struct MiuraOri {
    pub params: MiuraParams,
    pub grid_size: (usize, usize),
}

impl MiuraOri {
    pub fn new(params: MiuraParams, grid_size: (usize, usize)) -> Self {
        Self { params, grid_size }
    }

    pub fn generate(&self, extension_factor: f32) -> Mesh {
        let (cols, rows) = self.grid_size;
        let a = self.params.a;
        let b = self.params.b;
        let gamma = self.params.gamma;

        // Calculate limits
        // sin(theta) >= cos(gamma) for the sqrt to be real
        let cos_gamma = gamma.cos();
        let theta_min = cos_gamma.asin();
        let theta_max = std::f32::consts::FRAC_PI_2;

        // Map extension factor 0..1 to theta_min..theta_max
        // 1.0 = Flat (theta = 90 deg)
        // 0.0 = Folded (theta = theta_min)
        // Add a small epsilon to theta_min to avoid div by zero or precision issues
        let theta = theta_min + (theta_max - theta_min) * extension_factor.clamp(0.001, 1.0);

        let sin_theta = theta.sin();
        let cos_theta = theta.cos();

        // Derived dimensions
        let sx = a * sin_theta;
        let h_amp = a * cos_theta; // Z amplitude

        let x_off = b * cos_gamma / sin_theta;
        let sy_sq = b * b - x_off * x_off;
        let sy = if sy_sq > 0.0 { sy_sq.sqrt() } else { 0.0 };

        let mut vertices = Vec::new();
        let mut indices = Vec::new();

        // Center offsets
        let total_w = (cols as f32) * sx + x_off; // Approx width
        let total_h = (rows as f32) * sy;
        let cx = total_w / 2.0;
        let cy = total_h / 2.0;

        for j in 0..=rows {
            for i in 0..=cols {
                // X position: i * sx + shift for row
                // The shift logic for Miura:
                // Rows alternate shift direction?
                // Based on derivation: x_ij = i*Sx + (j%2)*x_off
                let x = (i as f32) * sx + ((j % 2) as f32) * x_off;

                let y = (j as f32) * sy;

                // Z position: alternates along X
                let z = ((i % 2) as f32) * h_amp;

                vertices.push(Vertex {
                    pos: vec3(x - cx, y - cy, z),
                    uv: vec2(i as f32 / cols as f32, j as f32 / rows as f32),
                });
            }
        }

        // Generate Indices (Quads -> Triangles)
        for j in 0..rows {
            for i in 0..cols {
                let v_cols = cols + 1;
                let p00 = (j * v_cols + i) as u16;
                let p10 = (j * v_cols + (i + 1)) as u16;
                let p01 = ((j + 1) * v_cols + i) as u16;
                let p11 = ((j + 1) * v_cols + (i + 1)) as u16;

                // Two triangles
                indices.push(p00);
                indices.push(p10);
                indices.push(p01);

                indices.push(p10);
                indices.push(p11);
                indices.push(p01);
            }
        }

        Mesh { vertices, indices }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_miura_flat() {
        let params = MiuraParams {
            a: 1.0,
            b: 1.0,
            gamma: 80.0f32.to_radians(),
        };
        let origami = MiuraOri::new(params, (2, 2));

        // Fully expanded (flat)
        // factor 1.0 -> theta = PI/2 -> cos(theta) = 0 -> z = 0
        let mesh = origami.generate(1.0);

        assert!(!mesh.vertices.is_empty(), "Mesh should have vertices");

        for v in mesh.vertices {
            assert!(
                v.pos.z.abs() < 1e-4,
                "Z should be approx 0 when flat, got {}",
                v.pos.z
            );
        }
    }

    #[test]
    fn test_miura_folded() {
        let params = MiuraParams {
            a: 1.0,
            b: 1.0,
            gamma: 80.0f32.to_radians(),
        };
        let origami = MiuraOri::new(params, (4, 4));

        // Partially folded
        let mesh = origami.generate(0.5);

        assert!(!mesh.vertices.is_empty());

        let mut has_nonzero_z = false;
        for v in mesh.vertices {
            if v.pos.z.abs() > 1e-4 {
                has_nonzero_z = true;
                break;
            }
        }
        assert!(has_nonzero_z, "Should have non-zero Z when folded");
    }
}
