#![allow(clippy::needless_range_loop)]
use rand::Rng;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Soil {
    Empty,    // Open air or cave
    HardRock, // Wall
    SoftSoil, // Dirt (diggable)
    Water,    // Goal
}

pub struct Grid {
    pub cells: Vec<Vec<Soil>>,
    pub width: usize,
    pub height: usize,
    pub start: (usize, usize),
    pub goal: (usize, usize),
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        // Start all SoftSoil
        let mut cells = vec![vec![Soil::SoftSoil; height]; width];

        // 1. Initial Random Noise (40% HardRock)
        for x in 1..width - 1 {
            for y in 1..height - 1 {
                if rng.gen_bool(0.40) {
                    cells[x][y] = Soil::HardRock;
                } else {
                    cells[x][y] = Soil::SoftSoil;
                }
            }
        }

        // 2. Cellular Automata Smoothing (4 steps)
        for _ in 0..4 {
            let mut next_cells = cells.clone();
            for x in 1..width - 1 {
                for y in 1..height - 1 {
                    let neighbors = count_neighbors(&cells, x, y, Soil::HardRock);

                    if neighbors > 4 {
                        next_cells[x][y] = Soil::HardRock;
                    } else if neighbors < 4 {
                        next_cells[x][y] = Soil::SoftSoil;
                    }
                }
            }
            cells = next_cells;
        }

        // 3. Ensure boundaries are HardRock
        for x in 0..width {
            cells[x][0] = Soil::HardRock;
            cells[x][height - 1] = Soil::HardRock;
        }
        for y in 0..height {
            cells[0][y] = Soil::HardRock;
            cells[width - 1][y] = Soil::HardRock;
        }

        // 4. Place Start (Top) and Goal (Bottom)
        // Find a valid SoftSoil spot near top
        let mut start = (width / 2, 1);
        for y in 1..height / 4 {
            for x in 1..width - 1 {
                if cells[x][y] == Soil::SoftSoil {
                    start = (x, y);
                    break;
                }
            }
        }
        // If still rock (unlikely), force it
        cells[start.0][start.1] = Soil::SoftSoil;

        // Find a valid spot near bottom for Water
        let mut goal = (width / 2, height - 2);
        for y in (height * 3 / 4..height - 1).rev() {
            for x in 1..width - 1 {
                if cells[x][y] == Soil::SoftSoil {
                    goal = (x, y);
                    cells[x][y] = Soil::Water;
                    // Make a small pool
                    if x > 0 {
                        cells[x - 1][y] = Soil::Water;
                    }
                    if x < width - 1 {
                        cells[x + 1][y] = Soil::Water;
                    }
                    if y > 0 {
                        cells[x][y - 1] = Soil::Water;
                    }
                    break;
                }
            }
        }

        Self {
            cells,
            width,
            height,
            start,
            goal,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Soil {
        if x >= self.width || y >= self.height {
            return Soil::HardRock;
        }
        self.cells[x][y]
    }
}

fn count_neighbors(cells: &[Vec<Soil>], cx: usize, cy: usize, target: Soil) -> usize {
    let mut count = 0;
    // Bounds check handled inside loop condition
    let min_x = cx.saturating_sub(1);
    let max_x = (cx + 1).min(cells.len() - 1);
    let min_y = cy.saturating_sub(1);
    let max_y = (cy + 1).min(cells[0].len() - 1);

    for x in min_x..=max_x {
        for y in min_y..=max_y {
            if x == cx && y == cy {
                continue;
            }
            if cells[x][y] == target {
                count += 1;
            }
        }
    }
    count
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_generation() {
        let grid = Grid::new(50, 50);
        assert_eq!(grid.width, 50);
        assert_eq!(grid.height, 50);
        assert_eq!(grid.get(0, 0), Soil::HardRock); // Boundary

        let (sx, sy) = grid.start;
        // Start might be overwritten by CA, but usually it's open.
        let start_soil = grid.get(sx, sy);
        assert!(
            start_soil == Soil::SoftSoil || start_soil == Soil::Water || start_soil == Soil::Empty,
            "Start should not be rock"
        );

        let (gx, gy) = grid.goal;
        assert_eq!(grid.get(gx, gy), Soil::Water);
    }
}
