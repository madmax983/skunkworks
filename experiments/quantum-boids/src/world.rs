use crate::boid::{Boid, ENTANGLEMENT_RADIUS, PERCEPTION_RADIUS};
use crate::qubit::apply_cnot_approx;
use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use rand::Rng;

pub struct World {
    pub boids: Vec<Boid>,
    pub width: f64,
    pub height: f64,
}

impl World {
    pub fn new(width: f64, height: f64) -> Self {
        let mut boids = Vec::new();
        for _ in 0..50 {
            boids.push(Boid::new(width / 2.0, height / 2.0));
        }
        Self {
            boids,
            width,
            height,
        }
    }

    pub fn update(&mut self) {
        // Quantum interactions (Entanglement)
        let n = self.boids.len();
        let mut new_boids = self.boids.clone();

        for i in 0..n {
            let boid = &mut new_boids[i];

            // Randomly apply H gate (Quantum fluctuations)
            if rand::thread_rng().gen_bool(0.01) {
                boid.qubit.h();
            }

            // Entanglement logic
            for j in 0..n {
                if i == j {
                    continue;
                }

                let pos_i = boid.position();
                let pos_j = self.boids[j].position();
                let dist = pos_i.distance(pos_j);

                if dist < ENTANGLEMENT_RADIUS {
                    // Close proximity: Interact
                    // If not entangled, entangle
                    if boid.entangled_partner.is_none() && self.boids[j].entangled_partner.is_none()
                    {
                        if rand::thread_rng().gen_bool(0.1) {
                            boid.entangled_partner = Some(j);
                            // Partner update handled when j is processed or we can do it here via index lookups if careful
                        }
                    }
                }
            }

            // Apply entanglement correlation
            if let Some(partner_idx) = boid.entangled_partner {
                // If partner exists, synchronize states or apply CNOT
                // We use self.boids (old state) for partner to avoid index confusion with new_boids
                let partner = &self.boids[partner_idx];

                // Simplified "Spooky Action": If partner is likely |1>, I tend to flip
                // Or: CNOT gate where partner is Control, I am Target
                apply_cnot_approx(&partner.qubit, &mut boid.qubit);
            }
        }

        self.boids = new_boids;

        // Physics Loop
        let positions: Vec<Vec2> = self.boids.iter().map(|b| b.position).collect();
        let velocities: Vec<Vec2> = self.boids.iter().map(|b| b.velocity).collect();
        let mut forces = Vec::with_capacity(n);

        for (i, boid) in self.boids.iter().enumerate() {
            // Quantum Weighting
            // If Prob(|1>) is high, prefer Separation (Scatter)
            // If Prob(|0>) is high, prefer Cohesion (Gather)
            let p_one = boid.qubit.prob_one();
            let align_w = 1.0;
            let coh_w = 1.0 + (1.0 - p_one);
            let sep_w = 1.0 + p_one * 2.0;

            let params = FlockingParams {
                view_radius: PERCEPTION_RADIUS,
                separation_radius: PERCEPTION_RADIUS / 2.0,
                max_speed: boid.current_max_speed,
                max_force: 0.05,
                separation_weight: sep_w,
                alignment_weight: align_w,
                cohesion_weight: coh_w,
            };

            let force = compute_force(&positions, &velocities, i, &params);
            forces.push(force);
        }

        for (i, boid) in self.boids.iter_mut().enumerate() {
            boid.apply_force(forces[i]);
            boid.update(self.width, self.height);
        }
    }

    pub fn measure_all(&mut self) {
        for boid in &mut self.boids {
            boid.qubit.measure();
            boid.entangled_partner = None; // Measurement breaks entanglement
        }
    }
}
