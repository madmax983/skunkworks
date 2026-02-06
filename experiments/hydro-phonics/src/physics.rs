
use rayon::prelude::*;

pub struct WaveGrid {
    pub width: usize,
    pub height: usize,
    // We use flat vectors for cache locality
    pub current: Vec<f32>,
    pub previous: Vec<f32>,
    pub next: Vec<f32>,
    pub damping: f32,
}

impl WaveGrid {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        Self {
            width,
            height,
            current: vec![0.0; size],
            previous: vec![0.0; size],
            next: vec![0.0; size],
            damping: 0.99, // Energy loss per step
        }
    }

    pub fn update(&mut self) {
        let w = self.width;
        let h = self.height;
        let c_sq = 0.5; // Wave speed coefficient squared. Needs to be < 0.5 for stability

        // Parallel update
        // We compute 'next' based on 'current' and 'previous'
        self.next.par_iter_mut().enumerate().for_each(|(i, next_val)| {
            let x = i % w;
            let y = i / w;

            if x == 0 || x == w - 1 || y == 0 || y == h - 1 {
                // Fixed boundary conditions (walls)
                *next_val = 0.0;
                return;
            }

            let current_val = self.current[i];
            let prev_val = self.previous[i];

            // Laplacian (simple 5-point stencil)
            let up = self.current[i - w];
            let down = self.current[i + w];
            let left = self.current[i - 1];
            let right = self.current[i + 1];

            let laplacian = up + down + left + right - 4.0 * current_val;

            // Discrete wave equation
            let new_val = 2.0 * current_val - prev_val + c_sq * laplacian;

            // Damping
            *next_val = new_val * self.damping;
        });

        // Rotate buffers
        // We want:
        // previous <- current (Old C)
        // current <- next (New Data)
        // next <- previous (Old P, used as scratch)

        std::mem::swap(&mut self.previous, &mut self.current); // P gets C, C gets P
        std::mem::swap(&mut self.current, &mut self.next);     // C gets N, N gets P (which was C) -> No wait.

        // Trace:
        // Initial: P=OldP, C=OldC, N=NewData
        // Swap(P, C): P=OldC, C=OldP
        // Swap(C, N): C=NewData, N=OldP
        // Result: P=OldC, C=NewData, N=OldP. Correct.
    }

    pub fn perturb(&mut self, x: usize, y: usize, amount: f32) {
        if x > 0 && x < self.width - 1 && y > 0 && y < self.height - 1 {
            let idx = y * self.width + x;
            self.current[idx] += amount;
        }
    }

    pub fn get_height(&self, x: usize, y: usize) -> f32 {
        if x < self.width && y < self.height {
            self.current[y * self.width + x]
        } else {
            0.0
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_initialization() {
        let grid = WaveGrid::new(10, 10);
        assert_eq!(grid.current.len(), 100);
        assert_eq!(grid.get_height(5, 5), 0.0);
    }

    #[test]
    fn test_perturbation() {
        let mut grid = WaveGrid::new(10, 10);
        grid.perturb(5, 5, 10.0);
        assert_eq!(grid.get_height(5, 5), 10.0);
    }

    #[test]
    fn test_propagation() {
        let mut grid = WaveGrid::new(10, 10);
        grid.perturb(5, 5, 10.0);

        // Step 1
        grid.update();
        // Center should decrease, neighbors should increase
        // center = 2*10 - 0 + 0.5 * (0+0+0+0 - 4*10) = 20 - 20 = 0?
        // Wait, 2*u - u_prev + c^2 * (-4u) = u(2 - 4c^2) - u_prev
        // if c^2 = 0.5, 2 - 2 = 0. So center becomes 0.
        // Neighbors: 2*0 - 0 + 0.5 * (10 + 0 + 0 + 0 - 0) = 5.

        // Actually damping applies.
        let center = grid.get_height(5, 5);
        let neighbor = grid.get_height(5, 4);

        assert!(center < 10.0);
        assert!(neighbor > 0.0);
    }
}
