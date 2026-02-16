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

    /// Computes vertices for a given fold parameter `rho` in [0, 1].
    /// 0.0 = Fully Folded (collapsed)
    /// 1.0 = Fully Flat
    pub fn compute_vertices(&self, rho: f64) -> Vec<Point3<f64>> {
        let rho = rho.clamp(0.0, 1.0);
        let mut vertices = Vec::with_capacity(self.rows * self.cols);

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

                let stagger = if row_odd {
                    self.a * self.gamma.cos() * sx
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

    pub fn get_face_center(&self, vertices: &[Point3<f64>], r: usize, c: usize) -> Option<Point3<f64>> {
        let indices = self.get_quad_indices(r, c)?;

        let mut cx = 0.0;
        let mut cy = 0.0;
        let mut cz = 0.0;

        for &i in &indices {
            let v = vertices.get(i)?;
            cx += v.x;
            cy += v.y;
            cz += v.z;
        }

        Some(Point3::new(cx / 4.0, cy / 4.0, cz / 4.0))
    }
}
