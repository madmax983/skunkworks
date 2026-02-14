use rayon::prelude::*;

pub const WIDTH: usize = 120;
pub const HEIGHT: usize = 60;
const N_CELLS: usize = WIDTH * HEIGHT;
const VISCOSITY: f32 = 0.02;

// D2Q9 constants
const N_DIRS: usize = 9;
const W0: f32 = 4.0 / 9.0;
const W1: f32 = 1.0 / 9.0;
const W2: f32 = 1.0 / 36.0;

const WEIGHTS: [f32; 9] = [W0, W1, W1, W1, W1, W2, W2, W2, W2];
const DIRS_X: [i32; 9] = [0, 1, 0, -1, 0, 1, -1, -1, 1];
const DIRS_Y: [i32; 9] = [0, 0, 1, 0, -1, 1, 1, -1, -1];
// Inverse directions (bounce-back)
const INV_DIRS: [usize; 9] = [0, 3, 4, 1, 2, 7, 8, 5, 6];
// Y-Flip mapping for Klein twist
// 0(0,0)->0, 1(1,0)->1, 2(0,1)->4(0,-1), 3(-1,0)->3, 4(0,-1)->2(0,1)
// 5(1,1)->8(1,-1), 6(-1,1)->7(-1,-1), 7(-1,-1)->6(-1,1), 8(1,-1)->5(1,1)
const Y_FLIP: [usize; 9] = [0, 1, 4, 3, 2, 8, 7, 6, 5];

#[derive(Clone)]
pub struct FluidSim {
    f: Vec<f32>,
    f_next: Vec<f32>,
    pub obstacles: Vec<bool>,
    pub density: Vec<f32>,
    pub velocity_x: Vec<f32>,
    pub velocity_y: Vec<f32>,
    pub curl: Vec<f32>,
}

impl FluidSim {
    pub fn new() -> Self {
        let size = N_CELLS * N_DIRS;
        let mut f = vec![0.0; size];
        let obstacles = vec![false; N_CELLS];

        for i in 0..N_CELLS {
            for k in 0..N_DIRS {
                f[i * N_DIRS + k] = WEIGHTS[k];
            }
        }

        Self {
            f: f.clone(),
            f_next: f,
            obstacles,
            density: vec![1.0; N_CELLS],
            velocity_x: vec![0.0; N_CELLS],
            velocity_y: vec![0.0; N_CELLS],
            curl: vec![0.0; N_CELLS],
        }
    }

    pub fn step(&mut self) {
        // Split borrows for parallel loop
        let f = &self.f;
        let obstacles = &self.obstacles;
        let f_next = &mut self.f_next;

        // Collision + Streaming step (Pull scheme)
        f_next
            .par_chunks_mut(N_DIRS)
            .enumerate()
            .for_each(|(idx, cell_next)| {
                let x = (idx % WIDTH) as i32;
                let y = (idx / WIDTH) as i32;
                let is_solid = obstacles[idx];

                // Streaming: Pull from neighbors
                let mut rho = 0.0;
                let mut ux = 0.0;
                let mut uy = 0.0;
                let mut f_in = [0.0; 9];

                for k in 0..9 {
                    let mut nx = x - DIRS_X[k];
                    let mut ny = y - DIRS_Y[k];
                    let mut source_k = k; // By default, read same direction component

                    // Handle Klein Twist on X-Boundaries
                    if nx < 0 {
                        nx = (WIDTH - 1) as i32;
                        ny = (HEIGHT - 1) as i32 - ny;
                        source_k = Y_FLIP[k];
                    } else if nx >= WIDTH as i32 {
                        nx = 0;
                        ny = (HEIGHT - 1) as i32 - ny;
                        source_k = Y_FLIP[k];
                    }

                    // Handle Periodic on Y-Boundaries
                    if ny < 0 {
                        ny += HEIGHT as i32;
                    } else if ny >= HEIGHT as i32 {
                        ny -= HEIGHT as i32;
                    }

                    let n_idx = (ny as usize) * WIDTH + (nx as usize);
                    let val;

                    if obstacles[n_idx] {
                         // Bounce-back from obstacle
                         val = f[idx * N_DIRS + INV_DIRS[k]];
                    } else {
                        // Stream from neighbor
                        val = f[n_idx * N_DIRS + source_k];
                    }

                    f_in[k] = val;
                    rho += val;
                    // For macroscopic velocity, we use local directions
                    ux += val * DIRS_X[k] as f32;
                    uy += val * DIRS_Y[k] as f32;
                }

                if is_solid {
                    for k in 0..9 {
                        cell_next[k] = WEIGHTS[k];
                    }
                } else {
                    if rho > 0.0 {
                        ux /= rho;
                        uy /= rho;
                    } else {
                        ux = 0.0;
                        uy = 0.0;
                    }

                    // Collision (BGK)
                    let omega = 1.0 / (3.0 * VISCOSITY + 0.5);
                    let u2 = ux * ux + uy * uy;

                    for k in 0..9 {
                        let cu = DIRS_X[k] as f32 * ux + DIRS_Y[k] as f32 * uy;
                        let f_eq = rho * WEIGHTS[k] * (1.0 + 3.0 * cu + 4.5 * cu * cu - 1.5 * u2);
                        cell_next[k] = f_in[k] + omega * (f_eq - f_in[k]);
                    }
                }
            });

        // Swap buffers
        std::mem::swap(&mut self.f, &mut self.f_next);

        // Update macroscopic variables
        let f = &self.f;
        let obstacles = &self.obstacles;
        let density = &mut self.density;
        let velocity_x = &mut self.velocity_x;
        let velocity_y = &mut self.velocity_y;

        density
            .par_iter_mut()
            .zip(velocity_x.par_iter_mut())
            .zip(velocity_y.par_iter_mut())
            .zip(obstacles.par_iter())
            .enumerate()
            .for_each(|(idx, (((rho_out, ux_out), uy_out), &is_solid))| {
                if is_solid {
                    *rho_out = 0.0;
                    *ux_out = 0.0;
                    *uy_out = 0.0;
                } else {
                    let mut rho = 0.0;
                    let mut ux = 0.0;
                    let mut uy = 0.0;
                    let offset = idx * N_DIRS;
                    for k in 0..9 {
                        let val = f[offset + k];
                        rho += val;
                        ux += val * DIRS_X[k] as f32;
                        uy += val * DIRS_Y[k] as f32;
                    }
                    *rho_out = rho;
                    if rho > 0.0 {
                        *ux_out = ux / rho;
                        *uy_out = uy / rho;
                    } else {
                        *ux_out = 0.0;
                        *uy_out = 0.0;
                    }
                }
            });

        // Compute Curl
        // We need neighbors again. Since we updated density/velocity, we can use them directly.
        // We need random access to velocity_x/y, so we can't iterate simply.
        // We'll use unsafe or just iterate indices with slice access.
        let vx = &self.velocity_x;
        let vy = &self.velocity_y;
        let curl = &mut self.curl;

        curl.par_iter_mut().enumerate().for_each(|(idx, c)| {
            let x = (idx % WIDTH) as i32;
            let y = (idx / WIDTH) as i32;

            // Neighbors for curl
            let idx_r = if x < (WIDTH - 1) as i32 { idx + 1 } else { (y as usize) * WIDTH }; // Wrap X (simplistic)
            let idx_l = if x > 0 { idx - 1 } else { (y as usize) * WIDTH + WIDTH - 1 };

            let idx_d = if y < (HEIGHT - 1) as i32 { idx + WIDTH } else { idx % WIDTH  }; // Wrap Y
            let idx_u = if y > 0 { idx - WIDTH } else { (HEIGHT - 1) * WIDTH + (idx % WIDTH) };

            // Note: Curl calculation doesn't strictly need to follow Klein topology for visualization
            // unless we want it perfect at seams.
            // But let's just use simple periodic for curl to avoid complexity here,
            // the fluid motion itself respects the topology.
            // Actually, if velocity flips at boundary, curl should also flip?
            // Curl is a pseudo-vector. If Y flips, dy becomes -dy. uy becomes -uy.
            // curl = dv/dx - du/dy.
            // At boundary, v flips sign, y flips direction.
            // It's complex. Let's stick to local difference.

            let dv_dx = (vy[idx_r] - vy[idx_l]) * 0.5;
            let du_dy = (vx[idx_d] - vx[idx_u]) * 0.5;

            *c = dv_dx - du_dy;
        });
    }

    pub fn add_density(&mut self, x: usize, y: usize, amount: f32) {
        for dy in 0..3 {
            for dx in 0..3 {
                let px = x + dx;
                let py = y + dy;
                if px < WIDTH && py < HEIGHT {
                    let idx = py * WIDTH + px;
                    if !self.obstacles[idx] {
                        for k in 0..9 {
                            self.f[idx * N_DIRS + k] += amount * WEIGHTS[k];
                        }
                    }
                }
            }
        }
    }

    pub fn add_velocity(&mut self, x: usize, y: usize, amount_x: f32, amount_y: f32) {
        for dy in 0..3 {
            for dx in 0..3 {
                let px = x + dx;
                let py = y + dy;
                if px < WIDTH && py < HEIGHT {
                    let idx = py * WIDTH + px;
                    if !self.obstacles[idx] {
                        let rho = self.density[idx];
                        for k in 0..9 {
                            let cu = DIRS_X[k] as f32 * amount_x + DIRS_Y[k] as f32 * amount_y;
                            self.f[idx * N_DIRS + k] += 3.0 * rho * WEIGHTS[k] * cu;
                        }
                    }
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conservation_of_mass() {
        let mut sim = FluidSim::new();
        // Add some density
        sim.add_density(WIDTH / 2, HEIGHT / 2, 10.0);

        // Step once to incorporate added density into macroscopic variables
        sim.step();
        let mass1: f32 = sim.density.iter().sum();

        sim.step();
        let mass2: f32 = sim.density.iter().sum();

        // Allow small floating point error
        assert!(
            (mass1 - mass2).abs() < 0.1,
            "Mass not conserved: {} vs {}",
            mass1,
            mass2
        );
    }

    #[test]
    fn test_boundary_crossing() {
        let mut sim = FluidSim::new();
        // Add velocity towards right edge at the top
        // x = WIDTH-10 to avoid clipping
        sim.add_velocity(WIDTH - 10, 10, 1.0, 0.0);
        sim.add_density(WIDTH - 10, 10, 50.0); // Large density pulse

        // Step enough to cross boundary (10 steps for 10 distance + buffer)
        for _ in 0..20 {
            sim.step();
        }

        // Check if density appeared on the Left side at flipped Y
        // y_flipped = HEIGHT - 1 - 10 = 49 (if HEIGHT=60)
        // We expect density near (0, 49).

        let mut max_rho = 0.0;
        for y in 40..60 { // Broad search
            for x in 0..10 {
                let rho = sim.density[y * WIDTH + x];
                if rho > max_rho { max_rho = rho; }
            }
        }

        // Base mass is 1.0. With 50.0 added, we expect significant bump.
        assert!(max_rho > 1.5, "Density did not cross twisted boundary. Max rho in target area: {}", max_rho);
    }
}
