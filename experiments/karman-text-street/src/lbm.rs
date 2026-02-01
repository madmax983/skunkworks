use rayon::prelude::*;

const Q: usize = 9;
const W: [f32; Q] = [
    4.0 / 9.0,
    1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0, 1.0 / 9.0,
    1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0, 1.0 / 36.0,
];

// Directions: 0:C, 1:E, 2:N, 3:W, 4:S, 5:NE, 6:NW, 7:SW, 8:SE
const CX: [i32; Q] = [0, 1, 0, -1, 0, 1, -1, -1, 1];
const CY: [i32; Q] = [0, 0, 1, 0, -1, 1, 1, -1, -1];
const OPPOSITE: [usize; Q] = [0, 3, 4, 1, 2, 7, 8, 5, 6];

#[derive(Clone)]
pub struct Fluid {
    pub width: usize,
    pub height: usize,
    cells: Vec<f32>,     // flattened [width * height * Q]
    new_cells: Vec<f32>, // double buffer
    pub obstacles: Vec<bool>, // flattened [width * height]
    pub rho: Vec<f32>,   // cache for viz
    pub u_x: Vec<f32>,   // cache for viz
    pub u_y: Vec<f32>,   // cache for viz
    tau: f32,            // relaxation time
}

impl Fluid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let cells = vec![0.0; size * Q];
        let mut f = Self {
            width,
            height,
            cells: cells.clone(),
            new_cells: cells,
            obstacles: vec![false; size],
            rho: vec![1.0; size],
            u_x: vec![0.0; size],
            u_y: vec![0.0; size],
            tau: 0.6, // Viscosity related to tau. tau > 0.5. 0.6 is low viscosity (turbulent).
        };
        f.init_equilibrium();
        f
    }

    fn init_equilibrium(&mut self) {
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                // Initial flow: slight rightward velocity to start things off?
                // Or just still. Let's do u = 0.1, 0.0
                let u_x = 0.1;
                let u_y = 0.0;
                let rho = 1.0;
                let u2 = u_x * u_x + u_y * u_y;

                for i in 0..Q {
                    let eu = (CX[i] as f32) * u_x + (CY[i] as f32) * u_y;
                    let feq = W[i] * rho * (1.0 + 3.0 * eu + 4.5 * eu * eu - 1.5 * u2);
                    self.cells[idx * Q + i] = feq;
                }
            }
        }
    }

    pub fn step(&mut self) {
        let width = self.width;
        let height = self.height;
        let tau = self.tau;

        // Parallelize over rows
        // We need to write to new_cells based on cells.
        // STREAMING + COLLISION combined step?
        // Standard LBM:
        // 1. Stream: f_i(x+e_i) = f_i(x) (post-collision)
        // 2. Collide: f_i = f_i - (f_i - f_eq)/tau

        // It is easier to pull: f_i_new(x) comes from f_i(x - e_i)

        let obstacles = &self.obstacles;
        let cells = &self.cells;

        // We use `par_chunks_mut` to update `new_cells` in parallel.
        // Each chunk is one row (width * Q floats).
        self.new_cells
            .par_chunks_mut(width * Q)
            .enumerate()
            .for_each(|(y, row_slice)| {
                for x in 0..width {
                    let idx = y * width + x; // This idx is global if we didn't use chunks, but here we just need x.
                    // Actually, let's just use global index mapping for simplicity inside the loop.
                    let current_idx = idx;
                    let is_solid = obstacles[current_idx];

                    // Macroscopic variables at this cell (needed for collision)
                    // Wait, we need to compute macroscopic from the STREAMED particles arriving here?
                    // Standard algorithm:
                    // 1. Stream: f_in[i] = cells[(x-cx, y-cy)][i]
                    // 2. Compute rho, u from f_in
                    // 3. Collide: f_out[i] = f_in[i] - (f_in[i] - feq[i])/tau

                    if is_solid {
                         // Solid cell. Keep previous values to avoid garbage.
                         for i in 0..Q {
                            row_slice[x * Q + i] = cells[current_idx * Q + i];
                        }
                    } else {
                        // Let's proceed with "Pull" streaming for Fluid cells.
                        let mut f_in = [0.0; Q];
                        let mut rho = 0.0;
                        let mut u_x = 0.0;
                        let mut u_y = 0.0;

                        for i in 0..Q {
                            // Where is the particle coming FROM?
                            // We are at (x,y). Particle i moves by (cx, cy).
                            // So it comes from (x - cx, y - cy).
                            let src_x = (x as i32 - CX[i]) as isize;
                            let src_y = (y as i32 - CY[i]) as isize;

                            let val;

                            // Periodic boundaries or hard walls?
                            // Let's do Periodic for Left/Right (Wind Tunnel), Hard for Top/Bottom.

                            let valid_src_x = if src_x < 0 {
                                (width as isize + src_x) as usize
                            } else if src_x >= width as isize {
                                (src_x - width as isize) as usize
                            } else {
                                src_x as usize
                            };

                            if src_y >= 0 && src_y < height as isize {
                                let src_idx = (src_y as usize) * width + valid_src_x;

                                if obstacles[src_idx] {
                                    // Neighbor is solid. Bounce-back.
                                    // We get the particle that was going INTO the wall.
                                    // That is direction OPPOSITE[i].
                                    // And it comes from HERE (x,y).
                                    // Wait, this is the "Push" vs "Pull" confusion.

                                    // Let's stick to standard:
                                    // f_new[i](x) comes from f[i](x-e_i).
                                    // If (x-e_i) is solid, then f_new[i](x) = f[OPPOSITE[i]](x).
                                    // i.e., the particle went from x TO wall, hit it, and came back as i.
                                    // So we read from SELF at index OPPOSITE[i].

                                    val = cells[current_idx * Q + OPPOSITE[i]];
                                } else {
                                    // Neighbor is fluid. Normal streaming.
                                    val = cells[src_idx * Q + i];
                                }
                            } else {
                                // Top/Bottom boundary.
                                // Let's assume equilibrium or bounce back?
                                // Bounce back for top/bottom walls.
                                val = cells[current_idx * Q + OPPOSITE[i]];
                            }

                            f_in[i] = val;
                            rho += val;
                            u_x += val * CX[i] as f32;
                            u_y += val * CY[i] as f32;
                        }

                        // Macroscopic update
                        if rho > 0.0 {
                            u_x /= rho;
                            u_y /= rho;
                        }

                        // Inlet boundary condition (Left side)
                        // Force velocity at x=0?
                        if x == 0 {
                            u_x = 0.1;
                            u_y = 0.0;
                            rho = 1.0;
                            // Reset f_in to equilibrium for inlet
                             let u2 = u_x * u_x + u_y * u_y;
                             for i in 0..Q {
                                let eu = (CX[i] as f32) * u_x + (CY[i] as f32) * u_y;
                                f_in[i] = W[i] * rho * (1.0 + 3.0 * eu + 4.5 * eu * eu - 1.5 * u2);
                             }
                        }

                        // Collision
                        let u2 = u_x * u_x + u_y * u_y;
                        for i in 0..Q {
                            let eu = (CX[i] as f32) * u_x + (CY[i] as f32) * u_y;
                            let feq = W[i] * rho * (1.0 + 3.0 * eu + 4.5 * eu * eu - 1.5 * u2);

                            // Write to new buffer
                            row_slice[x * Q + i] = f_in[i] - (f_in[i] - feq) / tau;
                        }
                    }
                }
            });

        // Swap buffers
        std::mem::swap(&mut self.cells, &mut self.new_cells);

        // Update cache (rho, u)
        self.update_cache();
    }

    fn update_cache(&mut self) {
        let cells = &self.cells;
        let obstacles = &self.obstacles;

        // Parallel update of cache
        // We need to zip iterators or use chunks
        let size = self.width * self.height;

        // Sequential cache update for simplicity
        for idx in 0..size {
             if obstacles[idx] {
                 self.rho[idx] = 0.0;
                 self.u_x[idx] = 0.0;
                 self.u_y[idx] = 0.0;
                 continue;
             }
             let mut r = 0.0;
             let mut ux = 0.0;
             let mut uy = 0.0;
             for i in 0..Q {
                let val = cells[idx * Q + i];
                r += val;
                ux += val * CX[i] as f32;
                uy += val * CY[i] as f32;
             }
             self.rho[idx] = r;
             if r > 0.0 {
                 self.u_x[idx] = ux / r;
                 self.u_y[idx] = uy / r;
             }
        }
    }

    pub fn get_curl(&self, x: usize, y: usize) -> f32 {
        if x == 0 || x == self.width - 1 || y == 0 || y == self.height - 1 {
            return 0.0;
        }
        // idx is not needed
        let idx_n = (y + 1) * self.width + x;
        let idx_s = (y - 1) * self.width + x;
        let idx_e = y * self.width + x + 1;
        let idx_w = y * self.width + x - 1;

        let uy_e = self.u_y[idx_e];
        let uy_w = self.u_y[idx_w];
        let ux_n = self.u_x[idx_n];
        let ux_s = self.u_x[idx_s];

        // Curl = dUy/dx - dUx/dy
        // Finite difference
        (uy_e - uy_w) / 2.0 - (ux_n - ux_s) / 2.0
    }

    pub fn add_obstacle(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            self.obstacles[y * self.width + x] = true;
        }
    }

    pub fn clear_obstacles(&mut self) {
        self.obstacles.fill(false);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lbm_stability() {
        let mut fluid = Fluid::new(50, 50);
        // Run for 100 steps
        for _ in 0..100 {
            fluid.step();
        }

        // Check for NaN
        for &v in &fluid.rho {
            assert!(!v.is_nan());
        }
    }

    #[test]
    fn test_obstacle_bounce() {
        let mut fluid = Fluid::new(10, 10);
        fluid.add_obstacle(5, 5);
        assert!(fluid.obstacles[5 * 10 + 5]);

        fluid.step();
        // Just ensure it doesn't panic
    }
}
