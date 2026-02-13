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
        }
    }

    pub fn step(&mut self) {
        // Destructure to avoid borrow checker conflicts
        let FluidSim {
            f,
            f_next,
            obstacles,
            density,
            velocity_x,
            velocity_y,
        } = self;

        // Collision + Streaming step
        f_next.par_chunks_mut(N_DIRS).enumerate().for_each(|(idx, cell_next)| {
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
                        val = f[idx * N_DIRS + INV_DIRS[k]];
                    } else {
                        val = f[n_idx * N_DIRS + k];
                    }
                } else {
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

                let omega = 1.0 / (3.0 * VISCOSITY + 0.5);
                let u2 = ux * ux + uy * uy;

                for k in 0..9 {
                    let cu = DIRS_X[k] as f32 * ux + DIRS_Y[k] as f32 * uy;
                    let f_eq = rho * WEIGHTS[k] * (1.0 + 3.0 * cu + 4.5 * cu * cu - 1.5 * u2);
                    cell_next[k] = f_in[k] + omega * (f_eq - f_in[k]);
                }
            }
        });

        std::mem::swap(f, f_next);

        density.par_iter_mut()
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

    pub fn set_obstacle(&mut self, x: usize, y: usize, active: bool) {
        if x < WIDTH && y < HEIGHT {
            self.obstacles[y * WIDTH + x] = active;
        }
    }

    pub fn _clear_obstacles(&mut self) {
        self.obstacles.fill(false);
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

        // Float precision might cause drift, but should be small
        // With density initialized to 1.0, sum is N_CELLS = 20000.
        // Step shouldn't change it much.
        assert!((initial_mass - final_mass).abs() < 10.0, "Mass not conserved: {} vs {}", initial_mass, final_mass);
    }

    #[test]
    fn test_obstacle_setting() {
        let mut sim = FluidSim::new();
        sim.set_obstacle(10, 10, true);
        assert!(sim.obstacles[10 * WIDTH + 10]);
    }
}
