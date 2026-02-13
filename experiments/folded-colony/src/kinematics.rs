use macroquad::prelude::*;

#[derive(Clone, Copy, Debug)]
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

#[derive(Clone, Copy, Debug)]
pub struct MiuraState {
    pub l_x: f32,
    pub l_y: f32,
    pub s_y: f32,
    pub h: f32,
}

impl MiuraGrid {
    pub fn new(cols: usize, rows: usize) -> Self {
        Self {
            params: MiuraParams::default(),
            cols,
            rows,
        }
    }

    pub fn calculate_state(&self, expansion: f32) -> MiuraState {
        let a = self.params.a;
        let b = self.params.b;
        let alpha = self.params.alpha;

        let expansion = expansion.clamp(0.01, 1.0);
        let l_x_max = a * alpha.sin();
        let l_x = expansion * l_x_max;

        let c1 = a * a - l_x * l_x - b * b;
        let c2 = a * b * alpha.cos() - b * b;
        let denominator = c1 - 2.0 * c2;

        let (l_y, s_y, h) = if denominator.abs() < 1e-6 {
             (b, a * alpha.cos(), 0.0)
        } else {
             let l_y_sq = (c2 * c2) / denominator;
             if l_y_sq < 0.0 {
                 (b, a * alpha.cos(), 0.0)
             } else {
                 let l_y = l_y_sq.sqrt();
                 let s_y = l_y + c2 / l_y;
                 let h_sq_4 = b * b - l_y * l_y;
                 let h = if h_sq_4 < 0.0 { 0.0 } else { (h_sq_4 / 4.0).sqrt() };
                 (l_y, s_y, h)
             }
        };

        MiuraState { l_x, l_y, s_y, h }
    }

    pub fn get_vertices(&self, expansion: f32) -> Vec<Vec3> {
        let state = self.calculate_state(expansion);
        self.get_vertices_from_state(state)
    }

    pub fn get_vertices_from_state(&self, state: MiuraState) -> Vec<Vec3> {
        let mut vertices = Vec::with_capacity((self.rows + 1) * (self.cols + 1));
        for j in 0..=self.rows {
            for i in 0..=self.cols {
                // Integer coordinates match exact grid points
                // We can use get_pos but optimized for integers to avoid float precision noise on boundaries?
                // Actually get_pos should be precise enough.
                vertices.push(self.get_pos(i as f32, j as f32, state));
            }
        }
        vertices
    }

    pub fn get_pos(&self, u: f32, v: f32, state: MiuraState) -> Vec3 {
        let MiuraState { l_x, l_y, s_y, h } = state;

        let x = u * l_x;

        // Triangle wave for y-offset
        // u % 2.0 gives 0..2
        // If u is integer i:
        // i=0 -> 0. i=1 -> 1. i=2 -> 0.
        // Formula: if (u%2) < 1.0 { u%2 } else { 2.0 - (u%2) } matches.

        let u_mod = u % 2.0;
        let wave = if u_mod <= 1.0 { u_mod } else { 2.0 - u_mod };

        // Wait, for integer i:
        // i=0 -> wave=0 -> y = v*ly.
        // i=1 -> wave=1 -> y = v*ly + sy.
        // This matches.

        let y = v * l_y + wave * s_y;

        // Z height interpolation
        // Base sign depends on parity of cell.
        // But for get_pos(integer), we want:
        // (i+j)%2 == 0 -> h
        // (i+j)%2 == 1 -> -h

        // Let's use cosine approximation for smooth interpolation
        // z = h * cos(pi * u) * cos(pi * v)
        // Check integers:
        // (0,0) -> h * 1 * 1 = h. Correct.
        // (1,0) -> h * -1 * 1 = -h. Correct.
        // (0,1) -> h * 1 * -1 = -h. Correct.
        // (1,1) -> h * -1 * -1 = h. Correct.
        // This is much smoother and simpler than linear interpolation with parity checks!

        let z = h * (u * std::f32::consts::PI).cos() * (v * std::f32::consts::PI).cos();

        vec3(x, y, z)
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

        for exp in expansions {
            let verts = grid.get_vertices(exp);
            let width = grid.cols + 1;

            // Check horizontal edges (length a)
            for j in 0..=grid.rows {
                for i in 0..grid.cols {
                    let idx1 = j * width + i;
                    let idx2 = j * width + i + 1;
                    let d = verts[idx1].distance(verts[idx2]);
                    assert!((d - a).abs() < 1e-4, "Horizontal edge length mismatch at exp={}: got {}, expected {}", exp, d, a);
                }
            }

            // Check vertical edges (length b)
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
