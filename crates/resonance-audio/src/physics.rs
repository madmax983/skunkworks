/// Shared Physics Grid Logic for Resonance Experiments
///
/// This module implements the Finite Difference Time Domain (FDTD) solver for the 2D wave equation.
/// The solver uses a discrete grid to simulate wave propagation, reflection, and interference.
///
/// # Examples
///
/// ```
/// use resonance_audio::physics::PhysicsGrid;
///
/// // Create a grid
/// let mut grid = PhysicsGrid::new(50, 50);
///
/// // Pluck the center
/// grid.pluck(25, 25, 1.0);
///
/// // Run simulation step
/// grid.step();
///
/// // Check propagation
/// assert!(grid.get(25, 25) < 1.0); // Energy spreads out
/// ```
pub struct PhysicsGrid {
    /// The width of the simulation grid in cells.
    pub width: usize,
    /// The height of the simulation grid in cells.
    pub height: usize,
    /// The current state of the wave field (pressure/displacement at each cell).
    pub u: Vec<f32>,
    /// The previous state of the wave field (t - 1), used for time integration.
    pub u_prev: Vec<f32>,
    /// Scratch buffer for calculating the next state (t + 1).
    pub u_next: Vec<f32>,
    /// Boolean mask where `true` indicates a wall (reflective boundary) and `false` is open space.
    pub walls: Vec<bool>,
    /// Damping factor applied at each step to simulate energy loss (0.0 = instant stop, 1.0 = no loss).
    pub damping: f32,
}

impl PhysicsGrid {
    /// Creates a new physics grid with the specified dimensions.
    ///
    /// The grid is initialized with zero energy (silence) and no walls.
    /// The default damping is set to 0.999 for high resonance.
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            u: vec![0.0; width * height],
            u_prev: vec![0.0; width * height],
            u_next: vec![0.0; width * height],
            walls: vec![false; width * height],
            damping: 0.999, // High resonance
        }
    }

    /// Advances the simulation by one time step.
    ///
    /// This method implements the discrete 2D wave equation:
    /// `u_next[x,y] = 2*u[x,y] - u_prev[x,y] + c^2 * Laplacian(u)`
    ///
    /// It handles:
    /// - Wave propagation using a 5-point stencil Laplacian.
    /// - Wall reflections (walls force value to 0, causing reflection).
    /// - Damping.
    /// - Buffer swapping (prev -> curr, curr -> next).
    pub fn step(&mut self) {
        let w = self.width;
        let h = self.height;
        let c2 = 0.5; // Courant number squared

        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let idx = y * w + x;

                if self.walls[idx] {
                    self.u_next[idx] = 0.0;
                    continue;
                }

                let up = (y - 1) * w + x;
                let down = (y + 1) * w + x;
                let left = y * w + (x - 1);
                let right = y * w + (x + 1);

                let u_curr = self.u[idx];
                let u_prev = self.u_prev[idx];

                // Laplacian with wall check?
                // If neighbor is wall, its value is 0.
                // Since we enforce 0 for walls, we don't need explicit check here,
                // provided we zeroed them out.
                let laplacian =
                    self.u[up] + self.u[down] + self.u[left] + self.u[right] - 4.0 * u_curr;

                let mut val = 2.0 * u_curr - u_prev + c2 * laplacian;
                val *= self.damping;

                self.u_next[idx] = val;
            }
        }

        // Swap buffers
        std::mem::swap(&mut self.u_prev, &mut self.u);
        std::mem::swap(&mut self.u, &mut self.u_next);
    }

    /// Injects energy into the grid at a specific point (like a pluck).
    ///
    /// The energy is added to the current state, creating a disturbance that will propagate.
    /// Does nothing if the coordinates are out of bounds or inside a wall.
    pub fn pluck(&mut self, x: usize, y: usize, strength: f32) {
        if x > 0 && x < self.width - 1 && y > 0 && y < self.height - 1 {
            let idx = y * self.width + x;
            if !self.walls[idx] {
                self.u[idx] += strength;
            }
        }
    }

    /// Adds a wall at the specified coordinates.
    ///
    /// Also clears any existing energy at that point to prevent instabilities.
    pub fn add_wall(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.walls[idx] = true;
            self.u[idx] = 0.0;
            self.u_prev[idx] = 0.0;
            self.u_next[idx] = 0.0;
        }
    }

    /// Removes a wall from the specified coordinates.
    pub fn remove_wall(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.walls[idx] = false;
        }
    }

    /// Resets all wave states to zero, silencing the simulation.
    pub fn clear_waves(&mut self) {
        self.u.fill(0.0);
        self.u_prev.fill(0.0);
        self.u_next.fill(0.0);
    }

    /// Removes all walls from the grid.
    pub fn clear_walls(&mut self) {
        self.walls.fill(false);
    }

    /// Gets the current wave value (pressure) at the specified coordinates.
    ///
    /// Returns 0.0 if coordinates are out of bounds.
    pub fn get(&self, x: usize, y: usize) -> f32 {
        if x < self.width && y < self.height {
            self.u[y * self.width + x]
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_propagation() {
        let mut grid = PhysicsGrid::new(10, 10);
        grid.pluck(5, 5, 1.0);

        // Initial state
        assert_eq!(grid.get(5, 5), 1.0);
        assert_eq!(grid.get(5, 6), 0.0);

        grid.step();

        // After one step, the wave should propagate to neighbors
        assert!(
            grid.get(5, 6).abs() > 0.001,
            "Wave did not propagate to (5,6)"
        );
        assert!(
            grid.get(4, 5).abs() > 0.001,
            "Wave did not propagate to (4,5)"
        );
    }

    #[test]
    fn test_wall() {
        let mut grid = PhysicsGrid::new(10, 10);
        grid.add_wall(5, 6);
        grid.pluck(5, 5, 1.0);

        grid.step();

        // Value at wall should be 0
        assert_eq!(grid.get(5, 6), 0.0);
        // But value at (5,4) should be non-zero
        assert!(grid.get(5, 4).abs() > 0.001);
    }
}
