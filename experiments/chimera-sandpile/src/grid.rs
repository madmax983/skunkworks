use rayon::prelude::*;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Cell {
    pub load: u32,
}

pub struct SandGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
    pub next_cells: Vec<Cell>,
}

impl SandGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            cells: vec![Cell { load: 0 }; size],
            next_cells: vec![Cell { load: 0 }; size],
        }
    }

    pub fn get_index(&self, x: usize, y: usize) -> usize {
        y * self.width + x
    }

    pub fn add_load(&mut self, x: usize, y: usize, amount: u32) {
        if x < self.width && y < self.height {
            let idx = self.get_index(x, y);
            self.cells[idx].load = self.cells[idx].load.saturating_add(amount);
        }
    }

    pub fn reduce_load(&mut self, x: usize, y: usize, amount: u32) {
        if x < self.width && y < self.height {
            let idx = self.get_index(x, y);
            self.cells[idx].load = self.cells[idx].load.saturating_sub(amount);
        }
    }

    /// Updates the sandpile and returns a list of indices that toppled.
    pub fn update(&mut self) -> Vec<usize> {
        let width = self.width;
        let height = self.height;

        let current_cells = &self.cells;

        // Parallel update calculation
        let toppled_indices: Vec<usize> = self.next_cells
            .par_iter_mut()
            .enumerate()
            .map(|(i, next_cell)| {
                let x = i % width;
                let y = i / width;

                let mut my_load = current_cells[i].load;
                let mut did_topple = false;

                // 1. Toppling
                if my_load >= 4 {
                    my_load -= 4;
                    did_topple = true;
                }

                // 2. Inflow
                let neighbors = [
                    (x.wrapping_sub(1), y),
                    (x + 1, y),
                    (x, y.wrapping_sub(1)),
                    (x, y + 1),
                ];

                for (nx, ny) in neighbors {
                    if nx < width && ny < height {
                        let n_idx = ny * width + nx;
                        if current_cells[n_idx].load >= 4 {
                            my_load += 1;
                        }
                    }
                }

                next_cell.load = my_load;

                if did_topple {
                    Some(i)
                } else {
                    None
                }
            })
            .filter_map(|x| x)
            .collect();

        std::mem::swap(&mut self.cells, &mut self.next_cells);

        toppled_indices
    }

    pub fn get_load(&self, x: usize, y: usize) -> u32 {
        if x < self.width && y < self.height {
            self.cells[self.get_index(x, y)].load
        } else {
            0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_sand_topple() {
        let mut grid = SandGrid::new(3, 3);
        grid.add_load(1, 1, 4); // Center
        let toppled = grid.update();
        assert_eq!(toppled.len(), 1);
        assert_eq!(toppled[0], grid.get_index(1, 1));
        assert_eq!(grid.get_load(1, 1), 0);
        assert_eq!(grid.get_load(0, 1), 1);
        assert_eq!(grid.get_load(2, 1), 1);
        assert_eq!(grid.get_load(1, 0), 1);
        assert_eq!(grid.get_load(1, 2), 1);
    }
}
