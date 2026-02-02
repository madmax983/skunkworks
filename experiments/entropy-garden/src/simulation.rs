#[allow(dead_code)]
pub struct GrayScott {
    width: usize,
    height: usize,
    u: Vec<f64>,
    v: Vec<f64>,
    next_u: Vec<f64>,
    next_v: Vec<f64>,
    diff_u: f64,
    diff_v: f64,
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
            diff_u: 0.16, // Typical diffusion rates
            diff_v: 0.08,
        }
    }

    pub fn seed(&mut self) {
        // Place a seed in the center
        let center_x = self.width / 2;
        let center_y = self.height / 2;
        let radius = 5;

        for y in (center_y - radius)..(center_y + radius) {
            for x in (center_x - radius)..(center_x + radius) {
                if x < self.width && y < self.height {
                    let idx = y * self.width + x;
                    self.v[idx] = 1.0;
                }
            }
        }
    }

    pub fn update(&mut self, f: f64, k: f64) {
        let w = self.width;
        let h = self.height;

        for y in 0..h {
            for x in 0..w {
                let idx = y * w + x;
                let u = self.u[idx];
                let v = self.v[idx];

                // Laplacian with 3x3 convolution and wrapping
                let mut lap_u = 0.0;
                let mut lap_v = 0.0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        let nx = (x as isize + dx).rem_euclid(w as isize) as usize;
                        let ny = (y as isize + dy).rem_euclid(h as isize) as usize;
                        let n_idx = ny * w + nx;

                        let weight = if dx == 0 && dy == 0 {
                            -1.0
                        } else if dx == 0 || dy == 0 {
                            0.2
                        } else {
                            0.05
                        };

                        lap_u += self.u[n_idx] * weight;
                        lap_v += self.v[n_idx] * weight;
                    }
                }

                let uvv = u * v * v;

                let du = (self.diff_u * lap_u) - uvv + (f * (1.0 - u));
                let dv = (self.diff_v * lap_v) + uvv - ((f + k) * v);

                self.next_u[idx] = (u + du).clamp(0.0, 1.0);
                self.next_v[idx] = (v + dv).clamp(0.0, 1.0);
            }
        }

        std::mem::swap(&mut self.u, &mut self.next_u);
        std::mem::swap(&mut self.v, &mut self.next_v);
    }

    pub fn get_concentration(&self, x: usize, y: usize) -> (f64, f64) {
        let idx = y * self.width + x;
        (self.u[idx], self.v[idx])
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_simulation_step() {
        let mut sim = GrayScott::new(20, 20);
        sim.seed(); // Seeds with v=1.0 in center

        let center_idx = 10 * 20 + 10;
        let initial_u = sim.u[center_idx];

        // F=0.055, k=0.062 are standard "spot" parameters
        sim.update(0.055, 0.062);

        let new_u = sim.u[center_idx];

        // u should decrease because it is consumed by the reaction (uv^2 term)
        // v might stay at 1.0 if it clamps
        assert!(
            new_u < initial_u,
            "Concentration u should decrease (consumed)"
        );
    }
}
