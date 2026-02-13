use macroquad::prelude::*;

pub struct MiuraParams {
    pub a: f32,     // Side length a (horizontal-ish)
    pub b: f32,     // Side length b (vertical-ish)
    pub alpha: f32, // Sector angle (radians)
}

impl Default for MiuraParams {
    fn default() -> Self {
        Self {
            a: 1.0,
            b: 1.0,
            alpha: 80.0f32.to_radians(),
        }
    }
}

pub struct MiuraGrid {
    pub params: MiuraParams,
    pub cols: usize,
    pub rows: usize,
}

impl MiuraGrid {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            params: MiuraParams::default(),
            cols,
            rows,
        }
    }

    /// Computes the vertices for a given expansion factor (0.0 to 1.0).
    /// 1.0 = fully flat.
    /// 0.0 = fully compressed (mathematical limit).
    pub fn get_vertices(&self, expansion: f32) -> Vec<Vec3> {
        let a = self.params.a;
        let b = self.params.b;
        let alpha = self.params.alpha;

        // Clamp expansion to avoid singularities or negative roots
        let expansion = expansion.clamp(0.01, 1.0);

        // Max l_x occurs when flat (h=0)
        // flat state: s_y = a * cos(alpha), l_x = a * sin(alpha)
        let l_x_max = a * alpha.sin();
        let l_x = expansion * l_x_max;

        // Constants derived from constraints
        // 1. s_y^2 - l_y^2 = a^2 - l_x^2 - b^2 = C1
        let c1 = a * a - l_x * l_x - b * b;

        // 2. s_y * l_y - l_y^2 = a * b * cos(alpha) - b^2 = C2
        let c2 = a * b * alpha.cos() - b * b;

        // l_y^2 = C2^2 / (C1 - 2*C2)
        let denominator = c1 - 2.0 * c2;

        if denominator.abs() < 1e-6 {
             // Singularity handling (should correspond to flat state usually)
             // If flat, l_y = b
             return self.generate_grid(l_x, b, a * alpha.cos(), 0.0);
        }

        let l_y_sq = (c2 * c2) / denominator;

        if l_y_sq < 0.0 {
            // Invalid state (physically impossible for this geometry/expansion)
            // Fallback to flat
            return self.generate_grid(l_x_max, b, a * alpha.cos(), 0.0);
        }

        let l_y = l_y_sq.sqrt();

        // s_y = l_y + C2 / l_y
        let s_y = l_y + c2 / l_y;

        // 4h^2 = b^2 - l_y^2
        let h_sq_4 = b * b - l_y * l_y;

        let h = if h_sq_4 < 0.0 {
            0.0
        } else {
            (h_sq_4 / 4.0).sqrt()
        };

        self.generate_grid(l_x, l_y, s_y, h)
    }

    fn generate_grid(&self, l_x: f32, l_y: f32, s_y: f32, h: f32) -> Vec<Vec3> {
        let mut vertices = Vec::with_capacity((self.rows + 1) * (self.cols + 1));

        for j in 0..=self.rows {
            for i in 0..=self.cols {
                let x = i as f32 * l_x;

                // Even columns (0, 2, ...) have y-offset 0 relative to odd columns?
                // Logic derived:
                // v(i,j) y = j * l_y + (i%2) * s_y
                // Let's verify this matches the zigzag.
                // i=0: y = j*l_y.
                // i=1: y = j*l_y + s_y.
                // The offset is s_y for odd columns.
                let y = j as f32 * l_y + (i % 2) as f32 * s_y;

                // Checkerboard height
                let z = if (i + j) % 2 == 0 { h } else { -h };

                vertices.push(vec3(x, y, z));
            }
        }
        vertices
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_edge_length_preservation() {
        let grid = MiuraGrid::new(2, 2);
        let expansions = vec![0.1, 0.5, 0.9, 1.0];
        let a = grid.params.a;
        let b = grid.params.b;
        // The effective diagonal for the check isn't strictly necessary if a and b are correct.
        // We just check neighbor distances.

        for exp in expansions {
            let verts = grid.get_vertices(exp);
            let width = grid.cols + 1;

            // Check horizontal edges (length a)
            // v(i,j) to v(i+1,j)
            for j in 0..=grid.rows {
                for i in 0..grid.cols {
                    let idx1 = j * width + i;
                    let idx2 = j * width + i + 1;
                    let d = verts[idx1].distance(verts[idx2]);
                    assert!((d - a).abs() < 1e-4, "Horizontal edge length mismatch at exp={}: got {}, expected {}", exp, d, a);
                }
            }

            // Check vertical edges (length b)
            // v(i,j) to v(i,j+1)
            for j in 0..grid.rows {
                for i in 0..=grid.cols {
                    let idx1 = j * width + i;
                    let idx2 = (j + 1) * width + i;
                    let d = verts[idx1].distance(verts[idx2]);
                    assert!((d - b).abs() < 1e-4, "Vertical edge length mismatch at exp={}: got {}, expected {}", exp, d, b);
                }
            }
        }
    }
}
