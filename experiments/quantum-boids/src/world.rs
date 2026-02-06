use rand::Rng;
use crate::boid::{Boid, ENTANGLEMENT_RADIUS};
use crate::qubit::apply_cnot_approx;

pub struct World {
    pub boids: Vec<Boid>,
    pub width: f64,
    pub height: f64,
}

impl World {
    pub fn new(width: f64, height: f64) -> Self {
        let mut boids = Vec::new();
        for i in 0..50 {
            boids.push(Boid::new(width / 2.0, height / 2.0, i));
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
                if i == j { continue; }

                let dist = distance(boid.position, self.boids[j].position);
                if dist < ENTANGLEMENT_RADIUS {
                    // Close proximity: Interact
                    // If not entangled, entangle
                    if boid.entangled_partner.is_none() && self.boids[j].entangled_partner.is_none() {
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

        // Physics update
        // We can't clone boids inside the loop easily for flocking, so we do it in two passes
        // Pass 1: Calculate forces (needs read access to all)
        // Pass 2: Apply updates
        // Since Boid struct is simple copy, let's just do it

        let old_boids = self.boids.clone();
        for boid in &mut self.boids {
             boid.flock(&old_boids); // Using old positions for flocking calculation
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

fn distance(a: (f64, f64), b: (f64, f64)) -> f64 {
    ((a.0 - b.0).powi(2) + (a.1 - b.1).powi(2)).sqrt()
}
