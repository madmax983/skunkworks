use nalgebra::Point3;

#[derive(Debug, Clone)]
pub struct MiuraPattern {
    pub rows: usize,
    pub cols: usize,
    pub a: f64,     // Horizontal segment length
    pub b: f64,     // Vertical segment length
    pub gamma: f64, // Sector angle (radians). Usually around 84 deg.
}

impl MiuraPattern {
    pub fn new(rows: usize, cols: usize) -> Self {
        Self {
            rows,
            cols,
            a: 5.0,
            b: 5.0,
            gamma: 84.0_f64.to_radians(),
        }
    }

    /// Checks Kawasaki's theorem for a single internal vertex.
    /// For the Miura-ori standard CP, the sector angles are gamma, 180-gamma, 180-gamma, gamma.
    /// Alternating sum: gamma - (180-gamma) + (180-gamma) - gamma = 0.
    /// This function verifies that property mathematically given the stored gamma.
    pub fn check_kawasaki(&self) -> bool {
        let alpha1 = self.gamma;
        let alpha2 = std::f64::consts::PI - self.gamma;
        let alpha3 = std::f64::consts::PI - self.gamma;
        let alpha4 = self.gamma;

        let sum_odd = alpha1 + alpha3;
        let sum_even = alpha2 + alpha4;

        // Using epsilon for float comparison
        (sum_odd - sum_even).abs() < 1e-6
    }

    /// Computes vertices for a given fold parameter `rho` in [0, 1].
    /// 0.0 = Fully Folded (collapsed)
    /// 1.0 = Fully Flat
    pub fn compute_vertices(&self, rho: f64) -> Vec<Point3<f64>> {
        let rho = rho.clamp(0.0, 1.0);
        let mut vertices = Vec::with_capacity(self.rows * self.cols);

        // We use the parameterization where `theta` is the angle of the horizontal edges
        // relative to the "folded" vertical plane.
        // When rho = 1 (Flat), the effective angle theta matches gamma.
        // When rho = 0 (Folded), theta goes to 0?

        // Let's model it:
        // The horizontal edges rotate out of the plane.
        // Let `psi` be the angle between the horizontal edge and the YZ plane?
        // No, let's use the scaling factors derived from the geometry.

        // S_x: Scaling in X (width). Max at Flat. Min at Folded.
        // S_y: Scaling in Y (height). Max at Flat? No, usually Miura contracts in BOTH directions.

        // Let's derive S_x and S_y from `theta` (the fold angle of the zigzag).
        // Let theta be the angle between the face and the horizontal plane.
        // Flat: theta = 0. Folded: theta = 90.
        // We use the parameterization where `rho` controls the deviation angle `psi`.
        // Flat state (rho=1) => psi = 0.
        // Folded state (rho=0) => psi approaches gamma.

        let max_psi = self.gamma - 0.05; // Stay away from singularity
        let psi = (1.0 - rho) * max_psi; // rho=1 (Flat) -> psi=0. rho=0 (Folded) -> psi=max.

        let sx = psi.cos(); // Width contraction
        let term = psi.tan() / self.gamma.tan();
        let sy = (1.0 - term * term).sqrt(); // Height contraction
        let sz = psi.sin(); // Z-amplitude factor (approx)

        // Center offsets
        let total_width = (self.cols as f64 - 1.0) * self.a * sx;
        let total_height = (self.rows as f64 - 1.0) * self.b * sy;
        let x_off = -total_width / 2.0;
        let y_off = -total_height / 2.0;

        for r in 0..self.rows {
            for c in 0..self.cols {
                let row_odd = r % 2 == 1;

                // Base grid positions
                let mut x = c as f64 * self.a * sx;
                let y = r as f64 * self.b * sy;

                // Stagger for odd rows
                // In flat state, stagger is a * cos(gamma).
                // In folded state, it scales?
                // Using `origami-ui` logic: dx = b * cos(alpha) * sx ??
                // Wait, the stagger comes from the `a` edge if vertical lines are straight.
                // If vertical lines are straight, stagger is 0 in Y direction?
                // Standard Miura: Horizontal lines are zigzags. Vertical lines are straight.
                // So odd rows are just shifted in X?
                // The shift is `a * cos(gamma)` in flat state?
                // No, standard Miura has vertices at (x, y) and (x + dx, y).

                let stagger = if row_odd {
                    self.a * self.gamma.cos() * sx // Scale stagger with width
                                                   // Wait, if sx=1 (flat), stagger = a * cos(gamma).
                                                   // But sx = cos(psi). So stagger = a * cos(gamma) * cos(psi).
                } else {
                    0.0
                };

                x += stagger;

                // Z-parity
                // (r+c) odd/even determines mountain/valley.
                let z_mag = sz * self.a; // Amplitude proportional to 'a'
                let z = if (r + c) % 2 == 1 { z_mag } else { -z_mag };

                vertices.push(Point3::new(x + x_off, -y - y_off, z));
            }
        }

        vertices
    }

    pub fn get_quad_indices(&self, r: usize, c: usize) -> Option<[usize; 4]> {
        if r >= self.rows - 1 || c >= self.cols - 1 {
            return None;
        }
        let w = self.cols;
        // Standard grid indexing
        let tl = r * w + c;
        let tr = r * w + (c + 1);
        let br = (r + 1) * w + (c + 1);
        let bl = (r + 1) * w + c;
        Some([tl, tr, br, bl])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_kawasaki() {
        let pattern = MiuraPattern::new(5, 5);
        assert!(pattern.check_kawasaki());
    }

    #[test]
    fn test_compute_bounds() {
        let pattern = MiuraPattern::new(5, 5);
        let flat = pattern.compute_vertices(1.0);
        let folded = pattern.compute_vertices(0.0);

        // Flat width should be greater than folded width
        let flat_width = flat.last().unwrap().x - flat.first().unwrap().x;
        let folded_width = folded.last().unwrap().x - folded.first().unwrap().x;

        assert!(flat_width.abs() > folded_width.abs());
    }
}
