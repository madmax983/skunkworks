use rayon::prelude::*;
use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub load: u32,
    pub processed: u64, // Total processed tasks
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
    pub next_cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            cells: vec![Cell { load: 0, processed: 0 }; size],
            next_cells: vec![Cell { load: 0, processed: 0 }; size],
        }
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn add_load(&mut self, x: usize, y: usize, amount: u32) {
        if x < self.width && y < self.height {
            let idx = self.get_index(x, y);
            self.cells[idx].load += amount;
        }
    }

    pub fn update(&mut self, process_rate: f64) {
        let width = self.width;
        let height = self.height;

        // Use par_iter_mut for parallel update
        // We need to capture 'cells' as read-only slice
        let current_cells = &self.cells;

        self.next_cells.par_iter_mut().enumerate().for_each(|(i, next_cell)| {
            let x = i % width;
            let y = i / width;

            let mut my_load = current_cells[i].load;
            let mut my_processed = current_cells[i].processed;

            // 1. Toppling Logic (Deterministic based on current state)
            // If I have >= 4, I will topple in this step.
            if my_load >= 4 {
                my_load -= 4;
            }

            // 2. Inflow from neighbors (Deterministic based on neighbor's current state)
            let neighbors = [
                (x.wrapping_sub(1), y), // Left (wrapping handles < 0 check with usize but usually creates huge number, we need check)
                (x + 1, y),
                (x, y.wrapping_sub(1)),
                (x, y + 1),
            ];

            for (nx, ny) in neighbors {
                if nx < width && ny < height {
                    let n_idx = ny * width + nx;
                    // Neighbor topples if it had >= 4
                    if current_cells[n_idx].load >= 4 {
                        my_load += 1;
                    }
                }
            }

            // 3. Processing (Stochastic)
            // Apply processing to the *result* of the flow.
            // This ensures conservation of mass during the flow phase.
            // Mass is only lost here, explicitly.
            let mut rng = rand::thread_rng();
            if my_load > 0 && rng.gen_bool(process_rate) {
                my_load -= 1;
                my_processed += 1;
            }

            next_cell.load = my_load;
            next_cell.processed = my_processed;
        });

        // Swap buffers
        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }

    pub fn total_load(&self) -> u64 {
        self.cells.iter().map(|c| c.load as u64).sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_topple() {
        let mut grid = Grid::new(3, 3);
        // Center cell
        grid.add_load(1, 1, 4);

        assert_eq!(grid.cells[grid.get_index(1, 1)].load, 4);

        // Update with 0 processing rate to test pure sandpile physics
        grid.update(0.0);

        // Center should have lost 4 -> 0
        assert_eq!(grid.cells[grid.get_index(1, 1)].load, 0);

        // Neighbors should have gained 1
        assert_eq!(grid.cells[grid.get_index(0, 1)].load, 1); // Left
        assert_eq!(grid.cells[grid.get_index(2, 1)].load, 1); // Right
        assert_eq!(grid.cells[grid.get_index(1, 0)].load, 1); // Up
        assert_eq!(grid.cells[grid.get_index(1, 2)].load, 1); // Down

        // Corners remain 0
        assert_eq!(grid.cells[grid.get_index(0, 0)].load, 0);
    }

    #[test]
    fn test_conservation() {
        // 5x5 grid, put 4 in center. Conservation holds (4 -> 4x1)
        let mut grid = Grid::new(5, 5);
        grid.add_load(2, 2, 4);

        let initial_load = grid.total_load();
        grid.update(0.0);
        let final_load = grid.total_load();

        assert_eq!(initial_load, final_load);
    }

    #[test]
    fn test_boundary_loss() {
        // 3x3, put 4 at corner (0,0). Should lose 2 grains to boundary.
        let mut grid = Grid::new(3, 3);
        grid.add_load(0, 0, 4);

        grid.update(0.0);

        // (0,0) becomes 0.
        // Neighbors (1,0) and (0,1) gain 1.
        // Total load should be 2.
        assert_eq!(grid.total_load(), 2);
    }
}
