use nalgebra::Point3;

pub struct MiuraGrid {
    pub rows: usize,
    pub cols: usize,
    pub a: f32,
    pub b: f32,
    pub gamma: f32,
}

impl MiuraGrid {
    pub fn new(rows: usize, cols: usize, a: f32, b: f32, gamma: f32) -> Self {
        Self { rows, cols, a, b, gamma }
    }

    /// Compute vertices for a given fold factor rho (0.0 to 1.0).
    /// rho represents the horizontal extension ratio (w / a).
    /// Returns vertices in row-major order.
    pub fn compute_vertices(&self, rho: f32) -> Vec<Point3<f32>> {
        // Constrain rho to valid range [0.1, sin(gamma)]
        // We avoid 0.0 to prevent division by zero (d=0).
        // We cap at sin(gamma) because d >= a cos(gamma) is required.

        let sin_gamma = self.gamma.sin();
        // Allow a small epsilon buffer
        let max_rho = (sin_gamma - 0.001).max(0.1);
        let clamped_rho = rho.clamp(0.1, max_rho);

        let w = self.a * clamped_rho;
        // d = sqrt(a^2 - w^2)
        let d = (self.a.powi(2) - w.powi(2)).sqrt();

        // q = (ab cos gamma) / d
        // If d is very small (near 0), q blows up. But clamped_rho limits d from below?
        // No, d is small if rho is near 1.
        // We limit rho <= sin(gamma) < 1. So d > 0.
        // d is minimal when rho is maximal.

        let q = (self.a * self.b * self.gamma.cos()) / d;

        // v = sqrt(b^2 - q^2)
        let v_sq = self.b.powi(2) - q.powi(2);
        let v = if v_sq > 0.0 { v_sq.sqrt() } else { 0.0 };

        let mut vertices = Vec::with_capacity((self.rows + 1) * (self.cols + 1));

        for i in 0..=self.rows {
            for j in 0..=self.cols {
                let x = j as f32 * w;
                // y coordinate based on row spacing v
                // But typically y is the "straight" direction in projection.
                // My derivation assumed straight columns (vertical in projection).
                // So y is just i * v.
                let y = i as f32 * v;

                // z coordinate logic:
                // Row 0: 0, d, 0, d...
                // Row 1: q, d+q, q, d+q...
                // Row 2: 2q, d+2q, 2q, d+2q...
                let z_base = if j % 2 == 1 { d } else { 0.0 };
                let z = z_base + i as f32 * q;

                vertices.push(Point3::new(x, y, z));
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
        let a = 10.0;
        let b = 10.0;
        // Gamma = 60 degrees. sin(60) ~ 0.866.
        // Rho = 0.5 (safe range).
        let grid = MiuraGrid::new(2, 2, a, b, 60.0f32.to_radians());
        let vertices = grid.compute_vertices(0.5);

        // Check horizontal edge (0,0) to (0,1)
        let v00 = vertices[0];
        let v01 = vertices[1];
        let dist_a = (v01 - v00).norm();
        assert!((dist_a - a).abs() < 1e-4, "Horizontal edge length should be preserved. Expected {}, got {}", a, dist_a);

        // Check vertical edge (0,0) to (1,0)
        // Index of (1,0) is (cols+1) * 1 = 3
        let v10 = vertices[3];
        let dist_b = (v10 - v00).norm();
        assert!((dist_b - b).abs() < 1e-4, "Vertical edge length should be preserved. Expected {}, got {}", b, dist_b);
    }
}
