use rayon::prelude::*;

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<f32>,
    pub params_r: Vec<f32>,
    pub buffer: Vec<f32>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            cells: vec![0.0; size],
            params_r: vec![0.0; size],
            buffer: vec![0.0; size],
        }
    }

    pub fn get_idx(&self, x: isize, y: isize) -> usize {
        let w = self.width as isize;
        let h = self.height as isize;
        let x = (x % w + w) % w;
        let y = (y % h + h) % h;
        (y * w + x) as usize
    }

    pub fn update(&mut self, epsilon: f32) {
        let width_isize = self.width as isize;
        let height_isize = self.height as isize;
        let width_usize = self.width; // Capture Copy type
        let cells = &self.cells;
        let params = &self.params_r;

        self.buffer
            .par_iter_mut()
            .enumerate()
            .for_each(|(i, new_val)| {
                let x = (i % width_usize) as isize;
                let y = (i / width_usize) as isize;

                // Helper to get value safely with wrapping
                let get = |dx: isize, dy: isize| {
                    let nx = (x + dx + width_isize) % width_isize;
                    let ny = (y + dy + height_isize) % height_isize;
                    let idx = (ny * width_isize + nx) as usize;
                    cells[idx]
                };

                let get_r = |dx: isize, dy: isize| {
                    let nx = (x + dx + width_isize) % width_isize;
                    let ny = (y + dy + height_isize) % height_isize;
                    let idx = (ny * width_isize + nx) as usize;
                    params[idx]
                };

                let val = cells[i];
                let r = params[i];

                // Logistic map function f(x) = r * x * (1 - x)
                let f = |v: f32, r: f32| r * v * (1.0 - v);

                let self_term = f(val, r);

                // Von Neumann neighborhood (Up, Down, Left, Right)
                // We use the neighbor's r for their own update, so when reading them, we should technically simulate their f(x).
                // The CML equation says sum of f(x_neighbor).
                // Does f(x_neighbor) use neighbor's r or my r? Usually neighbor's r (it's their output).

                // Let's assume f(x) uses the r of the cell where x resides.
                let n_up = get(0, -1);
                let r_up = get_r(0, -1);
                let term_up = f(n_up, r_up);

                let n_down = get(0, 1);
                let r_down = get_r(0, 1);
                let term_down = f(n_down, r_down);

                let n_left = get(-1, 0);
                let r_left = get_r(-1, 0);
                let term_left = f(n_left, r_left);

                let n_right = get(1, 0);
                let r_right = get_r(1, 0);
                let term_right = f(n_right, r_right);

                let neighbor_sum = term_up + term_down + term_left + term_right;

                *new_val = (1.0 - epsilon) * self_term + (epsilon / 4.0) * neighbor_sum;
            });

        // Swap buffer
        std::mem::swap(&mut self.cells, &mut self.buffer);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_update() {
        let width = 10;
        let height = 10;
        let mut grid = Grid::new(width, height);

        // Initialize with r=3.0 (stable period 2 eventually, but initially just growth)
        // and x=0.5
        for i in 0..grid.cells.len() {
            grid.cells[i] = 0.1;
            grid.params_r[i] = 2.0;
        }

        // With r=2.0, x=0.1 -> f(x) = 2 * 0.1 * 0.9 = 0.18
        // If epsilon=0, next x should be 0.18 everywhere.

        grid.update(0.0);

        for val in grid.cells.iter() {
            assert!(
                (val - 0.18).abs() < 1e-6,
                "Value should be 0.18, got {}",
                val
            );
        }
    }

    #[test]
    fn test_coupling() {
        let width = 3;
        let height = 3;
        let mut grid = Grid::new(width, height);

        // Center cell
        let center_idx = grid.get_idx(1, 1);
        grid.cells[center_idx] = 0.5;
        grid.params_r[center_idx] = 4.0; // f(0.5) = 4*0.5*0.5 = 1.0

        // Neighbors 0.0
        // If epsilon = 1.0, center cell should become average of neighbors (0.0).
        // Wait, neighbor's f(x) is f(0) = 0.
        // So center should become 0.0.

        grid.update(1.0);

        assert_eq!(grid.cells[center_idx], 0.0);
    }
}
