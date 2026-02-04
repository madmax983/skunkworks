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

        std::mem::swap(&mut self.pressure_prev, &mut self.pressure);
        std::mem::swap(&mut self.pressure, &mut self.pressure_next);
    }

    pub fn pluck(&mut self, x: usize, y: usize, strength: f32) {
        if x > 0 && x < self.width - 1 && y > 0 && y < self.height - 1 {
            let idx = y * self.width + x;
            if !self.walls[idx] {
                self.pressure[idx] += strength;
                self.pressure_prev[idx] += strength;
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
}
