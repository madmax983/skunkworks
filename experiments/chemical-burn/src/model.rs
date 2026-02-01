pub struct GrayScott {
    pub width: usize,
    pub height: usize,
    pub u: Vec<f64>,
    pub v: Vec<f64>,
    pub next_u: Vec<f64>,
    pub next_v: Vec<f64>,
    pub du: f64,
    pub dv: f64,
    pub feed: f64,
    pub kill: f64,
}

impl GrayScott {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let u = vec![1.0; size];
        let v = vec![0.0; size];

        let mut model = Self {
            width,
            height,
            u: u.clone(),
            v: v.clone(),
            next_u: u,
            next_v: v,
            du: 0.16,
            dv: 0.08,
            feed: 0.055,
            kill: 0.062,
        };

        // Seed center
        model.add_chemical(width / 2, height / 2, 10);
        model
    }

    pub fn resize(&mut self, width: usize, height: usize) {
        if self.width == width && self.height == height {
            return;
        }
        self.width = width;
        self.height = height;
        let size = width * height;
        // Re-initialize (simplest logic for now, preserves nothing)
        self.u = vec![1.0; size];
        self.v = vec![0.0; size];
        self.next_u = vec![1.0; size];
        self.next_v = vec![0.0; size];
        self.add_chemical(width / 2, height / 2, 10);
    }

    pub fn add_chemical(&mut self, cx: usize, cy: usize, radius: usize) {
        let r = radius as isize;
        for dy in -r..=r {
            for dx in -r..=r {
                if dx * dx + dy * dy <= r * r {
                    let x = cx as isize + dx;
                    let y = cy as isize + dy;
                    if x >= 0 && x < self.width as isize && y >= 0 && y < self.height as isize {
                        let idx = (y as usize) * self.width + (x as usize);
                        self.v[idx] = 1.0; // Add "predator"
                    }
                }
            }
        }
    }

    pub fn update(&mut self, dt: f64) {
        let w = self.width;
        let h = self.height;
        let du = self.du;
        let dv = self.dv;
        let f = self.feed;
        let k = self.kill;

        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let i = y * w + x;

                let u = self.u[i];
                let v = self.v[i];

                // Laplacian (Simple 5-point stencil)
                //   0  1  0
                //   1 -4  1
                //   0  1  0
                // Assumes toroid wrapping not needed if we skip edges (Dirichlet BC = 0, or just clamps)
                // For simplicity, we skip edges here (treated as walls with no flux effectively if neighbors constant)

                let neighbors_u = self.u[i - 1] + self.u[i + 1] + self.u[i - w] + self.u[i + w];
                let neighbors_v = self.v[i - 1] + self.v[i + 1] + self.v[i - w] + self.v[i + w];

                let lap_u = neighbors_u - 4.0 * u;
                let lap_v = neighbors_v - 4.0 * v;

                let uvv = u * v * v;

                let du_dt = du * lap_u - uvv + f * (1.0 - u);
                let dv_dt = dv * lap_v + uvv - (f + k) * v;

                self.next_u[i] = (u + du_dt * dt).clamp(0.0, 1.0);
                self.next_v[i] = (v + dv_dt * dt).clamp(0.0, 1.0);
            }
        }

        // Swap buffers
        std::mem::swap(&mut self.u, &mut self.next_u);
        std::mem::swap(&mut self.v, &mut self.next_v);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diffusion() {
        let mut gs = GrayScott::new(100, 100);
        let initial_v_sum: f64 = gs.v.iter().sum();

        // Run update
        gs.update(1.0);

        let next_v_sum: f64 = gs.v.iter().sum();

        // V should have changed (diffused/reacted)
        // With standard GS parameters, V grows initially or spreads.
        // Just checking it's not identical is enough for a basic smoke test.
        assert!(next_v_sum != initial_v_sum, "Model should evolve over time");

        // Check bounds
        for v in &gs.v {
            assert!(*v >= 0.0 && *v <= 1.0, "V value out of bounds");
        }
    }
}
