use rand::Rng;
use rayon::prelude::*;

pub struct LifeGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<u8>,
    next_cells: Vec<u8>,
}

impl LifeGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let mut rng = rand::thread_rng();
        let cells: Vec<u8> = (0..size)
            .map(|_| if rng.gen::<f32>() < 0.2 { 1 } else { 0 })
            .collect();

        Self {
            width,
            height,
            next_cells: cells.clone(),
            cells,
        }
    }

    pub fn reset(&mut self) {
        let mut rng = rand::thread_rng();
        for i in 0..self.cells.len() {
            self.cells[i] = if rng.gen::<f32>() < 0.2 { 1 } else { 0 };
        }
    }

    pub fn update(&mut self, rules: (u8, u8, u8)) {
        let (s_min, s_max, birth) = rules;
        let width = self.width;
        let height = self.height;

        // We iterate over the *current* cells (read-only) and write to *next_cells*.
        // Using par_chunks_mut on next_cells allows us to parallelize by rows.
        let cells_slice = &self.cells;

        self.next_cells
            .par_chunks_mut(width)
            .enumerate()
            .for_each(|(y, row)| {
                for x in 0..width {
                    let mut neighbors = 0;

                    // Manual unrolling/checking for performance? No, let's keep it simple first.
                    // Wrap-around logic
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }

                            let nx = (x as isize + dx).rem_euclid(width as isize) as usize;
                            let ny = (y as isize + dy).rem_euclid(height as isize) as usize;

                            // Safety: nx < width, ny < height, so index is valid
                            if cells_slice[ny * width + nx] == 1 {
                                neighbors += 1;
                            }
                        }
                    }

                    let current = cells_slice[y * width + x];
                    let next = if current == 1 {
                        if neighbors >= s_min && neighbors <= s_max {
                            1
                        } else {
                            0
                        }
                    } else {
                        if neighbors == birth {
                            1
                        } else {
                            0
                        }
                    };
                    row[x] = next;
                }
            });

        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }

    pub fn get_density(&self) -> f32 {
        let alive = self.cells.iter().filter(|&&c| c == 1).count();
        alive as f32 / self.cells.len() as f32
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_update_glider() {
        // Test standard GoL rules (2, 3, 3) on a Glider
        let width = 10;
        let height = 10;
        let mut grid = LifeGrid::new(width, height);
        // Clear random init
        for c in grid.cells.iter_mut() {
            *c = 0;
        }

        // Glider
        // .O.
        // ..O
        // OOO
        let offset_x = 2;
        let offset_y = 2;
        grid.cells[offset_y * width + offset_x + 1] = 1;
        grid.cells[(offset_y + 1) * width + offset_x + 2] = 1;
        grid.cells[(offset_y + 2) * width + offset_x] = 1;
        grid.cells[(offset_y + 2) * width + offset_x + 1] = 1;
        grid.cells[(offset_y + 2) * width + offset_x + 2] = 1;

        let _initial_density = grid.get_density();
        grid.update((2, 3, 3));

        // Glider should still exist (density > 0)
        assert!(grid.get_density() > 0.0);
        // And should have moved/changed
        // (Just check it didn't explode or vanish immediately)
    }

    #[test]
    fn test_wrap_around() {
        let width = 3;
        let height = 3;
        let mut grid = LifeGrid::new(width, height);
        for c in grid.cells.iter_mut() {
            *c = 0;
        }

        // Blinker at edge
        // O O O (row 0)
        grid.cells[0] = 1;
        grid.cells[1] = 1;
        grid.cells[2] = 1;

        // Update with standard rules.
        // Center cell (1, 0) has neighbors (0,0) and (2,0). Count = 2.
        // Also (1, 2) (top wrap) and (1, 1) (bottom).
        // Let's just check simple survival logic.

        // If we set center only:
        for c in grid.cells.iter_mut() {
            *c = 0;
        }
        grid.cells[1 * width + 1] = 1; // Center

        // Neighbors:
        // (0,0), (1,0), (2,0)
        // (0,1),        (2,1)
        // (0,2), (1,2), (2,2)

        // If we set (0, 1) and (2, 1), they wrap around horizontally.
        grid.cells[1 * width + 0] = 1;
        grid.cells[1 * width + 2] = 1;

        // Update with rules requiring 2 neighbors to survive.
        // Center has 2 neighbors. Should survive.
        grid.update((2, 3, 3));
        assert_eq!(grid.cells[1 * width + 1], 1);
    }
}
