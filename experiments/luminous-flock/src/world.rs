use crate::boid::Boid;
use flocking::{FlockingParams, PhysicsState, compute_force};
use std::f64::consts::PI;

pub struct World {
    pub boids: Vec<Boid>,
    pub width: f64,
    pub height: f64,
}

impl World {
    pub fn new(width: f64, height: f64) -> Self {
        let mut boids = Vec::new();
        // Create a nice swarm
        for _ in 0..150 {
            boids.push(Boid::new(width / 2.0, height / 2.0));
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
        let physics_states: Vec<PhysicsState> = self.boids.iter().map(|b| b.physics).collect();

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

            let flocking_force = compute_force(&physics_states, i, &params);
            forces.push(flocking_force);

            // 2. Calculate Firefly Phase Nudge
            // This is domain-specific, so we keep it here.
            let mut nudge = 0.0;
            let coupling_radius_sq = boid.dna.coupling_radius.powi(2);
            let p1 = boid.position();

            for (j, other) in self.boids.iter().enumerate() {
                if i == j {
                    continue;
                }

                // Firefly coupling
                // If neighbor is flashing (timer == 5), it pulls us
                // Check distance
                if p1.distance_squared(other.position()) < coupling_radius_sq {
                    if other.flash_timer == 5 {
                        nudge += boid.dna.coupling_strength;
                    }
                }
            }
            phase_nudges[i] = nudge;
        }

        // Apply updates
        for (i, boid) in self.boids.iter_mut().enumerate() {
            // Apply physics
            boid.apply_force(forces[i]);
            boid.update_physics(self.width, self.height);

            // Apply phase update
            // Natural frequency + nudges
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
