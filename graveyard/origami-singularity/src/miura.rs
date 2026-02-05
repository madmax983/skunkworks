use nalgebra::Point3;

pub struct MiuraGrid {
    pub rows: usize,
    pub cols: usize,
    pub a: f64,     // Length of segment along the horizontal zig-zag
    pub b: f64,     // Length of segment along the vertical direction
    pub alpha: f64, // The angle of the parallelogram (in radians)
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
        let max_psi = self.alpha - 0.1; // Limit slightly below alpha
        let psi = t * max_psi;

        let sx = psi.cos();
        let term = psi.tan() / self.alpha.tan();
        let sy = (1.0 - term * term).sqrt(); // cos(asin(...))
        let sz = psi.sin();

        // Now generate grid
        // Center the grid
        let width = (self.cols as f64 - 1.0) * self.a * sx;
        let height = (self.rows as f64 - 1.0) * self.b * sy;
        let x_offset = -width / 2.0;
        let y_offset = -height / 2.0;

        for r in 0..self.rows {
            for c in 0..self.cols {
                // Parity check
                let row_odd = r % 2 == 1;

                // Base coords
                let mut x = c as f64 * self.a * sx;
                let y = r as f64 * self.b * sy;

                // Real Miura stagger
                let dx = if row_odd {
                    self.b * self.alpha.cos() * sx
                } else {
                    0.0
                }; // Approximate

                x += dx;

                // Z height depends on parity
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
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_vertices_count() {
        let grid = MiuraGrid::new(10, 10);
        let vertices = grid.compute_vertices(0.5);
        assert_eq!(vertices.len(), 100);
    }

    #[test]
    fn test_compute_vertices_bounds() {
        let grid = MiuraGrid::new(2, 2);
        // Test flat state
        let vertices_flat = grid.compute_vertices(0.0);
        assert_eq!(vertices_flat.len(), 4);

        // Z should be 0 for all points in flat state?
        // Wait, my implementation adds Z based on parity even if sz is small?
        // if t=0, psi=0, sz=0. So Z should be 0.
        for v in vertices_flat {
            assert!(
                (v.z).abs() < 1e-10,
                "Z coordinate should be 0 in flat state, got {}",
                v.z
            );
        }
    }
}
