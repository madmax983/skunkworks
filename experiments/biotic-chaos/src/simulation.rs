use rayon::prelude::*;
use rand::Rng;

pub struct Lattice {
    pub width: usize,
    pub height: usize,
    pub cells: Vec<f32>,
    pub next_cells: Vec<f32>,
    pub r_map: Vec<f32>,
    pub epsilon: f32, // Coupling constant
}

impl Lattice {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width * height;
        let mut rng = rand::thread_rng();

        let cells = (0..size).map(|_| rng.gen_range(0.0..1.0)).collect();
        let next_cells = vec![0.0; size];
        // Initialize r_map with a gradient or interesting pattern
        let r_map = (0..size).map(|i| {
            let x = (i % width) as f32 / width as f32;
            let y = (i / width) as f32 / height as f32;
            // Base gradient (diagonal)
            let base_r = 2.8 + (x + y) * 0.6;
            // Add some noise
            let noise = rng.gen_range(-0.05..0.05);
            (base_r + noise).clamp(0.0, 4.0)
        }).collect();

        Self {
            width,
            height,
            cells,
            next_cells,
            r_map,
            epsilon: 0.1, // Default weak coupling
        }
    }

    pub fn update(&mut self) {
        let width = self.width;
        let height = self.height;
        let epsilon = self.epsilon;
        let cells = &self.cells;
        let r_map = &self.r_map;

        self.next_cells.par_iter_mut().enumerate().for_each(|(i, next_val)| {
            let x = i % width;
            let y = i / width;

            // Logistic map function
            let logistic = |val: f32, r: f32| r * val * (1.0 - val);

            // Neighbors (periodic boundary conditions)
            let left = if x == 0 { width - 1 } else { x - 1 };
            let right = if x == width - 1 { 0 } else { x + 1 };
            let top = if y == 0 { height - 1 } else { y - 1 };
            let bottom = if y == height - 1 { 0 } else { y + 1 };

            let idx_l = y * width + left;
            let idx_r = y * width + right;
            let idx_t = top * width + x;
            let idx_b = bottom * width + x;

            let val_c = cells[i];
            let r_c = r_map[i];

            let f_c = logistic(val_c, r_c);
            let f_l = logistic(cells[idx_l], r_map[idx_l]);
            let f_r = logistic(cells[idx_r], r_map[idx_r]);
            let f_t = logistic(cells[idx_t], r_map[idx_t]);
            let f_b = logistic(cells[idx_b], r_map[idx_b]);

            // CML Update Rule
            // x(t+1) = (1-e) * f(x(t)) + (e/4) * sum(f(neighbors))
            *next_val = (1.0 - epsilon) * f_c + (epsilon / 4.0) * (f_l + f_r + f_t + f_b);
        });

        std::mem::swap(&mut self.cells, &mut self.next_cells);
    }

    pub fn paint_r(&mut self, x: usize, y: usize, radius: usize, target_r: f32) {
        for dy in -(radius as isize)..=(radius as isize) {
            for dx in -(radius as isize)..=(radius as isize) {
                if dx*dx + dy*dy <= (radius * radius) as isize {
                    let nx = (x as isize + dx).rem_euclid(self.width as isize) as usize;
                    let ny = (y as isize + dy).rem_euclid(self.height as isize) as usize;
                    self.r_map[ny * self.width + nx] = target_r;
                }
            }
        }
    }

    pub fn perturb(&mut self, x: usize, y: usize, radius: usize) {
        let mut rng = rand::thread_rng();
        for dy in -(radius as isize)..=(radius as isize) {
            for dx in -(radius as isize)..=(radius as isize) {
                if dx*dx + dy*dy <= (radius * radius) as isize {
                    let nx = (x as isize + dx).rem_euclid(self.width as isize) as usize;
                    let ny = (y as isize + dy).rem_euclid(self.height as isize) as usize;
                    self.cells[ny * self.width + nx] = rng.gen_range(0.0..1.0);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_lattice_initialization() {
        let width = 10;
        let height = 10;
        let lattice = Lattice::new(width, height);

        assert_eq!(lattice.cells.len(), width * height);
        assert_eq!(lattice.r_map.len(), width * height);

        for val in lattice.cells {
            assert!(val >= 0.0 && val <= 1.0);
        }
    }

    #[test]
    fn test_update_bounds() {
        let width = 10;
        let height = 10;
        let mut lattice = Lattice::new(width, height);

        // Run a few updates
        for _ in 0..5 {
            lattice.update();
            for val in &lattice.cells {
                // Logistic map is [0, 1] -> [0, 1] if r in [0, 4] and x in [0, 1]
                assert!(*val >= 0.0 && *val <= 1.0, "Value {} out of bounds", val);
            }
        }
    }
}
