use crate::grid::{CellType, Grid};
use macroquad::prelude::*;

pub struct FluidSim {
    pub width: usize,
    pub height: usize,
    pub temperature: Vec<f32>,
}

impl FluidSim {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            temperature: vec![0.0; width * height],
        }
    }

    pub fn get_temp(&self, x: usize, y: usize) -> f32 {
        if x >= self.width || y >= self.height {
            0.0
        } else {
            self.temperature[y * self.width + x]
        }
    }

    pub fn add_heat(&mut self, x: usize, y: usize, amount: f32) {
        if x < self.width && y < self.height {
            self.temperature[y * self.width + x] += amount;
        }
    }

    pub fn update(&mut self, grid: &Grid, dt: f32) {
        self.diffuse_heat(grid, dt);
        self.apply_buoyancy(dt);
        // Cool down
        for t in self.temperature.iter_mut() {
            *t *= 0.99; // Cooling
        }
    }

    fn diffuse_heat(&mut self, grid: &Grid, _dt: f32) {
        let mut new_temp = self.temperature.clone();

        for y in 1..self.height - 1 {
            for x in 1..self.width - 1 {
                // Obstacles block diffusion
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

                new_temp[idx] = sum / count;
            }
        }
        self.temperature = new_temp;
    }

    fn apply_buoyancy(&mut self, _dt: f32) {
        // Simple advection up
        let mut new_temp = self.temperature.clone();
        for y in 1..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                let up_idx = (y - 1) * self.width + x;

                if self.temperature[idx] > 0.01 {
                    // Heat rises
                    let amount = self.temperature[idx] * 0.1;
                    if y > 0 {
                        new_temp[up_idx] += amount;
                        new_temp[idx] -= amount;
                    }
                }
            }
        }
        self.temperature = new_temp;
    }
}
