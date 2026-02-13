use macroquad::prelude::{Color, WHITE};
use fastrand;
use rayon::prelude::*;

#[derive(Clone, Copy, Debug)]
pub struct Blueprint {
    pub build_threshold: f32,
    pub destroy_threshold: f32,
    pub optimal_density: f32,
    pub randomness: f32,
    pub color: Color,
}

impl Default for Blueprint {
    fn default() -> Self {
        Self {
            build_threshold: 0.5,
            destroy_threshold: 0.2,
            optimal_density: 0.3,
            randomness: 0.1,
            color: WHITE,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Termite {
    pub x: f32,
    pub y: f32,
    pub blueprint: Blueprint,
    pub fitness: f32,
    pub carrying_wall: bool,
}

impl Termite {
    pub fn new(x: f32, y: f32) -> Self {
        Self {
            x,
            y,
            blueprint: Blueprint::default(),
            fitness: 0.0,
            carrying_wall: false,
        }
    }

    pub fn update(&mut self, grid: &mut Grid) {
        if self.blueprint.randomness > 0.0 {
            self.x += (fastrand::f32() - 0.5) * 2.0;
            self.y += (fastrand::f32() - 0.5) * 2.0;
        }

        // Clamp to bounds
        self.x = self.x.clamp(0.0, grid.width as f32 - 0.1);
        self.y = self.y.clamp(0.0, grid.height as f32 - 0.1);

        let ix = self.x as usize;
        let iy = self.y as usize;

        let local_heat = grid.get_heat(ix, iy);

        // Update Fitness (lower heat is better)
        // Fitness = 1.0 - local_heat (clamped)
        self.fitness = (1.0 - local_heat).clamp(0.0, 1.0);

        // Build logic
        if local_heat > self.blueprint.build_threshold {
            if !grid.is_wall(ix, iy) {
                 if self.blueprint.randomness == 0.0 || fastrand::f32() > 0.5 {
                     grid.set_wall(ix, iy, true);
                 }
            }
        } else if local_heat < self.blueprint.destroy_threshold {
             if grid.is_wall(ix, iy) {
                 grid.set_wall(ix, iy, false);
             }
        }
    }

    pub fn interact(&mut self, other: &Termite) {
        if other.fitness > self.fitness {
            // Adopt better blueprint
            self.blueprint = other.blueprint;
            // Mutation?
            if fastrand::f32() < 0.01 {
                self.blueprint.build_threshold += (fastrand::f32() - 0.5) * 0.1;
                // Change color slightly
                // Not easy with macroquad Color struct without new methods
                // Just keep same or pick new random?
            }
        }
    }
}

pub struct Grid {
    pub width: usize,
    pub height: usize,
    pub heat: Vec<f32>,
    pub wall: Vec<bool>,
    pub pheromone: Vec<f32>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            heat: vec![0.0; width * height],
            wall: vec![false; width * height],
            pheromone: vec![0.0; width * height],
        }
    }

    pub fn update_heat(&mut self) {
        let w = self.width;
        let h = self.height;

        // Parallel update using Rayon
        // We need immutable access to 'current' state (self.heat, self.wall)
        // and mutable access to 'next' state.
        // We create a new vector for next_heat.

        let current_heat = &self.heat;
        let wall = &self.wall;

        let next_heat: Vec<f32> = (0..w*h).into_par_iter().map(|idx| {
            let x = idx % w;
            let y = idx / w;

            // Boundary or Wall check
            if x == 0 || x == w - 1 || y == 0 || y == h - 1 {
                return 0.0; // Cool boundary
            }

            if wall[idx] {
                 return current_heat[idx] * 0.99; // Insulate
            }

            let sum = current_heat[idx - 1] +
                      current_heat[idx + 1] +
                      current_heat[idx - w] +
                      current_heat[idx + w];

            let k = 0.2;
            let val = current_heat[idx] + k * (sum - 4.0 * current_heat[idx]);
            val * 0.999 // Cooling
        }).collect();

        self.heat = next_heat;
    }

    pub fn get_heat(&self, x: usize, y: usize) -> f32 {
        if x < self.width && y < self.height {
            self.heat[y * self.width + x]
        } else {
            0.0
        }
    }

    pub fn set_heat(&mut self, x: usize, y: usize, val: f32) {
         if x < self.width && y < self.height {
            self.heat[y * self.width + x] = val;
        }
    }

    pub fn is_wall(&self, x: usize, y: usize) -> bool {
        if x < self.width && y < self.height {
            self.wall[y * self.width + x]
        } else {
            true // Out of bounds is wall
        }
    }

    pub fn set_wall(&mut self, x: usize, y: usize, val: bool) {
        if x < self.width && y < self.height {
            self.wall[y * self.width + x] = val;
        }
    }
}
