use crate::boid::Boid;
use locus::flocking::{compute_force, FlockingParams};
use locus::Vec2;
use rayon::prelude::*;
use std::f64::consts::PI;

pub struct World {
    pub boids: Vec<Boid>,
    pub width: f64,
    pub height: f64,
    pub trails: Vec<f64>,
    pub next_trails: Vec<f64>,
}

impl World {
    pub fn new(width: f64, height: f64) -> Self {
        let mut boids = Vec::new();
        // Create a nice swarm
        for _ in 0..250 {
            boids.push(Boid::new(width / 2.0, height / 2.0));
        }
        let grid_size = (width as usize) * (height as usize);
        Self {
            boids,
            width,
            height,
            trails: vec![0.0; grid_size],
            next_trails: vec![0.0; grid_size],
        }
    }

    pub fn get_trail(&self, x: usize, y: usize) -> f64 {
        let w = self.width as usize;
        let h = self.height as usize;
        if x >= w || y >= h {
            return 0.0;
        }
        self.trails[y * w + x]
    }

    pub fn set_trail(&mut self, x: usize, y: usize, value: f64) {
        let w = self.width as usize;
        let h = self.height as usize;
        if x < w && y < h {
            self.trails[y * w + x] = value;
        }
    }

    pub fn update(&mut self) {
        let count = self.boids.len();

        // Extract physics states for the flocking algorithm
        let positions: Vec<Vec2> = self.boids.iter().map(|b| b.position).collect();
        let velocities: Vec<Vec2> = self.boids.iter().map(|b| b.velocity).collect();

        let mut forces = Vec::with_capacity(count);

        for (i, boid) in self.boids.iter().enumerate() {
            // 1. Calculate Flocking Force
            let params = FlockingParams {
                view_radius: boid.dna.view_radius,
                separation_radius: boid.dna.view_radius / 2.0,
                max_speed: boid.dna.max_speed,
                max_force: boid.dna.max_force,
                separation_weight: boid.dna.separation_weight,
                alignment_weight: boid.dna.alignment_weight,
                cohesion_weight: boid.dna.cohesion_weight,
            };

            let mut total_force = compute_force(&positions, &velocities, i, &params);

            // 2. Calculate Pheromone Attraction Force
            let sensor_dist = boid.dna.view_radius;
            let sensor_angles = [
                -PI / 4.0, // Left
                0.0,       // Center
                PI / 4.0,  // Right
            ];

            let heading = boid.velocity.y.atan2(boid.velocity.x);
            let mut best_angle = 0.0;
            let mut max_trail = -1.0;

            for &angle_offset in &sensor_angles {
                let sensor_angle = heading + angle_offset;
                let sx = boid.position.x + sensor_angle.cos() * sensor_dist;
                let sy = boid.position.y + sensor_angle.sin() * sensor_dist;

                let wrap_x = (sx.rem_euclid(self.width)) as usize;
                let wrap_y = (sy.rem_euclid(self.height)) as usize;

                let val = self.get_trail(wrap_x, wrap_y);
                if val > max_trail {
                    max_trail = val;
                    best_angle = sensor_angle;
                }
            }

            // If there's a strong pheromone trail nearby, steer towards it
            if max_trail > 5.0 {
                let desired = Vec2::new(best_angle.cos(), best_angle.sin()) * boid.dna.max_speed;
                let mut steer = desired - boid.velocity;
                steer = steer.limit(boid.dna.max_force);
                total_force += steer * boid.dna.pheromone_attraction_weight;
            }

            forces.push(total_force);
        }

        // Apply updates and deposit pheromones
        for (i, boid) in self.boids.iter_mut().enumerate() {
            // Apply physics
            boid.apply_force(forces[i]);
            boid.update_physics(self.width, self.height);

            // Deposit pheromones at new position
            let ix = boid.position.x as usize;
            let iy = boid.position.y as usize;

            if ix < self.width as usize && iy < self.height as usize {
                let idx = iy * (self.width as usize) + ix;
                self.trails[idx] = (self.trails[idx] + boid.dna.pheromone_deposit_amount).min(255.0);
            }
        }

        // Diffuse and Decay the pheromone grid
        self.diffuse_and_decay();
    }

    pub fn diffuse_and_decay(&mut self) {
        let decay_factor = 0.9;
        let width = self.width as usize;
        let height = self.height as usize;

        let trails_ref = &self.trails;

        self.next_trails
            .par_chunks_mut(width)
            .enumerate()
            .for_each(|(y, row)| {
                for (x, pixel) in row.iter_mut().enumerate() {
                    let mut sum = 0.0;
                    for dy in -1..=1 {
                        for dx in -1..=1 {
                            let nx = (x as isize + dx).rem_euclid(width as isize) as usize;
                            let ny = (y as isize + dy).rem_euclid(height as isize) as usize;
                            sum += trails_ref[ny * width + nx];
                        }
                    }
                    let avg = sum / 9.0;
                    *pixel = avg * decay_factor;
                }
            });

        std::mem::swap(&mut self.trails, &mut self.next_trails);
    }
}
