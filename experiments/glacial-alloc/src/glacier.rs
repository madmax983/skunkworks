pub struct GlacierGrid {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<f32>, // Represents ice depth
}

impl GlacierGrid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![0.0; width * height],
        }
    }

    /// Adds "snow" (ice) to a specific region.
    pub fn allocate(&mut self, x: usize, y: usize, amount: f32) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.cells[idx] += amount;
        }
    }

    /// Removes ice from a specific region (melting/freeing).
    pub fn free(&mut self, x: usize, y: usize, amount: f32) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.cells[idx] = (self.cells[idx] - amount).max(0.0);
        }
    }

    /// Simulates glacial flow.
    /// Ice moves from high cells to lower neighbors.
    pub fn update_flow(&mut self) {
        let mut changes = vec![0.0; self.cells.len()];
        let viscosity = 0.1; // How fast ice flows

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let current_height = self.cells[idx];

                if current_height <= 0.0 { continue; }

                // Neighbors: Up, Down, Left, Right
                let neighbors = [
                    (x as isize, y as isize - 1),
                    (x as isize, y as isize + 1),
                    (x as isize - 1, y as isize),
                    (x as isize + 1, y as isize),
                ];

                for &(nx, ny) in &neighbors {
                    if nx >= 0 && nx < self.width as isize && ny >= 0 && ny < self.height as isize {
                        let n_idx = (ny as usize) * self.width + (nx as usize);
                        let neighbor_height = self.cells[n_idx];

                        if current_height > neighbor_height {
                            let diff = current_height - neighbor_height;
                            let flow = diff * viscosity * 0.25; // Distribute flow

                            changes[idx] -= flow;
                            changes[n_idx] += flow;
                        }
                    } else {
                        // Calving into the void (edge of the map)
                        let flow = current_height * viscosity * 0.1;
                        changes[idx] -= flow;
                    }
                }
            }
        }

        for (i, change) in changes.iter().enumerate() {
            self.cells[i] = (self.cells[i] + change).max(0.0);
        }
    }

    pub fn get_total_mass(&self) -> f32 {
        self.cells.iter().sum()
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_allocation() {
        let mut grid = GlacierGrid::new(10, 10);
        grid.allocate(5, 5, 10.0);
        assert_eq!(grid.cells[5 * 10 + 5], 10.0);
        assert_eq!(grid.get_total_mass(), 10.0);
    }

    #[test]
    fn test_flow() {
        let mut grid = GlacierGrid::new(3, 3);
        // Place a stack in the middle
        grid.allocate(1, 1, 10.0);

        // Update flow
        grid.update_flow();

        let center = grid.cells[1 * 3 + 1];
        let neighbor = grid.cells[0 * 3 + 1]; // Left neighbor

        // Center should decrease, neighbor should increase
        assert!(center < 10.0);
        assert!(neighbor > 0.0);

        // Mass conservation check (some might be lost to edges, but center is safe from edges in 3x3 if logic works right?
        // Wait, edges are 0,0 0,1 etc. 1,1 has neighbors 1,0 1,2 0,1 2,1.
        // Neighbors are all valid in 3x3.
        // No mass should be lost to void in the first tick if edges are empty.
        // Actually, logic says: "if nx >= 0 ... else ... calving".
        // Neighbors of (1,1) are (1,0), (1,2), (0,1), (2,1). All are valid.
        // So mass should be conserved.
        assert!((grid.get_total_mass() - 10.0).abs() < 0.0001);
    }

    #[test]
    fn test_calving() {
         let mut grid = GlacierGrid::new(1, 1);
         grid.allocate(0, 0, 10.0);
         grid.update_flow();
         // It has no neighbors, so it should calve into the void.
         assert!(grid.get_total_mass() < 10.0);
    }
}
