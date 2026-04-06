use crate::grid::{CellType, Grid};
use macroquad::prelude::*;

pub struct FluidSim {
    pub width: usize,
    pub height: usize,
    pub temperature: Vec<f32>,
    /// Double buffer for temperature to avoid allocations during diffuse_heat and apply_buoyancy
    pub temperature_buf: Vec<f32>,
    pub chem_a: Vec<f32>, // U (Substrate)
    /// Double buffer for chem_a to avoid allocations during update
    pub chem_a_buf: Vec<f32>,
    pub chem_b: Vec<f32>, // V (Activator/Boid Pheromone)
    /// Double buffer for chem_b to avoid allocations during update
    pub chem_b_buf: Vec<f32>,
}

impl FluidSim {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            temperature: vec![0.0; width * height],
            temperature_buf: vec![0.0; width * height],
            chem_a: vec![1.0; width * height], // Start with full substrate
            chem_a_buf: vec![1.0; width * height],
            chem_b: vec![0.0; width * height],
            chem_b_buf: vec![0.0; width * height],
        }
    }

    #[allow(dead_code)]
    pub fn get_temp(&self, x: usize, y: usize) -> f32 {
        if x >= self.width || y >= self.height {
            0.0
        } else {
            self.temperature[y * self.width + x]
        }
    }

    pub fn get_chem_a(&self, x: usize, y: usize) -> f32 {
        if x >= self.width || y >= self.height {
            0.0
        } else {
            self.chem_a[y * self.width + x]
        }
    }

    pub fn get_chem_b(&self, x: usize, y: usize) -> f32 {
        if x >= self.width || y >= self.height {
            0.0
        } else {
            self.chem_b[y * self.width + x]
        }
    }

    #[allow(dead_code)]
    pub fn add_heat(&mut self, x: usize, y: usize, amount: f32) {
        if x < self.width && y < self.height {
            self.temperature[y * self.width + x] += amount;
        }
    }

    pub fn add_chem_b(&mut self, x: usize, y: usize, amount: f32) {
        if x < self.width && y < self.height {
            self.chem_b[y * self.width + x] += amount;
        }
    }

    pub fn update(&mut self, grid: &Grid, dt: f32) {
        self.diffuse_heat(grid, dt);
        self.apply_buoyancy(dt);

        // Reaction-Diffusion (Gray-Scott)
        // du/dt = Da*Laplacian(u) - u*v^2 + f*(1-u)
        // dv/dt = Db*Laplacian(v) + u*v^2 - (k+f)*v

        let da = 1.0;
        let db = 0.5;
        let feed = 0.055;
        let kill = 0.062;

        self.chem_a_buf.copy_from_slice(&self.chem_a);
        self.chem_b_buf.copy_from_slice(&self.chem_b);

        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                if grid
                    .get(x, y)
                    .map(|c| c.cell_type == CellType::Rock || c.cell_type == CellType::Chimney)
                    .unwrap_or(false)
                {
                    continue;
                }

                let idx = y * self.width + x;
                let u = self.chem_a[idx];
                let v = self.chem_b[idx];

                // Laplacian
                let mut sum_a = 0.0;
                let mut sum_b = 0.0;

                // Convolution kernel:
                // 0.05 0.2 0.05
                // 0.2  -1  0.2
                // 0.05 0.2 0.05

                let neighbors = [
                    (x, y, -1.0),
                    (x + 1, y, 0.2),
                    (x - 1, y, 0.2),
                    (x, y + 1, 0.2),
                    (x, y - 1, 0.2),
                    (x + 1, y + 1, 0.05),
                    (x - 1, y - 1, 0.05),
                    (x + 1, y - 1, 0.05),
                    (x - 1, y + 1, 0.05),
                ];

                for (nx, ny, w) in neighbors.iter() {
                    let n_idx = ny * self.width + nx;
                    sum_a += self.chem_a[n_idx] * w;
                    sum_b += self.chem_b[n_idx] * w;
                }

                let reaction = u * v * v;

                let du = da * sum_a - reaction + feed * (1.0 - u);
                let dv = db * sum_b + reaction - (feed + kill) * v;

                self.chem_a_buf[idx] = (u + du * dt * 10.0).clamp(0.0, 1.0); // Speed up
                self.chem_b_buf[idx] = (v + dv * dt * 10.0).clamp(0.0, 1.0);
            }
        }

        std::mem::swap(&mut self.chem_a, &mut self.chem_a_buf);
        std::mem::swap(&mut self.chem_b, &mut self.chem_b_buf);

        // Cooling
        for t in self.temperature.iter_mut() {
            *t *= 0.99; // Cooling
        }
    }

    fn diffuse_heat(&mut self, grid: &Grid, _dt: f32) {
        self.temperature_buf.copy_from_slice(&self.temperature);

        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                if grid
                    .get(x, y)
                    .map(|c| c.cell_type == CellType::Rock || c.cell_type == CellType::Chimney)
                    .unwrap_or(false)
                {
                    continue;
                }

                let idx = y * self.width + x;
                let mut sum = self.temperature[idx];
                let mut count = 1.0;

                let neighbors = [(x + 1, y), (x - 1, y), (x, y + 1), (x, y - 1)];

                for (nx, ny) in neighbors.iter() {
                    if !grid
                        .get(*nx, *ny)
                        .map(|c| c.cell_type == CellType::Rock || c.cell_type == CellType::Chimney)
                        .unwrap_or(true)
                    {
                        sum += self.temperature[ny * self.width + nx];
                        count += 1.0;
                    }
                }

                self.temperature_buf[idx] = sum / count;
            }
        }
        std::mem::swap(&mut self.temperature, &mut self.temperature_buf);
    }

    fn apply_buoyancy(&mut self, _dt: f32) {
        self.temperature_buf.copy_from_slice(&self.temperature);
        for y in 1..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let up_idx = (y - 1) * self.width + x;

                if self.temperature[idx] > 0.01 {
                    let amount = self.temperature[idx] * 0.1;
                    if y > 0 {
                        self.temperature_buf[up_idx] += amount;
                        self.temperature_buf[idx] -= amount;
                    }
                }
            }
        }
        std::mem::swap(&mut self.temperature, &mut self.temperature_buf);
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buoyancy() {
        let mut fluid = FluidSim::new(3, 3);
        // Middle cell
        fluid.temperature[4] = 1.0;

        fluid.apply_buoyancy(0.1);

        // Heat should move up
        let amount = 1.0 * 0.1;
        assert_eq!(fluid.temperature[1], amount); // up
        assert_eq!(fluid.temperature[4], 1.0 - amount); // original
    }
}
