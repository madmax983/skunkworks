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
    /// Material properties for each cell.
    pub materials: Vec<Material>,
    /// Speed of sound squared (c^2) map. Controls wave propagation speed.
    pub c2_map: Vec<f32>,
    /// Damping map. Controls energy loss per cell.
    pub damping_map: Vec<f32>,
    /// Accumulated energy map (sum of absolute values).
    pub energy_map: Vec<f32>,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Material {
    Air,
    Wall,
    Slow, // High refractive index
    Fast, // Low refractive index
    Void, // Absorbs everything
}

impl PhysicsGrid {
    /// Creates a new physics grid with the specified dimensions.
    ///
    /// The grid is initialized with zero energy (silence) and Air everywhere.
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            u: vec![0.0; size],
            u_prev: vec![0.0; size],
            u_next: vec![0.0; size],
            materials: vec![Material::Air; size],
            c2_map: vec![0.4; size],
            damping_map: vec![0.999; size],
            energy_map: vec![0.0; size],
        }
    }

    /// Sets the material at a specific coordinate.
    pub fn set_material(&mut self, x: usize, y: usize, material: Material) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.materials[idx] = material;
            match material {
                Material::Air => {
                    self.c2_map[idx] = 0.4;
                    self.damping_map[idx] = 0.999;
                }
                Material::Wall => {
                    self.c2_map[idx] = 0.0;
                    self.damping_map[idx] = 0.0; // Irrelevant as value is forced to 0
                                                 // Clear energy at wall
                    self.u[idx] = 0.0;
                    self.u_prev[idx] = 0.0;
                    self.u_next[idx] = 0.0;
                }
                Material::Slow => {
                    self.c2_map[idx] = 0.1; // Slower wave speed
                    self.damping_map[idx] = 0.995; // Slightly more damping
                }
                Material::Fast => {
                    self.c2_map[idx] = 0.5; // Max 0.5 for 2D stability (Courant limit)
                    self.damping_map[idx] = 0.999;
                }
                Material::Void => {
                    self.c2_map[idx] = 0.5;
                    self.damping_map[idx] = 0.5; // Heavy damping
                }
            }
        }
    }

    /// Advances the simulation by one time step.
    pub fn step(&mut self) {
        let w = self.width;
        let h = self.height;

        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let idx = y * w + x;

                if self.materials[idx] == Material::Wall {
                    self.u_next[idx] = 0.0;
                    continue;
                }

                let up = (y - 1) * w + x;
                let down = (y + 1) * w + x;
                let left = y * w + (x - 1);
                let right = y * w + (x + 1);

                let u_curr = self.u[idx];
                let u_prev = self.u_prev[idx];
                let c2 = self.c2_map[idx];
                let damping = self.damping_map[idx];

                let laplacian =
                    self.u[up] + self.u[down] + self.u[left] + self.u[right] - 4.0 * u_curr;

                let mut val = 2.0 * u_curr - u_prev + c2 * laplacian;
                val *= damping;

                self.u_next[idx] = val;

                // Accumulate energy with decay
                self.energy_map[idx] = self.energy_map[idx] * 0.9995 + val.abs() * 0.005;
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
            if self.materials[idx] != Material::Wall {
                self.u[idx] += strength;
            }
        }
    }

    /// Adds a wall at the specified coordinates.
    pub fn add_wall(&mut self, x: usize, y: usize) {
        self.set_material(x, y, Material::Wall);
    }

    /// Removes a wall from the specified coordinates.
    pub fn remove_wall(&mut self, x: usize, y: usize) {
        self.set_material(x, y, Material::Air);
    }

    /// Resets all wave states to zero, silencing the simulation.
    pub fn clear_waves(&mut self) {
        self.u.fill(0.0);
        self.u_prev.fill(0.0);
        self.u_next.fill(0.0);
        self.energy_map.fill(0.0);
    }

    /// Removes all walls from the grid.
    pub fn clear_walls(&mut self) {
        for i in 0..self.materials.len() {
            if self.materials[i] == Material::Wall {
                self.materials[i] = Material::Air;
                self.c2_map[i] = 0.5;
                self.damping_map[i] = 0.999;
            }
        }
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
