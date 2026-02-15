use rayon::prelude::*;

pub struct GrayScottGrid {
    width: usize,
    height: usize,
    u: Vec<f32>,
    v: Vec<f32>,
    next_u: Vec<f32>,
    next_v: Vec<f32>,
}

impl GrayScottGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        // Initialize with U=1, V=0
        let u = vec![1.0; size];
        let v = vec![0.0; size];

        // Seed a small area with V to start reaction if needed,
        // but agents will be the seed in this hybrid.

        Self {
            width,
            height,
            u: u.clone(),
            v: v.clone(),
            next_u: u,
            next_v: v,
        }
    }

    pub fn width(&self) -> usize {
        self.width
    }

    pub fn height(&self) -> usize {
        self.height
    }

    pub fn u(&self) -> &[f32] {
        &self.u
    }

    pub fn v(&self) -> &[f32] {
        &self.v
    }

    pub fn deposit_v(&mut self, x: usize, y: usize, amount: f32) {
        let idx = y * self.width + x;
        if idx < self.v.len() {
            // Clamp to 1.0? Or let it go higher?
            // Standard Gray-Scott is usually 0..1.
            self.v[idx] = (self.v[idx] + amount).min(1.0);
            // Optionally deplete U? No, let U be replenished by feed.
        }
    }

    pub fn update(&mut self, feed: f32, kill: f32, dt: f32) {
        let w = self.width;
        let h = self.height;
        let diff_u = 0.16; // Standard GS parameters or tweaked?
        let diff_v = 0.08;

        // Parallel update
        let u = &self.u;
        let v = &self.v;

        // We zip next_u and next_v to update them together
        // Note: par_iter_mut enables parallel execution
        self.next_u
            .par_iter_mut()
            .zip(self.next_v.par_iter_mut())
            .enumerate()
            .for_each(|(i, (nu, nv))| {
                let x = i % w;
                let y = i / w;

                // Laplacian
                let mut sum_u = 0.0;
                let mut sum_v = 0.0;

                // 3x3 Convolution
                // Kernel:
                // 0.05 0.2 0.05
                // 0.2  -1  0.2
                // 0.05 0.2 0.05

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = (x as isize + dx).rem_euclid(w as isize) as usize;
                        let ny = (y as isize + dy).rem_euclid(h as isize) as usize;
                        let idx = ny * w + nx;

                        // Simple 3x3 Laplacian weights
                        let weight = if dx == 0 && dy == 0 {
                            -1.0
                        } else if dx == 0 || dy == 0 {
                            0.2
                        } else {
                            0.05
                        };

                        sum_u += u[idx] * weight;
                        sum_v += v[idx] * weight;
                    }
                }

                let cur_u = u[i];
                let cur_v = v[i];

                let reaction = cur_u * cur_v * cur_v;

                // Gray-Scott Equations
                // du/dt = Du * laplace(u) - uv^2 + f(1-u)
                // dv/dt = Dv * laplace(v) + uv^2 - (f+k)v

                let du = diff_u * sum_u - reaction + feed * (1.0 - cur_u);
                let dv = diff_v * sum_v + reaction - (feed + kill) * cur_v;

                *nu = (cur_u + du * dt).clamp(0.0, 1.0);
                *nv = (cur_v + dv * dt).clamp(0.0, 1.0);
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
    fn test_grid_consistency() {
        let grid = GrayScottGrid::new(10, 10);
        assert_eq!(grid.width(), 10);
        assert_eq!(grid.height(), 10);
        assert_eq!(grid.u().len(), 100);
        assert_eq!(grid.v().len(), 100);
    }
}
