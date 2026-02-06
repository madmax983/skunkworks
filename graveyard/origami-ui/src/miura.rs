use nalgebra::Point3;

pub struct MiuraGrid {
    pub rows: usize, // Number of vertices in rows
    pub cols: usize, // Number of vertices in cols
    pub a: f64,      // Length of segment along the horizontal zig-zag
    pub b: f64,      // Length of segment along the vertical direction
    pub alpha: f64,  // The angle of the parallelogram (in radians)
}

impl MiuraGrid {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            a: 5.0,
            b: 5.0,
            alpha: 60.0_f64.to_radians(),
        }
    }

    /// Computes the 3D coordinates of the grid vertices for a given fold parameter `t`.
    /// `t` ranges from 0.0 (flat) to 1.0 (fully folded state).
    pub fn compute_vertices(&self, t: f64) -> Vec<Point3<f64>> {
        let mut vertices = Vec::with_capacity(self.rows * self.cols);

        // Clamp t
        let t = t.clamp(0.0, 1.0);

        // Let psi go from 0 (flat) to alpha (max fold).
        // If psi = alpha, then sx = cos(alpha), and the grid collapses.
        let max_psi = self.alpha - 0.1; // Limit slightly below alpha to avoid degeneracy
        let psi = t * max_psi;

        let sx = psi.cos();
        let term = psi.tan() / self.alpha.tan();
        let sy = (1.0 - term * term).sqrt(); // cos(asin(...))
        let sz = psi.sin();

        // Calculate total dimensions to center the grid
        // The width of the folded pattern depends on sx.
        // The height depends on sy.
        let width = (self.cols as f64 - 1.0) * self.a * sx;
        let height = (self.rows as f64 - 1.0) * self.b * sy;
        let x_offset = -width / 2.0;
        let y_offset = -height / 2.0;

        for r in 0..self.rows {
            for c in 0..self.cols {
                // Parity determines the zig-zag offset
                let row_odd = r % 2 == 1;

                // Base coords
                let mut x = c as f64 * self.a * sx;
                let y = r as f64 * self.b * sy;

                // Real Miura stagger: every other row is shifted
                let dx = if row_odd {
                    self.b * self.alpha.cos() * sx
                } else {
                    0.0
                };

                x += dx;

                // Z height depends on parity of (r+c)
                // Mountain/Valley assignment creates the Z structure
                let z = if (r + c) % 2 == 1 {
                    sz * self.a // Amplitude
                } else {
                    0.0
                };

                vertices.push(Point3::new(x + x_offset, -y + -y_offset, z));
            }
        }

        vertices
    }

    /// Returns the indices of vertices forming a face (quad) at (r, c).
    /// r in 0..rows-1, c in 0..cols-1
    /// Returns [top_left, top_right, bottom_right, bottom_left]
    pub fn get_face_indices(&self, r: usize, c: usize) -> Option<[usize; 4]> {
        if r >= self.rows - 1 || c >= self.cols - 1 {
            return None;
        }

        let tl = r * self.cols + c;
        let tr = r * self.cols + (c + 1);
        let br = (r + 1) * self.cols + (c + 1);
        let bl = (r + 1) * self.cols + c;

        Some([tl, tr, br, bl])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_face_indices() {
        let grid = MiuraGrid::new(3, 3);
        // Should have 2x2 = 4 faces
        let face = grid.get_face_indices(0, 0).unwrap();
        // 0, 1, 4, 3
        assert_eq!(face, [0, 1, 4, 3]);

        let face = grid.get_face_indices(1, 1).unwrap();
        // 4, 5, 8, 7
        assert_eq!(face, [4, 5, 8, 7]);
    }
}
