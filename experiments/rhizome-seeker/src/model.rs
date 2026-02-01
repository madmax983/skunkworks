use rand::prelude::*;

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum SoilType {
    Rock,
    Clay,
    Sand,
    Water,
}

#[derive(Debug, Clone, Copy)]
pub struct SoilCell {
    pub density: f32,  // 0.0 (Air/Water) to 1.0 (Bedrock)
    pub moisture: f32, // 0.0 (Dry) to 1.0 (Saturated)
    pub kind: SoilType,
    pub occupied_by: Option<usize>, // Root ID
}

impl Default for SoilCell {
    fn default() -> Self {
        Self {
            density: 0.5,
            moisture: 0.1,
            kind: SoilType::Clay,
            occupied_by: None,
        }
    }
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<SoilCell>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let cells = vec![SoilCell::default(); width * height];
        Self {
            width,
            height,
            cells,
        }
    }

    pub fn generate(&mut self) {
        let mut rng = rand::rng();

        // simple cellular automata / noise simulation
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let val: f32 = rng.random();

                // Base generation
                if val > 0.98 {
                    self.cells[idx] = SoilCell {
                        density: 0.1,
                        moisture: 1.0,
                        kind: SoilType::Water,
                        occupied_by: None,
                    };
                } else if val > 0.85 {
                    self.cells[idx] = SoilCell {
                        density: 0.9,
                        moisture: 0.0,
                        kind: SoilType::Rock,
                        occupied_by: None,
                    };
                } else if val > 0.5 {
                    self.cells[idx] = SoilCell {
                        density: 0.6,
                        moisture: 0.2,
                        kind: SoilType::Clay,
                        occupied_by: None,
                    };
                } else {
                    self.cells[idx] = SoilCell {
                        density: 0.3,
                        moisture: 0.05,
                        kind: SoilType::Sand,
                        occupied_by: None,
                    };
                }
            }
        }

        // Smoothing / Cellular Automata pass to make clumps
        self.smooth(2);
    }

    fn smooth(&mut self, iterations: usize) {
        let mut rng = rand::rng();
        for _ in 0..iterations {
            let mut new_cells = self.cells.clone();
            for y in 1..self.height - 1 {
                for x in 1..self.width - 1 {
                    let idx = y * self.width + x;

                    // Count neighbors
                    let mut rock_neighbors = 0;
                    let mut water_neighbors = 0;

                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            if dx == 0 && dy == 0 {
                                continue;
                            }
                            let n_idx = ((y as isize + dy) * self.width as isize
                                + (x as isize + dx))
                                as usize;
                            match self.cells[n_idx].kind {
                                SoilType::Rock => rock_neighbors += 1,
                                SoilType::Water => water_neighbors += 1,
                                _ => {}
                            }
                        }
                    }

                    if rock_neighbors >= 4 {
                        new_cells[idx] = SoilCell {
                            density: 0.95,
                            moisture: 0.0,
                            kind: SoilType::Rock,
                            occupied_by: None,
                        };
                    } else if water_neighbors >= 4 {
                        new_cells[idx] = SoilCell {
                            density: 0.1,
                            moisture: 1.0,
                            kind: SoilType::Water,
                            occupied_by: None,
                        };
                    } else if rng.random_bool(0.05) && water_neighbors > 0 {
                        // Seepage
                        new_cells[idx].moisture += 0.2;
                        new_cells[idx].moisture = new_cells[idx].moisture.min(1.0);
                    }
                }
            }
            self.cells = new_cells;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&SoilCell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&self.cells[y * self.width + x])
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_grid_generation() {
        let mut grid = Grid::new(50, 50);
        grid.generate();

        assert_eq!(grid.width, 50);
        assert_eq!(grid.height, 50);
        assert_eq!(grid.cells.len(), 2500);

        // Ensure there is variety
        let first_kind = grid.cells[0].kind;
        let all_same = grid.cells.iter().all(|c| c.kind == first_kind);
        assert!(!all_same, "Grid should generate diverse terrain");
    }
}
