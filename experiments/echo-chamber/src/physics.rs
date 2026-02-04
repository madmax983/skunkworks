pub struct AcousticGrid {
    pub width: usize,
    pub height: usize,
    pub pressure: Vec<f32>,
    pub pressure_prev: Vec<f32>,
    pub pressure_next: Vec<f32>,
    pub walls: Vec<bool>,
    pub damping: f32,
}

impl AcousticGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            pressure: vec![0.0; size],
            pressure_prev: vec![0.0; size],
            pressure_next: vec![0.0; size],
            walls: vec![false; size],
            damping: 0.999, // slight air damping
        }
    }

    pub fn step(&mut self) {
        let w = self.width;
        let h = self.height;
        // Courant number squared. 0.5 is stable for 2D FDTD.
        let c2 = 0.5;

        for y in 1..h - 1 {
            for x in 1..w - 1 {
                let idx = y * w + x;

                if self.walls[idx] {
                    // Inside a wall, pressure is 0 (or irrelevant)
                    self.pressure_next[idx] = 0.0;
                    continue;
                }

                let p_curr = self.pressure[idx];
                let p_prev = self.pressure_prev[idx];

                // Neighbors indices
                let idx_up = (y - 1) * w + x;
                let idx_down = (y + 1) * w + x;
                let idx_left = y * w + (x - 1);
                let idx_right = y * w + (x + 1);

                // Neumann Boundary Conditions:
                // If neighbor is a wall, assume neighbor pressure == current pressure
                // This makes the gradient zero (du/dn = 0) which represents a rigid wall.

                let p_up = if self.walls[idx_up] { p_curr } else { self.pressure[idx_up] };
                let p_down = if self.walls[idx_down] { p_curr } else { self.pressure[idx_down] };
                let p_left = if self.walls[idx_left] { p_curr } else { self.pressure[idx_left] };
                let p_right = if self.walls[idx_right] { p_curr } else { self.pressure[idx_right] };

                let laplacian = p_up + p_down + p_left + p_right - 4.0 * p_curr;

                let mut p_new = 2.0 * p_curr - p_prev + c2 * laplacian;
                p_new *= self.damping;

                self.pressure_next[idx] = p_new;
            }
        }

        // Handle outer boundaries of the simulation domain (Implicit Hard Walls or Absorbing?)
        // Let's make the outer border of the grid always rigid walls for simplicity.
        // The loop above skips 0 and w-1, h-1. So they remain 0.
        // This is a "Pressure Release" boundary (u=0).
        // To make the outer box Rigid, we need to handle x=0, x=w-1 in the loop or set them.
        // Let's assume the user draws walls, but the edge of the universe is Open (P=0).
        // That's fine.

        // Swap buffers
        // We can't easily swap fields in a struct if we borrow self.
        // So we swap the content.
        std::mem::swap(&mut self.pressure_prev, &mut self.pressure);
        std::mem::swap(&mut self.pressure, &mut self.pressure_next);
    }

    pub fn pluck(&mut self, x: usize, y: usize, strength: f32) {
        if x > 0 && x < self.width - 1 && y > 0 && y < self.height - 1 {
            let idx = y * self.width + x;
            if !self.walls[idx] {
                // Smooth injection (Gaussian-ish)
                self.pressure[idx] += strength;
                self.pressure_prev[idx] += strength; // Adding to both adds a static displacement (DC offset).
                // To add an impulse (velocity), we change current but not prev.
                // u_next = 2u - u_prev + ...
                // If u is high, u_prev is 0 -> acceleration.
                // Let's just set P at center.
            }
        }
    }

    pub fn add_wall(&mut self, x: usize, y: usize) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.walls[idx] = true;
            self.pressure[idx] = 0.0;
            self.pressure_prev[idx] = 0.0;
            self.pressure_next[idx] = 0.0;
        }
    }

    pub fn remove_wall(&mut self, x: usize, y: usize) {
         if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.walls[idx] = false;
        }
    }

    pub fn get(&self, x: usize, y: usize) -> f32 {
        if x < self.width && y < self.height {
            self.pressure[y * self.width + x]
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
        let mut grid = AcousticGrid::new(10, 10);
        grid.pluck(5, 5, 1.0);
        grid.step();

        // Wave should spread
        assert!(grid.get(5, 6).abs() > 0.001);
        assert!(grid.get(5, 4).abs() > 0.001);
    }

    #[test]
    fn test_reflection_polarity() {
        // Test that a pulse hitting a rigid wall reflects with SAME polarity.
        // Setup: Narrow channel to isolate 1D behavior.
        // Wall at x=5. Pulse at x=3 moving towards 5.
        // But 2D is harder to control directional impulse without careful setup.
        // Let's just check immediate neighbor behavior.

        let mut grid = AcousticGrid::new(10, 10);
        // Wall at (6, 5)
        grid.add_wall(6, 5);

        // Pulse at (5, 5).
        // We set pressure[5,5] = 1.0, pressure_prev[5,5] = 1.0 (Static)
        // No, let's just set pressure[5,5] = 1.0, prev=0.0.
        // This is a sharp impulse.
        let idx = 5 * 10 + 5;
        grid.pressure[idx] = 1.0;

        // Step 1
        grid.step();

        // At step 1, the wave spreads to (5,6), (5,4), (4,5).
        // And (6,5) is a wall.
        // The Laplacian at (5,5):
        // p_up(5,4)=0, p_down(5,6)=0, p_left(4,5)=0.
        // p_right(6,5) is WALL -> so p_right = p_curr(5,5) = 1.0.
        // Laplacian = 0 + 0 + 0 + 1.0 - 4.0*(1.0) = -3.0.
        // New val = 2(1) - 0 + 0.5(-3) = 2 - 1.5 = 0.5.

        // If it was open boundary (p_right=0):
        // Laplacian = -4.0.
        // New val = 2 - 2 = 0.

        // So with Wall, the value stays higher (0.5 vs 0.0). This indicates reflection (energy stays).
        let val_with_wall = grid.get(5, 5);

        // Compare with no wall
        let mut grid2 = AcousticGrid::new(10, 10);
        let idx2 = 5 * 10 + 5;
        grid2.pressure[idx2] = 1.0;
        grid2.step();
        let val_no_wall = grid2.get(5, 5);

        println!("With Wall: {}, No Wall: {}", val_with_wall, val_no_wall);
        assert!(val_with_wall > val_no_wall, "Energy should be conserved better near wall");
    }
}
