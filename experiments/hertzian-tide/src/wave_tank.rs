/// A 2D Wave Tank simulation using the Finite Difference Method.
pub struct WaveTank {
    pub width: usize,
    pub height: usize,
    /// Current state of the wave field (height at each cell).
    pub current: Vec<f32>,
    /// Previous state of the wave field (used for time integration).
    pub previous: Vec<f32>,
    /// Energy loss per step (0.0 - 1.0).
    pub damping: f32,
}

impl WaveTank {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            current: vec![0.0; size],
            previous: vec![0.0; size],
            damping: 0.99, // Energy loss per step
        }
    }

    /// Advances the simulation by one time step.
    /// Uses Verlet integration for the Wave Equation.
    pub fn step(&mut self) {
        let w = self.width;
        let h = self.height;

        // Parallel update logic:
        // u_next[i] = 2*u[i] - u_prev[i] + damping * (laplacian)
        // We write u_next into self.previous (overwriting old data)
        // Then we swap current and previous.

        for y in 1..h-1 {
            for x in 1..w-1 {
                let i = y * w + x;
                let val = self.current[i];
                let old_val = self.previous[i];

                // Laplacian neighbors
                let up = self.current[(y - 1) * w + x];
                let down = self.current[(y + 1) * w + x];
                let left = self.current[y * w + (x - 1)];
                let right = self.current[y * w + (x + 1)];

                let laplacian = up + down + left + right - 4.0 * val;

                // Wave speed coefficient (c^2 * dt^2)
                // 0.5 is stable for this discretization.
                let alpha = 0.5;

                let new_val = (2.0 * val - old_val + alpha * laplacian) * self.damping;

                // Write into 'previous' (which becomes 'next')
                self.previous[i] = new_val;
            }
        }

        // Swap references
        std::mem::swap(&mut self.current, &mut self.previous);
    }

    /// Adds a force/height at the specified coordinates.
    pub fn poke(&mut self, x: usize, y: usize, strength: f32) {
        if x > 0 && x < self.width - 1 && y > 0 && y < self.height - 1 {
            let idx = y * self.width + x;
            self.current[idx] += strength;
        }
    }

    /// Returns the height at the specified coordinates.
    pub fn get_height(&self, x: usize, y: usize) -> f32 {
        if x < self.width && y < self.height {
            self.current[y * self.width + x]
        } else {
            0.0
        }
    }
}
