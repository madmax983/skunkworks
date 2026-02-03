// 🧬 Allele A: Inherited from experiments/chem-sys
// Represents the Biological substrate (Gray-Scott Reaction Diffusion)

pub struct ChemicalSystem {
    pub width: usize,
    pub height: usize,
    pub u: Vec<f64>,
    pub v: Vec<f64>,
    next_u: Vec<f64>,
    next_v: Vec<f64>,
    pub f: f64,  // feed rate - will be driven by Lorenz
    pub k: f64,  // kill rate - will be driven by Lorenz
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
            let prev_y = if y == 0 { h - 1 } else { y - 1 };
            let next_y = if y == h - 1 { 0 } else { y + 1 };

            for x in 0..w {
                let prev_x = if x == 0 { w - 1 } else { x - 1 };
                let next_x = if x == w - 1 { 0 } else { x + 1 };

                // Indices
                let i = y * w + x;

                let i_up = prev_y * w + x;
                let i_down = next_y * w + x;
                let i_left = y * w + prev_x;
                let i_right = y * w + next_x;

                let i_ul = prev_y * w + prev_x;
                let i_ur = prev_y * w + next_x;
                let i_dl = next_y * w + prev_x;
                let i_dr = next_y * w + next_x;

                let u = self.u[i];
                let v = self.v[i];

                // Calculate Laplacian
                let lap_u = (self.u[i_up] + self.u[i_down] + self.u[i_left] + self.u[i_right])
                    * adj_w
                    + (self.u[i_ul] + self.u[i_ur] + self.u[i_dl] + self.u[i_dr]) * diag_w
                    + u * center_w;

                let lap_v = (self.v[i_up] + self.v[i_down] + self.v[i_left] + self.v[i_right])
                    * adj_w
                    + (self.v[i_ul] + self.v[i_ur] + self.v[i_dl] + self.v[i_dr]) * diag_w
                    + v * center_w;

                let uvv = u * v * v;

                // Gray-Scott formulas
                // du/dt = Du * lap_u - uv^2 + f * (1 - u)
                // dv/dt = Dv * lap_v + uv^2 - (f + k) * v

                let du_dt = self.du * lap_u - uvv + self.f * (1.0 - u);
                let dv_dt = self.dv * lap_v + uvv - (self.f + self.k) * v;

                self.next_u[i] = (u + du_dt * dt).clamp(0.0, 1.0);
                self.next_v[i] = (v + dv_dt * dt).clamp(0.0, 1.0);
            }
        }

        // Swap buffers
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
