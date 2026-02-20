#[allow(dead_code)]
pub struct GrayScottCpu {
    width: usize,
    height: usize,
    pub u: Vec<f32>,
    pub v: Vec<f32>,
}

#[allow(dead_code)]
impl GrayScottCpu {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            u: vec![1.0; width * height],
            v: vec![0.0; width * height],
        }
    }

    pub fn set(&mut self, x: usize, y: usize, u: f32, v: f32) {
        let idx = y * self.width + x;
        self.u[idx] = u;
        self.v[idx] = v;
    }

    pub fn step(&mut self, f: f32, k: f32, diff_u: f32, diff_v: f32) {
        let mut next_u = self.u.clone();
        let mut next_v = self.v.clone();
        let w = self.width as isize;
        let h = self.height as isize;

        for y in 0..h {
            for x in 0..w {
                let idx = (y * w + x) as usize;

                // Laplacian with toroidal wrapping
                let left = if x == 0 { w - 1 } else { x - 1 };
                let right = if x == w - 1 { 0 } else { x + 1 };
                let up = if y == 0 { h - 1 } else { y - 1 };
                let down = if y == h - 1 { 0 } else { y + 1 };

                let idx_l = (y * w + left) as usize;
                let idx_r = (y * w + right) as usize;
                let idx_u = (up * w + x) as usize;
                let idx_d = (down * w + x) as usize;

                let u = self.u[idx];
                let v = self.v[idx];

                let lap_u = self.u[idx_l] + self.u[idx_r] + self.u[idx_u] + self.u[idx_d] - 4.0 * u;
                let lap_v = self.v[idx_l] + self.v[idx_r] + self.v[idx_u] + self.v[idx_d] - 4.0 * v;

                let uvv = u * v * v;

                // Gray-Scott formulas
                // du/dt = Du * lap_u - uv^2 + f(1-u)
                // dv/dt = Dv * lap_v + uv^2 - (k+f)v
                let du = diff_u * lap_u - uvv + f * (1.0 - u);
                let dv = diff_v * lap_v + uvv - (k + f) * v;

                next_u[idx] = u + du;
                next_v[idx] = v + dv;

                // Clamp to prevent explosion
                next_u[idx] = next_u[idx].clamp(0.0, 1.0);
                next_v[idx] = next_v[idx].clamp(0.0, 1.0);
            }
        }

        self.u = next_u;
        self.v = next_v;
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diffusion() {
        // Setup a 3x3 grid
        // Center has V=1.0, others V=0.0. U=0.0 everywhere to disable reaction (uvv term).
        let mut gs = GrayScottCpu::new(3, 3);
        // Clear U to 0
        for i in 0..9 {
            gs.u[i] = 0.0;
        }

        gs.set(1, 1, 0.0, 1.0); // Center V=1.0

        // Step with no reaction (f=0, k=0) but high diffusion
        gs.step(0.0, 0.0, 0.2, 0.2);

        let center_idx = 1 * 3 + 1;
        let neighbor_idx = 1 * 3 + 0;

        // Assert diffusion happened
        // V center was 1.0. Laplacian was -4.
        // dV = 0.2 * -4 = -0.8. New V = 0.2.
        assert!(
            gs.v[center_idx] < 1.0,
            "Center should diffuse away (was 1.0, became {})",
            gs.v[center_idx]
        );

        // V neighbor was 0.0. Laplacian was 1.
        // dV = 0.2 * 1 = 0.2. New V = 0.2.
        assert!(
            gs.v[neighbor_idx] > 0.0,
            "Neighbor should receive diffusion (was 0.0, became {})",
            gs.v[neighbor_idx]
        );
    }
}
