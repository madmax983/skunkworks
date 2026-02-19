use rand::Rng;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Cell {
    Empty,
    Memory,
    IO,
    Corrupt,
    Root(usize), // Process ID
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut cells = vec![Cell::Empty; width * height];

        for i in 0..cells.len() {
            let roll: f32 = rng.gen();
            if roll < 0.05 {
                cells[i] = Cell::Memory;
            } else if roll < 0.08 {
                cells[i] = Cell::IO;
            } else if roll < 0.10 {
                cells[i] = Cell::Corrupt;
            }
        }

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<Cell> {
        if x < self.width && y < self.height {
            Some(self.cells[y * self.width + x])
        } else {
            None
        }
    }

    pub fn set(&mut self, x: usize, y: usize, cell: Cell) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = cell;
        }
    }

    pub fn neighbors(&self, x: usize, y: usize) -> Vec<(usize, usize)> {
        let mut neighbors = Vec::new();
        let dirs = [(0, 1), (0, -1), (1, 0), (-1, 0)];

        for (dx, dy) in dirs.iter() {
            let nx = x as isize + dx;
            let ny = y as isize + dy;

            if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                neighbors.push((nx as usize, ny as usize));
            }
        }
        neighbors
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_creation() {
        let grid = Grid::new(10, 10);
        assert_eq!(grid.width, 10);
        assert_eq!(grid.height, 10);
        assert_eq!(grid.cells.len(), 100);

        // Ensure at least some cells are resources (statistically likely)
        let _resources = grid.cells.iter().filter(|&&c| c == Cell::Memory || c == Cell::IO).count();
        // Probability is 8%, so on 100 cells, expected ~8. Let's just check >= 0 to be safe,
        // or ensure we handle the random nature.
        // Actually, just checking bounds is enough for a basic test.
        assert!(grid.get(0, 0).is_some());
        assert!(grid.get(10, 10).is_none());
    }
}
