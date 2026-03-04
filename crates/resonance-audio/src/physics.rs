//! # Resonance Audio Physics
//!
//! Provides the underlying 2D Finite Difference Time Domain (FDTD) wave physics simulation.
//!
//! The [`PhysicsGrid`] is responsible for advancing the wave equation on a discrete
//! 2D grid, taking into account different materials and their effect on wave
//! propagation and damping. It uses a triple-buffer technique to prevent
//! allocation during the simulation steps.

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
    pub(crate) width: usize,
    /// The height of the simulation grid in cells.
    pub(crate) height: usize,
    /// The current state of the wave field (pressure/displacement at each cell).
    pub(crate) u: Vec<f32>,
    /// The previous state of the wave field (t - 1), used for time integration.
    pub(crate) u_prev: Vec<f32>,
    /// Scratch buffer for calculating the next state (t + 1).
    pub(crate) u_next: Vec<f32>,
    /// Material properties for each cell.
    pub(crate) materials: Vec<Material>,
    /// Speed of sound squared (c^2) map. Controls wave propagation speed.
    pub(crate) c2_map: Vec<f32>,
    /// Damping map. Controls energy loss per cell.
    pub(crate) damping_map: Vec<f32>,
    /// Accumulated energy map (sum of absolute values).
    pub(crate) energy_map: Vec<f32>,
}

/// Represents the physical properties of a cell in the acoustic simulation grid.
///
/// Different materials affect how sound waves propagate through them, acting as
/// walls, lenses, or dampeners.
#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Material {
    /// Standard propagation medium ($c^2 = 0.4$, low damping).
    Air,
    /// Absolute boundary. Sound does not enter and reflects perfectly.
    Wall,
    /// High refractive index ($c^2 = 0.1$). Waves travel slower here, bending towards it (like an acoustic lens).
    Slow,
    /// Low refractive index ($c^2 = 0.5$). Waves travel faster here.
    Fast,
    /// High absorption. Waves enter but lose energy extremely quickly, minimizing reflection.
    Void,
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

    /// Returns the width of the grid.
    pub fn width(&self) -> usize {
        self.width
    }

    /// Returns the height of the grid.
    pub fn height(&self) -> usize {
        self.height
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
    ///
    /// Implements the standard 2D FDTD update for the wave equation.
    ///
    /// The wave equation is discretized as:
    /// $u_{x,y}^{t+1} = 2u_{x,y}^t - u_{x,y}^{t-1} + c^2 \cdot \nabla^2 u_{x,y}^t$
    ///
    /// Where $\nabla^2$ is the discrete Laplacian.
    ///
    /// # Buffer Swapping Logic
    ///
    /// To avoid allocation, we cycle through three buffers: `u_prev`, `u`, and `u_next`.
    ///
    /// 1. Compute `u_next` (t+1) using `u` (t) and `u_prev` (t-1).
    /// 2. `u_prev` becomes `u` (storing state t for the next step).
    /// 3. `u` becomes `u_next` (storing state t+1 for the next step).
    /// 4. `u_next` reclaims the old `u_prev` memory to be used as scratch space in the next iteration.
    ///
    /// # Examples
    ///
    /// ```
    /// use resonance_audio::physics::PhysicsGrid;
    /// let mut grid = PhysicsGrid::new(10, 10);
    /// grid.pluck(5, 5, 1.0);
    /// grid.step();
    /// // The wave has begun propagating outwards.
    /// assert!(grid.get(5, 5) < 1.0);
    /// ```
    pub fn step(&mut self) {
        let w = self.width;
        let h = self.height;

        // Grid must be at least 3x3 to have an interior for the 5-point stencil
        if w < 3 || h < 3 {
            return;
        }

        // Iterate over the interior of the grid (skipping boundaries)
        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let idx = y * w + x;

                // Wall handling is implicit via c2_map[idx] == 0.0 and damping_map[idx] == 0.0,
                // which results in val = 0.0. This allows us to skip the branch and memory lookup.

                let up = (y - 1) * w + x;
                let down = (y + 1) * w + x;
                let left = y * w + (x - 1);
                let right = y * w + (x + 1);

                let u_curr = self.u[idx];
                let u_prev = self.u_prev[idx];
                let c2 = self.c2_map[idx];
                let damping = self.damping_map[idx];

                // Standard 5-point discrete Laplacian stencil
                let laplacian =
                    self.u[up] + self.u[down] + self.u[left] + self.u[right] - 4.0 * u_curr;

                // Wave equation update
                let mut val = 2.0 * u_curr - u_prev + c2 * laplacian;
                val *= damping;

                self.u_next[idx] = val;

                // Accumulate energy with decay (for visualization)
                self.energy_map[idx] = self.energy_map[idx] * 0.9995 + val.abs() * 0.005;
            }
        }

        // Cycle buffers:
        // t-1 (u_prev) -> recycled
        // t   (u)      -> t-1 (u_prev)
        // t+1 (u_next) -> t   (u)
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

    #[test]
    fn test_energy_decay() {
        let mut grid = PhysicsGrid::new(10, 10);
        // Pluck with high energy
        grid.pluck(5, 5, 100.0);

        // Run simulation for a while
        for _ in 0..100 {
            grid.step();
        }

        // Energy should have spread out and be non-zero
        let mut total_energy = 0.0;
        for val in &grid.u {
            total_energy += val.abs();
        }

        assert!(total_energy > 0.0);
        assert!(total_energy.is_finite());
    }

    #[test]
    fn test_damping_effect() {
        let mut grid_damped = PhysicsGrid::new(10, 10);
        // Set strong damping
        grid_damped.damping_map.fill(0.9);
        grid_damped.pluck(5, 5, 1.0);

        let mut grid_undamped = PhysicsGrid::new(10, 10);
        // Set no damping
        grid_undamped.damping_map.fill(1.0);
        grid_undamped.pluck(5, 5, 1.0);

        // Run both
        for _ in 0..50 {
            grid_damped.step();
            grid_undamped.step();
        }

        let energy_damped: f32 = grid_damped.u.iter().map(|v| v.abs()).sum();
        let energy_undamped: f32 = grid_undamped.u.iter().map(|v| v.abs()).sum();

        // Damped grid should have less total "activity"
        assert!(energy_damped < energy_undamped);
    }
}
