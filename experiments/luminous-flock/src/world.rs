use crate::boid::Boid;
use std::f64::consts::PI;
use tui_shared::math::Vec2;

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
        let mut physics_forces = Vec::with_capacity(count);
        let mut phase_nudges = vec![0.0; count];

        // Combined O(N^2) loop for physics and synchronization
        for (i, nudge_out) in phase_nudges.iter_mut().enumerate() {
            let mut separation = Vec2::zero();
            let mut alignment = Vec2::zero();
            let mut cohesion = Vec2::zero();

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
                if i == j {
                    continue;
                }

                let b2 = &self.boids[j];
                let d_sq = p1.distance_squared(b2.position);

                if d_sq == 0.0 {
                    continue;
                }

                // --- Flocking Logic ---
                if d_sq < view_radius_sq {
                    // Separation
                    if d_sq < separation_radius_sq {
                        let diff = p1 - b2.position;
                        separation += diff / d_sq;
                        sep_count += 1;
                    }

                    // Alignment
                    alignment += b2.velocity;
                    ali_count += 1;

                    // Cohesion
                    cohesion += b2.position;
                    coh_count += 1;
                }

                // --- Firefly Logic ---
                // If neighbor is flashing (timer == 5), it pulls us
                if d_sq < coupling_radius_sq && b2.flash_timer == 5 {
                    nudge += dna.coupling_strength;
                }
            }

            // Calculate final steering forces
            let mut total_force = Vec2::zero();

            if sep_count > 0 && separation.magnitude_squared() > 0.0 {
                separation = separation.normalize() * dna.max_speed;
                separation -= v1;
                separation = separation.limit(dna.max_force);
                total_force += separation * dna.separation_weight;
            }

            if ali_count > 0 {
                alignment /= ali_count as f64;
                if alignment.magnitude_squared() > 0.0 {
                    alignment = alignment.normalize() * dna.max_speed;
                    alignment -= v1;
                    alignment = alignment.limit(dna.max_force);
                    total_force += alignment * dna.alignment_weight;
                }
            }

            if coh_count > 0 {
                cohesion /= coh_count as f64;
                let mut desired = cohesion - p1;
                if desired.magnitude_squared() > 0.0 {
                    desired = desired.normalize() * dna.max_speed;
                    desired -= v1;
                    desired = desired.limit(dna.max_force);
                    total_force += desired * dna.cohesion_weight;
                }
            }

            physics_forces.push(total_force);
            *nudge_out = nudge;
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
