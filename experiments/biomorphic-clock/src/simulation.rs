use rayon::prelude::*;
use rand::Rng;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub u: Vec<f32>,
    pub v: Vec<f32>,
    next_u: Vec<f32>,
    next_v: Vec<f32>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            u: vec![1.0; size],
            v: vec![0.0; size],
            next_u: vec![1.0; size],
            next_v: vec![0.0; size],
        }
    }

    pub fn seed(&mut self, x: usize, y: usize) {
        let idx = y * self.width + x;
        if idx < self.u.len() {
            self.v[idx] = 1.0;
        }
    }

    /// Seeds a random pattern in the center of the grid.
    pub fn random_seed_center(&mut self) {
        let cx = self.width / 2;
        let cy = self.height / 2;
        let mut rng = rand::thread_rng();

        for y in cy.saturating_sub(5)..cy.saturating_add(5) {
            for x in cx.saturating_sub(5)..cx.saturating_add(5) {
                if rng.gen_bool(0.5) {
                    self.seed(x, y);
                }
            }
        }
    }

    pub fn update(&mut self, dt: f32, angle: f32) {
        // Gray-Scott parameters
        let f = 0.0545;
        let k = 0.0620;
        let diff_u = 1.0;
        let diff_v = 0.5;

        // Compute anisotropic kernel weights
        // Angle 0 is along X+
        let cos_a = angle.cos();
        let sin_a = angle.sin();

        // 3x3 kernel neighbors (excluding center)
        let neighbors = [
            (-1, -1),
            (0, -1),
            (1, -1),
            (-1, 0),
            (1, 0),
            (-1, 1),
            (0, 1),
            (1, 1),
        ];

        let mut weights = [0.0f32; 8];
        let mut total_weight = 0.0;

        // Sigma for major and minor axes
        let sigma_major = 1.0;
        let sigma_minor = 0.1; // Much smaller to force anisotropy

        for (i, &(dx, dy)) in neighbors.iter().enumerate() {
            let dx = dx as f32;
            let dy = dy as f32;

            // Rotate point into aligned frame
            // x' = x cos - y sin
            // y' = x sin + y cos
            // Wait, usually x' is along the major axis.
            let rx = dx * cos_a + dy * sin_a;
            let ry = -dx * sin_a + dy * cos_a;

            let w = (-(rx * rx) / (2.0 * sigma_major) - (ry * ry) / (2.0 * sigma_minor)).exp();
            weights[i] = w;
            total_weight += w;
        }

        // Normalize weights so they sum to 1.0 (for the diffusion part)
        // Laplacian = sum(w_i * u_i) - 1.0 * u_center ?
        // Standard Laplacian stencil [1, -2, 1] is effectively (u_left + u_right - 2u_center).
        // Here we want sum(w_i * (u_i - u_c)).
        // So L = sum(w_i * u_i) - (sum w_i) * u_center.
        // If we normalize weights to sum to 1, then L = sum(w_norm_i * u_i) - u_center.

        for w in weights.iter_mut() {
            *w /= total_weight;
        }

        // Use read-only reference for input and mutable for output
        let width = self.width;
        let height = self.height;
        let u = &self.u;
        let v = &self.v;
        let next_u = &mut self.next_u;
        let next_v = &mut self.next_v;

        // Parallel iteration over rows
        next_u
            .par_iter_mut()
            .zip(next_v.par_iter_mut())
            .enumerate()
            .for_each(|(i, (nu, nv))| {
                let x = i % width;
                let y = i / width;

                // Handle boundaries (wrap or clamp) - Wrapping is better for patterns
                let mut lap_u = 0.0;
                let mut lap_v = 0.0;

                let cur_u = u[i];
                let cur_v = v[i];

                for (j, &(dx, dy)) in neighbors.iter().enumerate() {
                    // Wrap coordinates
                    let nx = (x as isize + dx).rem_euclid(width as isize) as usize;
                    let ny = (y as isize + dy).rem_euclid(height as isize) as usize;
                    let nidx = ny * width + nx;

                    let w = weights[j];
                    lap_u += w * u[nidx];
                    lap_v += w * v[nidx];
                }

                lap_u -= cur_u; // because sum of weights is 1.0
                lap_v -= cur_v;

                // Reaction-Diffusion
                let reaction = cur_u * cur_v * cur_v;
                *nu = cur_u + (diff_u * lap_u - reaction + f * (1.0 - cur_u)) * dt;
                *nv = cur_v + (diff_v * lap_v + reaction - (f + k) * cur_v) * dt;

                // Clamp to avoid instability
                *nu = nu.clamp(0.0, 1.0);
                *nv = nv.clamp(0.0, 1.0);
            });

        // Swap buffers
        std::mem::swap(&mut self.u, &mut self.next_u);
        std::mem::swap(&mut self.v, &mut self.next_v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_random_seed_center() {
        let mut grid = Grid::new(20, 20);
        grid.random_seed_center();

        let center_seeds = grid.v.iter().filter(|&&v| v > 0.0).count();
        assert!(center_seeds > 0, "Should have seeded at least one cell");
        assert!(center_seeds <= 100, "Should not exceed max possible seeds (10x10 area)");
    }
}
