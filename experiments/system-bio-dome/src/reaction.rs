// 🧬 Allele A: Inherited from experiments/chem-sys
// Represents the Biological substrate (Gray-Scott Reaction Diffusion)

#[derive(Debug)]
pub struct ChemicalSystem {
    pub width: usize,
    pub height: usize,
    pub u: Vec<f64>,
    pub v: Vec<f64>,
    next_u: Vec<f64>,
    next_v: Vec<f64>,
    pub f: f64,  // feed rate
    pub k: f64,  // kill rate
    pub du: f64, // diffusion u
    pub dv: f64, // diffusion v
}

impl ChemicalSystem {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            u: vec![1.0; size],
            v: vec![0.0; size],
            next_u: vec![0.0; size],
            next_v: vec![0.0; size],
            f: 0.055, // Initial defaults
            k: 0.062,
            du: 1.0,
            dv: 0.5,
        }
    }

    pub fn update(&mut self, dt: f64) {
        let w = self.width;
        let h = self.height;

        // Laplacian weights
        let center_w = -1.0;
        let adj_w = 0.2;
        let diag_w = 0.05;

        for y in 0..h {
            // Pre-calculate Y neighbors with wrap-around
            let prev_y = if y == 0 { h - 1 } else { y - 1 };
            let next_y = if y == h - 1 { 0 } else { y + 1 };

            let row = y * w;
            let row_up = prev_y * w;
            let row_down = next_y * w;

            for x in 0..w {
                // X neighbors with wrap-around
                let prev_x = if x == 0 { w - 1 } else { x - 1 };
                let next_x = if x == w - 1 { 0 } else { x + 1 };

                let i = row + x;
                let u = self.u[i];
                let v = self.v[i];

                // Laplacian U
                let sum_u_adj = self.u[row_up + x] + self.u[row_down + x] + self.u[row + prev_x] + self.u[row + next_x];
                let sum_u_diag = self.u[row_up + prev_x] + self.u[row_up + next_x] + self.u[row_down + prev_x] + self.u[row_down + next_x];
                let lap_u = sum_u_adj * adj_w + sum_u_diag * diag_w + u * center_w;

                // Laplacian V
                let sum_v_adj = self.v[row_up + x] + self.v[row_down + x] + self.v[row + prev_x] + self.v[row + next_x];
                let sum_v_diag = self.v[row_up + prev_x] + self.v[row_up + next_x] + self.v[row_down + prev_x] + self.v[row_down + next_x];
                let lap_v = sum_v_adj * adj_w + sum_v_diag * diag_w + v * center_w;

                let uvv = u * v * v;

                // Gray-Scott formulas
                let du_dt = self.du * lap_u - uvv + self.f * (1.0 - u);
                let dv_dt = self.dv * lap_v + uvv - (self.f + self.k) * v;

                self.next_u[i] = (u + du_dt * dt).clamp(0.0, 1.0);
                self.next_v[i] = (v + dv_dt * dt).clamp(0.0, 1.0);
            }
        }

        std::mem::swap(&mut self.u, &mut self.next_u);
        std::mem::swap(&mut self.v, &mut self.next_v);
    }

    pub fn add_chemical(&mut self, x: usize, y: usize, amount: f64) {
        if x < self.width && y < self.height {
            let i = y * self.width + x;
            self.v[i] += amount;
        }
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_diffusion() {
        let mut sys = ChemicalSystem::new(10, 10);
        // Add a spike of V in the middle
        sys.add_chemical(5, 5, 1.0);

        // Update should diffuse it
        sys.update(1.0);

        // After diffusion, the center should decrease (spread out)
        // and neighbors should increase.
        let neighbor_idx = sys.get_index(5, 6);
        assert!(
            sys.v[neighbor_idx] > 0.0,
            "Neighbor should receive chemical via diffusion: {}",
            sys.v[neighbor_idx]
        );
    }

    #[test]
    fn test_reaction() {
        let mut sys = ChemicalSystem::new(10, 10);
        // Setup for reaction: needs both U and V
        // U is 1.0 by default. Add V.
        sys.add_chemical(5, 5, 0.5);

        let idx = sys.get_index(5, 5);
        let initial_u = sys.u[idx];
        let initial_v = sys.v[idx];

        sys.update(1.0);

        assert_ne!(
            sys.u[idx], initial_u,
            "U concentration should change due to reaction"
        );
        assert_ne!(
            sys.v[idx], initial_v,
            "V concentration should change due to reaction"
        );
    }
}
