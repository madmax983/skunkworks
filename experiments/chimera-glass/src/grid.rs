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

    // Get coordinates from index
    pub fn coords(&self, idx: usize) -> (usize, usize, usize, usize) {
        let w = self.width;
        let h = self.height;
        let d = self.depth;

        let x = idx % w;
        let y = (idx / w) % h;
        let z = (idx / (w * h)) % d;
        let w_coord = idx / (w * h * d);
        (x, y, z, w_coord)
    }

    pub fn get_spin(&self, idx: usize) -> f32 {
        self.spins[idx]
    }

    pub fn set_spin(&mut self, idx: usize, theta: f32) {
        if idx < self.spins.len() {
            self.spins[idx] = theta;
        }
    }

    pub fn calculate_energy_delta(&self, idx: usize, new_theta: f32, field_strength: f32) -> f32 {
        let (x, y, z, w_coord) = self.coords(idx);
        let w = self.width;
        let h = self.height;
        let d = self.depth;
        let hs = self.hypersize;

        // Neighbors
        let n_x_p = self.spins[self.idx((x + 1) % w, y, z, w_coord)];
        let n_x_m = self.spins[self.idx((x + w - 1) % w, y, z, w_coord)];
        let n_y_p = self.spins[self.idx(x, (y + 1) % h, z, w_coord)];
        let n_y_m = self.spins[self.idx(x, (y + h - 1) % h, z, w_coord)];
        let n_z_p = self.spins[self.idx(x, y, (z + 1) % d, w_coord)];
        let n_z_m = self.spins[self.idx(x, y, (z + d - 1) % d, w_coord)];
        let n_w_p = self.spins[self.idx(x, y, z, (w_coord + 1) % hs)];
        let n_w_m = self.spins[self.idx(x, y, z, (w_coord + hs - 1) % hs)];

        let mut neighbor_sum_cos = 0.0;
        let mut neighbor_sum_sin = 0.0;

        for &n in &[n_x_p, n_x_m, n_y_p, n_y_m, n_z_p, n_z_m, n_w_p, n_w_m] {
            neighbor_sum_cos += n.cos();
            neighbor_sum_sin += n.sin();
        }

        let old_theta = self.spins[idx];
        let e_old = -(old_theta.cos() * neighbor_sum_cos + old_theta.sin() * neighbor_sum_sin)
                    - field_strength * old_theta.cos();

        let e_new = -(new_theta.cos() * neighbor_sum_cos + new_theta.sin() * neighbor_sum_sin)
                    - field_strength * new_theta.cos();

        e_new - e_old
    }

    pub fn step_metropolis(&mut self, temp: f32, field_strength: f32) {
        let mut rng = rand::thread_rng();
        let inv_t = if temp > 1e-4 { 1.0 / temp } else { 10000.0 };

        for _ in 0..(self.spins.len()) {
            let idx = rng.gen_range(0..self.spins.len());
            let delta_theta = (rng.gen::<f32>() - 0.5) * 0.5;
            let old_theta = self.spins[idx];
            let new_theta = old_theta + delta_theta;

            let d_e = self.calculate_energy_delta(idx, new_theta, field_strength);

            if d_e < 0.0 || rng.gen::<f32>() < (-d_e * inv_t).exp() {
                self.spins[idx] = new_theta;
            }
        }
    }
}
