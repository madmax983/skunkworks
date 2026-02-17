#[cfg(feature = "parallel")]
use rayon::prelude::*;

pub struct GrayScott {
    width: usize,
    height: usize,
    u: Vec<f32>,
    v: Vec<f32>,
    next_u: Vec<f32>,
    next_v: Vec<f32>,
    pub diff_u: f32,
    pub diff_v: f32,
}

impl GrayScott {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            u: vec![1.0; size],
            v: vec![0.0; size],
            next_u: vec![1.0; size],
            next_v: vec![0.0; size],
            // Default parameters (tuned for Myco-Diffusion / Standard GS)
            diff_u: 0.16,
            diff_v: 0.08,
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

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn add_chemical(&mut self, x: usize, y: usize, amount: f32) {
        let idx = self.get_index(x, y);
        if idx < self.v.len() {
            self.v[idx] = (self.v[idx] + amount).min(1.0);
        }
    }

    /// Update the reaction-diffusion simulation.
    ///
    /// * `feed`: The feed rate (f) - adds U.
    /// * `kill`: The kill rate (k) - removes V.
    /// * `dt`: Delta time.
    pub fn update(&mut self, feed: f32, kill: f32, dt: f32) {
        #[cfg(feature = "parallel")]
        self.update_parallel(feed, kill, dt);

        #[cfg(not(feature = "parallel"))]
        self.update_sequential(feed, kill, dt);
    }

    #[cfg(feature = "parallel")]
    fn update_parallel(&mut self, feed: f32, kill: f32, dt: f32) {
        let w = self.width;
        let h = self.height;
        let diff_u = self.diff_u;
        let diff_v = self.diff_v;

        // Destructure to separate borrows
        let u = &self.u;
        let v = &self.v;
        let next_u = &mut self.next_u;
        let next_v = &mut self.next_v;

        // Parallel update using rayon
        // We zip next_u and next_v to update them together
        next_u
            .par_iter_mut()
            .zip(next_v.par_iter_mut())
            .enumerate()
            .for_each(|(i, (nu, nv))| {
                let x = i % w;
                let y = i / w;

                let (cur_u, cur_v, lap_u, lap_v) = Self::compute_laplacian(x, y, w, h, u, v);

                let reaction = cur_u * cur_v * cur_v;

                let du = diff_u * lap_u - reaction + feed * (1.0 - cur_u);
                let dv = diff_v * lap_v + reaction - (feed + kill) * cur_v;

                *nu = (cur_u + du * dt).clamp(0.0, 1.0);
                *nv = (cur_v + dv * dt).clamp(0.0, 1.0);
            });

        std::mem::swap(&mut self.u, &mut self.next_u);
        std::mem::swap(&mut self.v, &mut self.next_v);
    }

    #[cfg(not(feature = "parallel"))]
    fn update_sequential(&mut self, feed: f32, kill: f32, dt: f32) {
        let w = self.width;
        let h = self.height;
        let diff_u = self.diff_u;
        let diff_v = self.diff_v;

        for y in 0..h {
            for x in 0..w {
                let i = y * w + x;

                let (cur_u, cur_v, lap_u, lap_v) =
                    Self::compute_laplacian(x, y, w, h, &self.u, &self.v);

                let reaction = cur_u * cur_v * cur_v;

                let du = diff_u * lap_u - reaction + feed * (1.0 - cur_u);
                let dv = diff_v * lap_v + reaction - (feed + kill) * cur_v;

                self.next_u[i] = (cur_u + du * dt).clamp(0.0, 1.0);
                self.next_v[i] = (cur_v + dv * dt).clamp(0.0, 1.0);
            }
        }

        std::mem::swap(&mut self.u, &mut self.next_u);
        std::mem::swap(&mut self.v, &mut self.next_v);
    }

    #[inline(always)]
    fn compute_laplacian(
        x: usize,
        y: usize,
        w: usize,
        h: usize,
        u: &[f32],
        v: &[f32],
    ) -> (f32, f32, f32, f32) {
        let i = y * w + x;
        let cur_u = u[i];
        let cur_v = v[i];

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

        (cur_u, cur_v, sum_u, sum_v)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let gs = GrayScott::new(10, 10);
        assert_eq!(gs.u.len(), 100);
        assert_eq!(gs.u[0], 1.0);
        assert_eq!(gs.v[0], 0.0);
    }

    #[test]
    fn test_add_chemical() {
        let mut gs = GrayScott::new(10, 10);
        gs.add_chemical(5, 5, 0.5);
        let idx = gs.get_index(5, 5);
        assert_eq!(gs.v[idx], 0.5);
    }

    #[test]
    fn test_update() {
        let mut gs = GrayScott::new(10, 10);
        gs.add_chemical(5, 5, 1.0); // Seed

        let idx = gs.get_index(5, 5);
        let neighbor = gs.get_index(5, 6);

        assert_eq!(gs.u[idx], 1.0);
        assert_eq!(gs.v[neighbor], 0.0);

        // Use standard "Spots" parameters
        let f = 0.055;
        let k = 0.062;
        let dt = 1.0;

        gs.update(f, k, dt);

        // U is consumed by reaction
        assert!(gs.u[idx] < 1.0, "U should be consumed");

        // V diffuses to neighbor
        assert!(gs.v[neighbor] > 0.0, "V should diffuse");
    }
}
