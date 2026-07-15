use crate::boid::{Boid, ENTANGLEMENT_RADIUS, PERCEPTION_RADIUS};
use crate::qubit::apply_cnot_approx;
use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use rand::Rng;

pub struct World {
    pub boids: Vec<Boid>,
    pub width: f64,
    pub height: f64,
    /// ⚡ Bolt: Pre-allocated buffer to avoid O(N) heap allocations per frame when updating boids.
    pub boids_buffer: Vec<Boid>,
    /// ⚡ Bolt: Pre-allocated buffer to avoid collecting positions into a new Vec per frame.
    pub positions_buffer: Vec<Vec2>,
    /// ⚡ Bolt: Pre-allocated buffer to avoid collecting velocities into a new Vec per frame.
    pub velocities_buffer: Vec<Vec2>,
    /// ⚡ Bolt: Pre-allocated buffer to avoid creating a new forces Vec per frame.
    pub forces_buffer: Vec<Vec2>,
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
            boids_buffer: Vec::with_capacity(50),
            positions_buffer: Vec::with_capacity(50),
            velocities_buffer: Vec::with_capacity(50),
            forces_buffer: Vec::with_capacity(50),
        }
    }

    pub fn update(&mut self) {
        // Quantum interactions (Entanglement)
        let n = self.boids.len();
        self.boids_buffer.clear();
        self.boids_buffer.extend_from_slice(&self.boids);

        // ⚡ Bolt: Precalculate squared radius to avoid expensive O(N^2) square roots in inner loop.
        let entanglement_radius_sq = ENTANGLEMENT_RADIUS * ENTANGLEMENT_RADIUS;

        for i in 0..n {
            let boid = &mut self.boids_buffer[i];

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
                let dist_sq = pos_i.distance_squared(pos_j);

                if dist_sq < entanglement_radius_sq {
                    // Close proximity: Interact
                    // If not entangled, entangle
                    if boid.entangled_partner.is_none()
                        && self.boids[j].entangled_partner.is_none()
                        && rand::thread_rng().gen_bool(0.1)
                    {
                        boid.entangled_partner = Some(j);
                        // Partner update handled when j is processed or we can do it here via index lookups if careful
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

        std::mem::swap(&mut self.boids, &mut self.boids_buffer);

        // Physics Loop
        self.positions_buffer.clear();
        self.positions_buffer
            .extend(self.boids.iter().map(|b| b.position));
        self.velocities_buffer.clear();
        self.velocities_buffer
            .extend(self.boids.iter().map(|b| b.velocity));
        self.forces_buffer.clear();

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

            let force = compute_force(&self.positions_buffer, &self.velocities_buffer, i, &params);
            self.forces_buffer.push(force);
        }

        for (i, boid) in self.boids.iter_mut().enumerate() {
            boid.apply_force(self.forces_buffer[i]);
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_buffers_are_reused_avoiding_allocations() {
        let mut world = World::new(100.0, 100.0);

        // Assert buffers are initialized with capacity
        assert!(world.boids_buffer.capacity() >= 50);
        assert!(world.positions_buffer.capacity() >= 50);
        assert!(world.velocities_buffer.capacity() >= 50);
        assert!(world.forces_buffer.capacity() >= 50);

        world.update();

        // Assert buffers are still there and reused (capacity didn't shrink to 0)
        assert!(world.boids_buffer.capacity() >= 50);
        assert!(world.positions_buffer.capacity() >= 50);
        assert!(world.velocities_buffer.capacity() >= 50);
        assert!(world.forces_buffer.capacity() >= 50);
    }
}
