use locus::Topology;
use rand::Rng;

pub struct LifeGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<bool>,
    pub next_cells: Vec<bool>,
}

impl LifeGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        let size = width * height;
        let cells = (0..size).map(|_| rng.gen::<bool>()).collect();
        let next_cells = vec![false; size];

        Self {
            width,
            height,
            cells,
            next_cells,
        }
    }

    pub fn get_index(&self, x: isize, y: isize) -> usize {
        // Use shared Klein topology logic from locus crate
        if let Some((ny, nx)) =
            Topology::Klein.normalize(y as i64, x as i64, self.width, self.height)
        {
            ny * self.width + nx
        } else {
            // Should not happen for Klein topology as it wraps indefinitely
            0
        }
    }

    pub fn update(&mut self) {
        let w = self.width as isize;
        let h = self.height as isize;

        for y in 0..h {
            for x in 0..w {
                let idx = (y as usize) * self.width + (x as usize);
                let mut alive_neighbors = 0;

                for dy in -1..=1 {
                    for dx in -1..=1 {
                        if dx == 0 && dy == 0 {
                            continue;
                        }
                        let neighbor_idx = self.get_index(x + dx, y + dy);
                        if self.cells[neighbor_idx] {
                            alive_neighbors += 1;
                        }
                    }
                }

                let is_alive = self.cells[idx];
                self.next_cells[idx] = match (is_alive, alive_neighbors) {
                    (true, 2) | (true, 3) => true,
                    (false, 3) => true,
                    _ => false,
                };
            }
        }

        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_cylinder_wrap() {
        // 10x10 grid
        let grid = LifeGrid::new(10, 10);

        // Right edge wrap
        // (9, 5) + (1, 0) -> (10, 5) -> (0, 5)
        assert_eq!(grid.get_index(10, 5), 5 * 10 + 0);

        // Left edge wrap
        // (0, 5) + (-1, 0) -> (-1, 5) -> (9, 5)
        assert_eq!(grid.get_index(-1, 5), 5 * 10 + 9);
    }

    #[test]
    fn test_mobius_twist_top() {
        let grid = LifeGrid::new(10, 10);

        // Top edge wrap (y < 0)
        // (x=0, y=0) neighbor Up (0, -1)
        // y -> 9 (Bottom)
        // x -> 9 - 0 = 9 (Right)
        // Result (9, 9)
        assert_eq!(grid.get_index(0, -1), 9 * 10 + 9);

        // (x=2, y=0) neighbor Up (2, -1)
        // y -> 9
        // x -> 9 - 2 = 7
        // Result (7, 9)
        assert_eq!(grid.get_index(2, -1), 9 * 10 + 7);
    }

    #[test]
    fn test_mobius_twist_bottom() {
        let grid = LifeGrid::new(10, 10);

        // Bottom edge wrap (y >= 10)
        // (x=9, y=9) neighbor Down (9, 10)
        // y -> 0
        // x -> 9 - 9 = 0
        // Result (0, 0)
        assert_eq!(grid.get_index(9, 10), 0 * 10 + 0);
    }

    #[test]
    fn test_corner_twist() {
        let grid = LifeGrid::new(10, 10);

        // Top Left (-1, -1)
        // target_y = -1 -> y wraps to 9.
        // target_x = -1.
        // twist x: 9 - (-1) = 10.
        // wrap x: 10 % 10 = 0.
        // Result (0, 9) (Bottom Left)
        assert_eq!(grid.get_index(-1, -1), 9 * 10 + 0);
    }
}
