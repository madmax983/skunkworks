use rayon::prelude::*;

pub const WIDTH: usize = 200;
pub const HEIGHT: usize = 100;
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
// Inverse directions
const INV_DIRS: [usize; 9] = [0, 3, 4, 1, 2, 7, 8, 5, 6];

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
        // Destructure to access fields inside the closure/loop
        // We can't destructure &mut self easily for parallel iteration on multiple fields unless we split borrowing.
        // But we can just use `self.f`, `self.f_next` if we are careful.
        // Actually, the previous implementation did it well by using `let FluidSim { ... } = self`.
        // But since we added `curl`, we need to include it.

        let FluidSim {
            f,
            f_next,
            obstacles,
            density,
            velocity_x,
            velocity_y,
            curl,
        } = self;

        // Collision + Streaming step (Pull scheme) + Macroscopic Update
        f_next
            .par_chunks_mut(N_DIRS)
            .zip(density.par_iter_mut())
            .zip(velocity_x.par_iter_mut())
            .zip(velocity_y.par_iter_mut())
            .enumerate()
            .for_each(|(idx, (((cell_next, rho_out), ux_out), uy_out))| {
                let x = (idx % WIDTH) as i32;
                let y = (idx / WIDTH) as i32;
                let is_solid = obstacles[idx];

                // Streaming: Pull from neighbors
                let mut rho = 0.0;
                let mut ux = 0.0;
                let mut uy = 0.0;
                let mut f_in = [0.0; 9];

                for k in 0..9 {
                    let nx = x - DIRS_X[k];
                    let ny = y - DIRS_Y[k];

                    let val;
                    if nx >= 0 && nx < WIDTH as i32 && ny >= 0 && ny < HEIGHT as i32 {
                        let n_idx = (ny as usize) * WIDTH + (nx as usize);
                        if obstacles[n_idx] {
                            // Bounce-back from obstacle
                            val = f[idx * N_DIRS + INV_DIRS[k]];
                        } else {
                            // Stream from neighbor
                            val = f[n_idx * N_DIRS + k];
                        }
                    } else {
                        // Bounce-back from boundary
                        val = f[idx * N_DIRS + INV_DIRS[k]];
                    }
                    f_in[k] = val;
                    rho += val;
                    ux += val * DIRS_X[k] as f32;
                    uy += val * DIRS_Y[k] as f32;
                }

                if is_solid {
                    // Solid node: bounce-back logic for next step?
                    // Standard bounce-back: f_i(x, t+1) = f_{-i}(x, t)
                    // But here we are updating f_next (t+1).
                    // The "Pull" scheme handles bounce-back by reading from self via INV_DIRS in the neighbor check.
                    // So if *this* node is solid, it doesn't really matter what we write to it,
                    // unless it becomes non-solid later.
                    // Let's just set it to equilibrium at zero velocity.
                    cell_next.copy_from_slice(&WEIGHTS);
                    *rho_out = 0.0;
                    *ux_out = 0.0;
                    *uy_out = 0.0;
                } else {
                    if rho > 0.0 {
                        ux /= rho;
                        uy /= rho;
                    } else {
                        ux = 0.0;
                        uy = 0.0;
                    }

                    // Write macroscopic variables
                    *rho_out = rho;
                    *ux_out = ux;
                    *uy_out = uy;

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
        std::mem::swap(f, f_next);

        // Compute Curl (Vorticity)
        // curl = dv/dx - du/dy
        // We need neighboring velocities.
        // This step cannot easily be parallelized in place if we read from self.velocity_x/y while writing to self.curl?
        // Actually, we read velocity (which is done) and write curl.
        // `velocity_x` and `velocity_y` are `Vec<f32>`, `curl` is `Vec<f32>`.
        // We can zip them or just iterate indices.
        // Since we need neighbors, we need random access to velocity.
        // So we can iterate `curl` mutably and read velocity immutably.
        // But inside `par_iter_mut` we can't capture `self` or `velocity_x`.
        // We need to slice them.
        let vx = &*velocity_x; // Immutable slice
        let vy = &*velocity_y; // Immutable slice

        curl.par_iter_mut().enumerate().for_each(|(idx, c)| {
            let x = (idx % WIDTH) as i32;
            let y = (idx / WIDTH) as i32;

            if x > 0 && x < (WIDTH - 1) as i32 && y > 0 && y < (HEIGHT - 1) as i32 {
                let idx_r = idx + 1;
                let idx_l = idx - 1;
                let idx_u = idx - WIDTH; // Up is smaller index (y-1) in this coordinate system? Wait.
                // Usually y=0 is top? In `main.rs` loop:
                // `y * WIDTH + x`.
                // If we treat index 0 as (0,0), then `idx - WIDTH` is (x, y-1).
                // So y increases downwards.
                // dy = 1.
                // curl = dv_x/dy - dv_y/dx?
                // Wait, standard 2D curl is (dv_y/dx - dv_x/dy).
                // v_x = u, v_y = v.
                // curl = dv/dx - du/dy.

                let idx_d = idx + WIDTH;

                let dv_dx = (vy[idx_r] - vy[idx_l]) * 0.5;
                let du_dy = (vx[idx_d] - vx[idx_u]) * 0.5;

                *c = dv_dx - du_dy;
            } else {
                *c = 0.0;
            }
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
                        let offset = idx * N_DIRS;
                        self.f[offset..offset + 9]
                            .iter_mut()
                            .zip(WEIGHTS.iter())
                            .for_each(|(f, &w)| *f += amount * w);
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

    #[allow(dead_code)]
    pub fn set_obstacle(&mut self, x: usize, y: usize, active: bool) {
        if x < WIDTH && y < HEIGHT {
            self.obstacles[y * WIDTH + x] = active;
        }
    }

    // Bilinear interpolation of velocity
    pub fn get_velocity(&self, x: f32, y: f32) -> (f32, f32) {
        let x = x.clamp(0.0, (WIDTH - 1) as f32);
        let y = y.clamp(0.0, (HEIGHT - 1) as f32);

        let x0 = x.floor() as usize;
        let y0 = y.floor() as usize;
        let x1 = (x0 + 1).min(WIDTH - 1);
        let y1 = (y0 + 1).min(HEIGHT - 1);

        let tx = x - x0 as f32;
        let ty = y - y0 as f32;

        let idx00 = y0 * WIDTH + x0;
        let idx10 = y0 * WIDTH + x1;
        let idx01 = y1 * WIDTH + x0;
        let idx11 = y1 * WIDTH + x1;

        let u00 = self.velocity_x[idx00];
        let u10 = self.velocity_x[idx10];
        let u01 = self.velocity_x[idx01];
        let u11 = self.velocity_x[idx11];

        let v00 = self.velocity_y[idx00];
        let v10 = self.velocity_y[idx10];
        let v01 = self.velocity_y[idx01];
        let v11 = self.velocity_y[idx11];

        let ux = (1.0 - tx) * (1.0 - ty) * u00
            + tx * (1.0 - ty) * u10
            + (1.0 - tx) * ty * u01
            + tx * ty * u11;

        let uy = (1.0 - tx) * (1.0 - ty) * v00
            + tx * (1.0 - ty) * v10
            + (1.0 - tx) * ty * v01
            + tx * ty * v11;

        (ux, uy)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_conservation_of_mass() {
        let mut sim = FluidSim::new();
        let initial_mass: f32 = sim.density.iter().sum();

        sim.step();

        let final_mass: f32 = sim.density.iter().sum();
        assert!(
            (initial_mass - final_mass).abs() < 10.0,
            "Mass not conserved: {} vs {}",
            initial_mass,
            final_mass
        );
    }

    #[test]
    fn test_get_velocity() {
        let mut sim = FluidSim::new();
        // Set some velocity
        sim.velocity_x[0] = 1.0;
        sim.velocity_x[1] = 2.0;
        sim.velocity_y[0] = 1.0; // v00
        sim.velocity_y[1] = 1.0; // v10
        // sim.velocity_y[WIDTH] is v01 (x=0, y=1)

        let (ux, uy) = sim.get_velocity(0.5, 0.0);
        assert!((ux - 1.5).abs() < 0.001);
        assert!((uy - 1.0).abs() < 0.001);
    }
}
