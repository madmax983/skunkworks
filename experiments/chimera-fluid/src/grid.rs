use rayon::prelude::*;

pub struct Grid4D {
    pub width: usize,
    pub height: usize,
    pub depth: usize,
    pub hypersize: usize,
    pub cells: Vec<f32>,
    pub next_cells: Vec<f32>,
}

impl Grid4D {
    pub fn new(size: usize) -> Self {
        let total = size * size * size * size;
        Self {
            width: size,
            height: size,
            depth: size,
            hypersize: size,
            cells: vec![0.0; total],
            next_cells: vec![0.0; total],
        }
    }

    pub fn idx(&self, x: usize, y: usize, z: usize, w: usize) -> usize {
        // Linear index: w * (size^3) + z * (size^2) + y * size + x
        w * (self.width * self.height * self.depth)
            + z * (self.width * self.height)
            + y * self.width
            + x
    }

    pub fn add_val(&mut self, x: usize, y: usize, z: usize, w: usize, val: f32) {
        if x < self.width && y < self.height && z < self.depth && w < self.hypersize {
            let idx = self.idx(x, y, z, w);
            self.cells[idx] = (self.cells[idx] + val).min(1.0);
        }
    }

    pub fn get(&self, x: usize, y: usize, z: usize, w: usize) -> f32 {
        if x < self.width && y < self.height && z < self.depth && w < self.hypersize {
            self.cells[self.idx(x, y, z, w)]
        } else {
            0.0
        }
    }

    pub fn diffuse_and_decay(&mut self, decay_rate: f32, diffuse_rate: f32) {
        let w = self.width;
        let h = self.height;
        let d = self.depth;
        let hs = self.hypersize;

        let cells = &self.cells;
        let next = &mut self.next_cells;

        // Parallel iteration over W slices
        // Each W slice is a 3D volume
        next.par_chunks_mut(w * h * d)
            .enumerate()
            .for_each(|(wi, slice)| {
                for zi in 0..d {
                    for yi in 0..h {
                        for xi in 0..w {
                            let idx = zi * (w * h) + yi * w + xi; // Local index in slice
                            let global_idx = wi * (w * h * d) + idx; // For verifying logic, but we can just use xi, yi, zi, wi

                            // 4D Laplacian / Average
                            // Neighbors: +/- 1 in each dimension
                            let mut sum = 0.0;
                            let mut count = 0.0;

                            // X neighbors
                            if xi > 0 {
                                sum += cells[global_idx - 1];
                                count += 1.0;
                            }
                            if xi < w - 1 {
                                sum += cells[global_idx + 1];
                                count += 1.0;
                            }

                            // Y neighbors
                            if yi > 0 {
                                sum += cells[global_idx - w];
                                count += 1.0;
                            }
                            if yi < h - 1 {
                                sum += cells[global_idx + w];
                                count += 1.0;
                            }

                            // Z neighbors
                            let z_stride = w * h;
                            if zi > 0 {
                                sum += cells[global_idx - z_stride];
                                count += 1.0;
                            }
                            if zi < d - 1 {
                                sum += cells[global_idx + z_stride];
                                count += 1.0;
                            }

                            // W neighbors
                            let w_stride = w * h * d;
                            if wi > 0 {
                                sum += cells[global_idx - w_stride];
                                count += 1.0;
                            }
                            if wi < hs - 1 {
                                sum += cells[global_idx + w_stride];
                                count += 1.0;
                            }

                            let avg = if count > 0.0 {
                                sum / count
                            } else {
                                cells[global_idx]
                            };

                            // Apply diffusion and decay
                            let val = cells[global_idx];
                            let diffused = val * (1.0 - diffuse_rate) + avg * diffuse_rate;
                            slice[idx] = diffused * (1.0 - decay_rate);
                        }
                    }
                }
            });

        // Swap buffers
        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }
}
