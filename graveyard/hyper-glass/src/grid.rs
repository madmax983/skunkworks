use rand::Rng;
use std::f32::consts::PI;

pub struct SpinGrid4D {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub hypersize: usize,
    pub spins: Vec<f32>, // Angle theta [0, 2PI)
}

impl SpinGrid4D {
    pub fn new(size: usize) -> Self {
        let total = size * size * size * size;
        let mut rng = rand::thread_rng();
        Self {
            width: size,
            height: size,
            depth: size,
            hypersize: size,
            spins: (0..total).map(|_| rng.gen::<f32>() * 2.0 * PI).collect(),
        }
    }

    #[inline(always)]
    pub fn idx(&self, x: usize, y: usize, z: usize, w: usize) -> usize {
        w * (self.width * self.height * self.depth)
            + z * (self.width * self.height)
            + y * self.width
            + x
    }

    pub fn step_metropolis(&mut self, temp: f32, field_strength: f32) {
        let mut rng = rand::thread_rng();
        let inv_t = if temp > 1e-4 { 1.0 / temp } else { 10000.0 };

        // For performance, we sample random sites rather than full sweep if grid is large
        // But 8^4 = 4096 is small enough for a full sweep.

        let w = self.width;
        let h = self.height;
        let d = self.depth;
        let hs = self.hypersize;

        // XY Model Hamiltonian: H = -J * sum(cos(theta_i - theta_j)) - h * sum(cos(theta_i))
        // Delta E = E_new - E_old
        // Only local interactions change.

        // Full sweep checkerboard update might be better for parallel, but sequential is fine here.
        for _ in 0..(self.spins.len()) {
            // Pick random site
            let idx = rng.gen_range(0..self.spins.len());

            // Calculate coordinate from index
            // But we can just use index arithmetic relative to strides
            // Need coords to find neighbors with wrapping? Or clamp?
            // Let's implement simple wrapping for now (Toroidal Hypercube).

            let x = idx % w;
            let y = (idx / w) % h;
            let z = (idx / (w * h)) % d;
            let _w_coord = idx / (w * h * d); // Not needed for neighbors if we use coords

            // Propose new angle: small random rotation
            // let delta_theta = (rng.gen::<f32>() - 0.5) * PI; // Large jumps
            let delta_theta = (rng.gen::<f32>() - 0.5) * 0.5; // Small jumps (Metropolis)
                                                              // Or just pick a completely new angle (Heat Bath / Metropolis)
                                                              // For XY model, usually small steps or random new angle.
                                                              // Let's try random new angle for faster exploration.
            let old_theta = self.spins[idx];
            let new_theta = old_theta + delta_theta;
            // Normalize? cos handles it.

            // Calculate Energy of neighbors
            let mut neighbor_sum_cos = 0.0;
            let mut neighbor_sum_sin = 0.0;

            // X Neighbors
            let n_x_p = self.spins[self.idx((x + 1) % w, y, z, _w_coord)];
            let n_x_m = self.spins[self.idx((x + w - 1) % w, y, z, _w_coord)];

            // Y Neighbors
            let n_y_p = self.spins[self.idx(x, (y + 1) % h, z, _w_coord)];
            let n_y_m = self.spins[self.idx(x, (y + h - 1) % h, z, _w_coord)];

            // Z Neighbors
            let n_z_p = self.spins[self.idx(x, y, (z + 1) % d, _w_coord)];
            let n_z_m = self.spins[self.idx(x, y, (z + d - 1) % d, _w_coord)];

            // W Neighbors
            let n_w_p = self.spins[self.idx(x, y, z, (_w_coord + 1) % hs)];
            let n_w_m = self.spins[self.idx(x, y, z, (_w_coord + hs - 1) % hs)];

            // Optimization: sum cos(theta_n) and sin(theta_n)
            // Energy = - cos(theta) * sum_cos - sin(theta) * sum_sin

            for &n in &[n_x_p, n_x_m, n_y_p, n_y_m, n_z_p, n_z_m, n_w_p, n_w_m] {
                neighbor_sum_cos += n.cos();
                neighbor_sum_sin += n.sin();
            }

            let e_old = -(old_theta.cos() * neighbor_sum_cos + old_theta.sin() * neighbor_sum_sin)
                - field_strength * old_theta.cos(); // Field aligns to 0

            let e_new = -(new_theta.cos() * neighbor_sum_cos + new_theta.sin() * neighbor_sum_sin)
                - field_strength * new_theta.cos();

            let d_e = e_new - e_old;

            if d_e < 0.0 || rng.gen::<f32>() < (-d_e * inv_t).exp() {
                self.spins[idx] = new_theta;
            }
        }
    }
}
