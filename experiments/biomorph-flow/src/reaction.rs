pub struct GrayScott {
    pub width: usize,
    pub height: usize,
    pub u: Vec<f64>,
    pub v: Vec<f64>,
    u_next: Vec<f64>,
    v_next: Vec<f64>,
    pub f: f64,
    pub k: f64,
    pub du: f64,
    pub dv: f64,
}

impl GrayScott {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        // Initial state: Full U, Empty V
        let u = vec![1.0; size];
        let v = vec![0.0; size];

        Self {
            width,
            height,
            u: u.clone(),
            v: v.clone(),
            u_next: u,
            v_next: v,
            f: 0.0545, // Default "Mitosis" roughly
            k: 0.062,
            du: 1.0,
            dv: 0.5,
        }
    }

    pub fn reset(&mut self) {
        self.u.fill(1.0);
        self.v.fill(0.0);
        self.seed_center();
    }

    pub fn seed_center(&mut self) {
        self.seed_at(self.width / 2, self.height / 2, 5);
    }

    pub fn seed_at(&mut self, cx: usize, cy: usize, radius: usize) {
        for y in (cy.saturating_sub(radius))..=(cy + radius).min(self.height - 1) {
            for x in (cx.saturating_sub(radius))..=(cx + radius).min(self.width - 1) {
                let dx = x as isize - cx as isize;
                let dy = y as isize - cy as isize;
                if dx * dx + dy * dy <= (radius * radius) as isize {
                    let idx = y * self.width + x;
                    self.v[idx] = 1.0;
                }
            }
        }
    }

    pub fn update(&mut self) {
        // Convolution weights (standard Laplacian 3x3)
        // 0.05  0.2  0.05
        // 0.2  -1.0  0.2
        // 0.05  0.2  0.05

        let w = self.width;
        let h = self.height;

        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let idx = y * w + x;

                let u = self.u[idx];
                let v = self.v[idx];

                // Calculate Laplacian
                // Neighbors
                // Top
                let n_top = (y - 1) * w + x;
                // Bottom
                let n_bot = (y + 1) * w + x;
                // Left
                let n_left = y * w + (x - 1);
                // Right
                let n_right = y * w + (x + 1);

                // Corners
                let n_tl = (y - 1) * w + (x - 1);
                let n_tr = (y - 1) * w + (x + 1);
                let n_bl = (y + 1) * w + (x - 1);
                let n_br = (y + 1) * w + (x + 1);

                // Apply kernel
                let center_w = -1.0;
                let adj_w = 0.2;
                let diag_w = 0.05;

                let sum_u = self.u[idx] * center_w
                    + (self.u[n_top] + self.u[n_bot] + self.u[n_left] + self.u[n_right]) * adj_w
                    + (self.u[n_tl] + self.u[n_tr] + self.u[n_bl] + self.u[n_br]) * diag_w;

                let sum_v = self.v[idx] * center_w
                    + (self.v[n_top] + self.v[n_bot] + self.v[n_left] + self.v[n_right]) * adj_w
                    + (self.v[n_tl] + self.v[n_tr] + self.v[n_bl] + self.v[n_br]) * diag_w;

                let lap_u = sum_u;
                let lap_v = sum_v;

                // Reaction-Diffusion
                let uvv = u * v * v;

                let du = self.du * lap_u - uvv + self.f * (1.0 - u);
                let dv = self.dv * lap_v + uvv - (self.f + self.k) * v;

                self.u_next[idx] = (u + du).clamp(0.0, 1.0);
                self.v_next[idx] = (v + dv).clamp(0.0, 1.0);
            }
        }

        // Swap buffers
        std::mem::swap(&mut self.u, &mut self.u_next);
        std::mem::swap(&mut self.v, &mut self.v_next);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conservation_sanity() {
        let mut gs = GrayScott::new(20, 20);
        gs.seed_center();

        // Run a few steps
        for _ in 0..10 {
            gs.update();
        }

        // Check values are within bounds
        for v in &gs.v {
            assert!(*v >= 0.0 && *v <= 1.0, "V value out of bounds: {}", v);
        }
        for u in &gs.u {
            assert!(*u >= 0.0 && *u <= 1.0, "U value out of bounds: {}", u);
        }
    }
}
