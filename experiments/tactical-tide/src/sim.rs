use rand::Rng;

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub terrain: f32,
    pub water: f32,
    pub velocity: (f32, f32), // Not strictly used in simple pipe model, but kept for future
}

impl Cell {
    pub fn new(terrain: f32) -> Self {
        Self {
            terrain,
            water: 0.0,
            velocity: (0.0, 0.0),
        }
    }

    pub fn total_height(&self) -> f32 {
        self.terrain + self.water
    }
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        let mut rng = rand::thread_rng();
        let mut cells = Vec::with_capacity(width * height);

        for _ in 0..height {
            for _ in 0..width {
                // Simple terrain generation: mostly flat with some hills
                let noise = rng.gen::<f32>();
                let terrain = if noise > 0.8 {
                    (noise - 0.8) * 5.0 // Some hills
                } else {
                    0.0 // Flat
                };
                cells.push(Cell::new(terrain));
            }
        }

        Self {
            width,
            height,
            cells,
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Option<&Cell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&self.cells[y * self.width + x])
        }
    }

    pub fn get_mut(&mut self, x: usize, y: usize) -> Option<&mut Cell> {
        if x >= self.width || y >= self.height {
            None
        } else {
            Some(&mut self.cells[y * self.width + x])
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut deltas = vec![0.0; self.cells.len()];
        let flow_rate = 10.0; // Tuning parameter

        // Calculate fluxes
        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let my_h = self.cells[idx].total_height();

                // Flow to Right
                if x + 1 < self.width {
                    let right_idx = y * self.width + (x + 1);
                    let right_h = self.cells[right_idx].total_height();
                    let diff = my_h - right_h;
                    let flow = diff * flow_rate * dt;

                    // Clamp flow to available water?
                    // Simple Euler integration is unstable without clamping or small dt.
                    // We'll trust dt is small for now, or use a damping factor.

                    deltas[idx] -= flow;
                    deltas[right_idx] += flow;
                }

                // Flow Down
                if y + 1 < self.height {
                    let down_idx = (y + 1) * self.width + x;
                    let down_h = self.cells[down_idx].total_height();
                    let diff = my_h - down_h;
                    let flow = diff * flow_rate * dt;

                    deltas[idx] -= flow;
                    deltas[down_idx] += flow;
                }
            }
        }

        // Apply deltas
        for (i, delta) in deltas.iter().enumerate() {
            self.cells[i].water += delta;
            if self.cells[i].water < 0.0 {
                self.cells[i].water = 0.0; // Conservation of mass violation if we just clamp?
                                          // Yes, but prevents negative water.
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_water_flow() {
        let mut grid = Grid::new(3, 3);
        // Flatten terrain
        for cell in grid.cells.iter_mut() {
            cell.terrain = 0.0;
        }

        // Add water to center
        grid.get_mut(1, 1).unwrap().water = 10.0;

        // Update
        grid.update(0.1);

        // Water should flow out of center
        let center_water = grid.get(1, 1).unwrap().water;
        let right_water = grid.get(2, 1).unwrap().water;
        let down_water = grid.get(1, 2).unwrap().water;

        assert!(center_water < 10.0);
        assert!(right_water > 0.0);
        assert!(down_water > 0.0);
    }
}
