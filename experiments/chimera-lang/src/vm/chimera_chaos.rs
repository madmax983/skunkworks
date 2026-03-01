use serde::{Deserialize, Serialize};

pub const GRID_SIZE: usize = 16;

#[derive(Clone, Debug, Serialize, Deserialize)]
pub struct ChimeraChaos {
    pub grid: Vec<Vec<f64>>,
    pub r_grid: Vec<Vec<f64>>, // Growth rate parameters (3.0 - 4.0)
    pub coupling: f64,
}

impl Default for ChimeraChaos {
    fn default() -> Self {
        Self::new()
    }
}

impl ChimeraChaos {
    pub fn new() -> Self {
        Self {
            grid: vec![vec![0.5; GRID_SIZE]; GRID_SIZE],
            r_grid: vec![vec![3.9; GRID_SIZE]; GRID_SIZE], // Default to chaotic
            coupling: 0.1,
        }
    }

    pub fn tick(&mut self) {
        let mut next_grid = self.grid.clone();
        for y in 0..GRID_SIZE {
            for x in 0..GRID_SIZE {
                let r = self.r_grid[y][x];
                let val = self.grid[y][x];
                // Logistic Map
                let f = |v: f64| r * v * (1.0 - v);

                let self_term = f(val);

                // Coupling (Von Neumann neighborhood)
                let neighbors = [
                    ((y + GRID_SIZE - 1) % GRID_SIZE, x),
                    ((y + 1) % GRID_SIZE, x),
                    (y, (x + GRID_SIZE - 1) % GRID_SIZE),
                    (y, (x + 1) % GRID_SIZE),
                ];

                let mut neighbor_sum = 0.0;
                for (ny, nx) in neighbors {
                    neighbor_sum += f(self.grid[ny][nx]);
                }
                let avg_neighbor = neighbor_sum / 4.0;

                // CML Update
                next_grid[y][x] = (1.0 - self.coupling) * self_term + self.coupling * avg_neighbor;
            }
        }
        self.grid = next_grid;
    }
}
