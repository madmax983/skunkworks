use crate::boid::Boid;
use crate::git::Commit;
use locus::Vec2;
use locus::flocking::{FlockingParams, compute_force};
use rand::Rng;
use std::f64::consts::PI;

pub struct World {
    pub boids: Vec<Boid>,
    pub width: f64,
    pub height: f64,
}

impl World {
    pub fn new(width: f64, height: f64, commits: Vec<Commit>) -> Self {
        let mut boids = Vec::new();
        let mut rng = rand::thread_rng();

        for commit in commits {
            // Spawn them somewhat near the center
            let x = width / 2.0 + rng.gen_range(-20.0..20.0);
            let y = height / 2.0 + rng.gen_range(-20.0..20.0);
            boids.push(Boid::new(x, y, commit));
        }

        Self {
            boids,
            width,
            height,
        }
    }

    pub fn update(&mut self) {
        let count = self.boids.len();

        // Extract physics states for the flocking algorithm
        let positions: Vec<Vec2> = self.boids.iter().map(|b| b.position).collect();
        let velocities: Vec<Vec2> = self.boids.iter().map(|b| b.velocity).collect();

        let mut forces = Vec::with_capacity(count);
        let mut phase_nudges = vec![0.0; count];

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

            let flocking_force = compute_force(&positions, &velocities, i, &params);
            forces.push(flocking_force);

            // 2. Calculate Firefly Phase Nudge
            let mut nudge = 0.0;
            let coupling_radius_sq = boid.dna.coupling_radius.powi(2);
            let p1 = boid.position();

            for (j, other) in self.boids.iter().enumerate() {
                if i == j {
                    continue;
                }

                if p1.distance_squared(other.position()) < coupling_radius_sq
                    && other.flash_timer == 5
                {
                    nudge += boid.dna.coupling_strength;
                }
            }
            phase_nudges[i] = nudge;
        }

        // Apply updates
        for (i, boid) in self.boids.iter_mut().enumerate() {
            boid.apply_force(forces[i]);
            boid.update_physics(self.width, self.height);

            boid.phase += boid.dna.natural_freq + phase_nudges[i];
            boid.update_flash();
        }
    }

    pub fn synchronization_index(&self) -> f64 {
        let mut sum_sin = 0.0;
        let mut sum_cos = 0.0;

        for b in &self.boids {
            let theta = b.phase * 2.0 * PI;
            sum_sin += theta.sin();
            sum_cos += theta.cos();
        }

        let n = self.boids.len() as f64;
        if n == 0.0 {
            return 0.0;
        }
        ((sum_sin / n).powi(2) + (sum_cos / n).powi(2)).sqrt()
    }
}
