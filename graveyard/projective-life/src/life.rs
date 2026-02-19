#[derive(Clone)]
pub struct LifeGame {
    pub grid: Vec<u8>,
    pub next_grid: Vec<u8>,
    pub width: usize,
    pub height: usize,
}

impl LifeGame {
    pub fn new(width: usize, height: usize) -> Self {
        let grid = vec![0; width * height];
        let next_grid = vec![0; width * height];
        Self {
            grid,
            next_grid,
            width,
            height,
        }
    }

    pub fn randomize(&mut self, seed: u64) {
        let mut s = seed;
        for i in 0..self.grid.len() {
            s = (s.wrapping_mul(6364136223846793005)).wrapping_add(1);
            self.grid[i] = if (s >> 32) % 2 == 0 { 0 } else { 1 };
        }
    }

    pub fn set(&mut self, x: usize, y: usize, val: u8) {
        if x < self.width && y < self.height {
            self.grid[y * self.width + x] = val;
        }
    }

    pub fn calculate_neighbor(width: usize, height: usize, x: i32, y: i32) -> (usize, usize) {
        let w = width as i32;
        let h = height as i32;

        let mut nx = x;
        let mut ny = y;

        // Wrap Top/Bottom (flip X)
        if ny < 0 {
            ny = h - 1;
            nx = w - 1 - nx;
        } else if ny >= h {
            ny = 0;
            nx = w - 1 - nx;
        }

        // Wrap Left/Right (flip Y)
        if nx < 0 {
            nx = w - 1;
            ny = h - 1 - ny;
        } else if nx >= w {
            nx = 0;
            ny = h - 1 - ny;
        }

        (nx as usize, ny as usize)
    }

    pub fn get_neighbor(&self, x: i32, y: i32) -> (usize, usize) {
        Self::calculate_neighbor(self.width, self.height, x, y)
    }

    pub fn step(&mut self) {
        let width = self.width;
        let height = self.height;
        let grid = &self.grid;
        let next_grid = &mut self.next_grid;

        for y in 0..height {
            for x in 0..width {
                let mut neighbors = 0;
                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let (nx, ny) =
                            Self::calculate_neighbor(width, height, x as i32 + dx, y as i32 + dy);
                        neighbors += grid[ny * width + nx];
                    }
                }

                let idx = y * width + x;
                let alive = grid[idx] == 1;

                // Conway's Rules
                if alive && (neighbors < 2 || neighbors > 3) {
                    next_grid[idx] = 0;
                } else if !alive && neighbors == 3 {
                    next_grid[idx] = 1;
                } else {
                    next_grid[idx] = grid[idx];
                }
            }
        }
        std::mem::swap(&mut self.grid, &mut self.next_grid);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_neighbor_wrapping() {
        let game = LifeGame::new(10, 10);

        // Top edge: (5, -1) -> (10-1-5, 9) = (4, 9)
        assert_eq!(game.get_neighbor(5, -1), (4, 9));

        // Bottom edge: (5, 10) -> (10-1-5, 0) = (4, 0)
        assert_eq!(game.get_neighbor(5, 10), (4, 0));

        // Left edge: (-1, 5) -> (9, 10-1-5) = (9, 4)
        assert_eq!(game.get_neighbor(-1, 5), (9, 4));

        // Right edge: (10, 5) -> (0, 10-1-5) = (0, 4)
        assert_eq!(game.get_neighbor(10, 5), (0, 4));

        // Corner: (-1, -1)
        // (-1, -1) -> Y check: ny=9, nx=10.
        // (10, 9) -> X check: nx=0, ny=0.
        // Result: (0, 0).
        assert_eq!(game.get_neighbor(-1, -1), (0, 0));
    }
}
