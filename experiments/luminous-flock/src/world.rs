use crate::boid::{Boid, distance_squared, limit};
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
        Self { boids, width, height }
    }

    pub fn update(&mut self) {
        let count = self.boids.len();
        let mut physics_forces = Vec::with_capacity(count);
        let mut phase_nudges = vec![0.0; count];

        // Combined O(N^2) loop for physics and synchronization
        for i in 0..count {
            let mut separation = (0.0, 0.0);
            let mut alignment = (0.0, 0.0);
            let mut cohesion = (0.0, 0.0);

            let mut sep_count = 0;
            let mut ali_count = 0;
            let mut coh_count = 0;

            let mut nudge = 0.0;

            // We need to access boids[i] multiple times, so we clone the necessary data
            // to avoid borrowing issues while iterating over the rest
            let p1 = self.boids[i].position;
            let v1 = self.boids[i].velocity;
            let dna = &self.boids[i].dna;

            let view_radius_sq = dna.view_radius.powi(2);
            let coupling_radius_sq = dna.coupling_radius.powi(2);
            let separation_radius_sq = (dna.view_radius / 2.0).powi(2);

            for j in 0..count {
                if i == j { continue; }

                let b2 = &self.boids[j];
                let d_sq = distance_squared(p1, b2.position);

                if d_sq == 0.0 { continue; }

                // --- Flocking Logic ---
                if d_sq < view_radius_sq {
                    // Separation
                    if d_sq < separation_radius_sq {
                        let diff = (p1.0 - b2.position.0, p1.1 - b2.position.1);
                        separation.0 += diff.0 / d_sq;
                        separation.1 += diff.1 / d_sq;
                        sep_count += 1;
                    }

                    // Alignment
                    alignment.0 += b2.velocity.0;
                    alignment.1 += b2.velocity.1;
                    ali_count += 1;

                    // Cohesion
                    cohesion.0 += b2.position.0;
                    cohesion.1 += b2.position.1;
                    coh_count += 1;
                }

                // --- Firefly Logic ---
                // If neighbor is flashing (timer == 5), it pulls us
                if d_sq < coupling_radius_sq && b2.flash_timer == 5 {
                    nudge += dna.coupling_strength;
                }
            }

            // Calculate final steering forces
            let mut total_force = (0.0, 0.0);

            if sep_count > 0 {
                let len = (separation.0.powi(2) + separation.1.powi(2)).sqrt();
                if len > 0.0 {
                    separation.0 = (separation.0 / len) * dna.max_speed;
                    separation.1 = (separation.1 / len) * dna.max_speed;
                    separation.0 -= v1.0;
                    separation.1 -= v1.1;
                    separation = limit(separation, dna.max_force);
                    total_force.0 += separation.0 * dna.separation_weight;
                    total_force.1 += separation.1 * dna.separation_weight;
                }
            }

            if ali_count > 0 {
                alignment.0 /= ali_count as f64;
                alignment.1 /= ali_count as f64;
                let len = (alignment.0.powi(2) + alignment.1.powi(2)).sqrt();
                if len > 0.0 {
                    alignment.0 = (alignment.0 / len) * dna.max_speed;
                    alignment.1 = (alignment.1 / len) * dna.max_speed;
                    alignment.0 -= v1.0;
                    alignment.1 -= v1.1;
                    alignment = limit(alignment, dna.max_force);
                    total_force.0 += alignment.0 * dna.alignment_weight;
                    total_force.1 += alignment.1 * dna.alignment_weight;
                }
            }

            if coh_count > 0 {
                cohesion.0 /= coh_count as f64;
                cohesion.1 /= coh_count as f64;
                let mut desired = (cohesion.0 - p1.0, cohesion.1 - p1.1);
                let len = (desired.0.powi(2) + desired.1.powi(2)).sqrt();
                if len > 0.0 {
                    desired.0 = (desired.0 / len) * dna.max_speed;
                    desired.1 = (desired.1 / len) * dna.max_speed;
                    desired.0 -= v1.0;
                    desired.1 -= v1.1;
                    desired = limit(desired, dna.max_force);
                    total_force.0 += desired.0 * dna.cohesion_weight;
                    total_force.1 += desired.1 * dna.cohesion_weight;
                }
            }

            physics_forces.push(total_force);
            phase_nudges[i] = nudge;
        }

        // Apply updates
        for (i, boid) in self.boids.iter_mut().enumerate() {
            // Apply physics
            boid.apply_force(physics_forces[i]);
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
