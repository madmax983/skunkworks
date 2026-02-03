pub const WIDTH: usize = 64;
pub const HEIGHT: usize = 64;

#[derive(Clone, Debug, PartialEq)]
pub struct Grid {
    pub cells: [[bool; WIDTH]; HEIGHT],
}

impl Grid {
    pub fn new() -> Self {
        Self {
            cells: [[false; WIDTH]; HEIGHT],
        }
    }

    pub fn toggle(&mut self, x: usize, y: usize) {
        if x < WIDTH && y < HEIGHT {
            self.cells[y][x] = !self.cells[y][x];
        }
    }

    pub fn step(&mut self) {
        let mut next = self.cells;
        for (y, row) in next.iter_mut().enumerate() {
            for (x, cell) in row.iter_mut().enumerate() {
                let neighbors = self.count_neighbors(x, y);
                let alive = self.cells[y][x];
                *cell = matches!((alive, neighbors), (true, 2) | (true, 3) | (false, 3));
            }
        }
        self.cells = next;
    }

    fn count_neighbors(&self, x: usize, y: usize) -> u8 {
        let mut count = 0;
        for dy in -1..=1 {
            for dx in -1..=1 {
                if dx == 0 && dy == 0 {
                    continue;
                }
                let nx = x as isize + dx;
                let ny = y as isize + dy;
                if nx >= 0
                    && nx < WIDTH as isize
                    && ny >= 0
                    && ny < HEIGHT as isize
                    && self.cells[ny as usize][nx as usize]
                {
                    count += 1;
                }
            }
        }
        count
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_block_stable() {
        let mut grid = Grid::new();
        // Block (2x2 square)
        grid.toggle(1, 1);
        grid.toggle(1, 2);
        grid.toggle(2, 1);
        grid.toggle(2, 2);

        let initial = grid.clone();
        grid.step();

        assert_eq!(grid, initial, "Block pattern should be stable");
    }

    #[test]
    fn test_blinker_oscillator() {
        let mut grid = Grid::new();
        // Blinker (horizontal line of 3)
        // (x, y) coordinates
        grid.toggle(1, 2);
        grid.toggle(2, 2);
        grid.toggle(3, 2);

        grid.step();

        // Should be vertical line at x=2
        // (2, 1), (2, 2), (2, 3)

        assert!(!grid.cells[2][1], "Left cell should die"); // y=2, x=1
        assert!(!grid.cells[2][3], "Right cell should die"); // y=2, x=3

        assert!(grid.cells[1][2], "Top neighbor should be born"); // y=1, x=2
        assert!(grid.cells[2][2], "Center cell should stay alive"); // y=2, x=2
        assert!(grid.cells[3][2], "Bottom neighbor should be born"); // y=3, x=2
    }
}
