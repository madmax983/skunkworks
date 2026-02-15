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

const GRAVITY_G: f32 = 0.5; // Tunable gravity constant for fluid interaction

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

    pub fn step(&mut self, bodies: &[(f32, f32, f32)]) {
        let FluidSim {
            f,
            f_next,
            obstacles,
            density,
            velocity_x,
            velocity_y,
            curl,
        } = self;

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

                    // Apply Gravity Forces
                    // F = G * M / r^2
                    // u_eq = u + F * dt (dt=1)
                    let mut fx = 0.0;
                    let mut fy = 0.0;

                    for (bx, by, mass) in bodies {
                        let dx = *bx - x as f32;
                        let dy = *by - y as f32;
                        let dist_sq = dx * dx + dy * dy;
                        let dist = dist_sq.sqrt();

                        // Softening to avoid division by zero and extreme forces
                        let soft_dist_sq = dist_sq + 1.0;

                        // F = G * M / r^2 direction = (dx/r, dy/r)
                        // F_vec = (G * M / r^2) * (vec/r) = G*M*vec / r^3
                        // Using softened distance for magnitude
                        let force = GRAVITY_G * mass / (soft_dist_sq * dist.max(0.1));

                        fx += dx * force;
                        fy += dy * force;
                    }

                    // Add force to velocity for equilibrium calculation
                    // Force typically adds to momentum: rho * u += F * dt
                    // So u += F/rho * dt
                    // We just add it directly to u for simplicity in this visual simulation
                    if rho > 0.0 {
                         ux += fx / rho;
                         uy += fy / rho;
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
        std::mem::swap(f, f_next);

        // Update macroscopic variables
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

        // Compute Curl (Vorticity)
        let vx = &*velocity_x;
        let vy = &*velocity_y;

        curl.par_iter_mut().enumerate().for_each(|(idx, c)| {
            let x = (idx % WIDTH) as i32;
            let y = (idx / WIDTH) as i32;

            if x > 0 && x < (WIDTH - 1) as i32 && y > 0 && y < (HEIGHT - 1) as i32 {
                let idx_r = idx + 1;
                let idx_l = idx - 1;
                let idx_u = idx - WIDTH;
                let idx_d = idx + WIDTH;

                let dv_dx = (vy[idx_r] - vy[idx_l]) * 0.5;
                let du_dy = (vx[idx_d] - vx[idx_u]) * 0.5;

                *c = dv_dx - du_dy;
            } else {
                *c = 0.0;
            }
        });
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
