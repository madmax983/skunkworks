//! # Hyperbolic Swarm Morphogenesis
//!
//! This hybrid experiment maps a Boids flocking simulation into the Poincaré disk model
//! of hyperbolic geometry.
//!
//! **Lineage:**
//! - Parent A (`crates/flocking`): Provides the behavioral intent for autonomous agents (Boids),
//!   calculating Separation, Alignment, and Cohesion forces.
//! - Parent B (`crates/poincare-disk`): Provides the non-Euclidean boundary physics. The
//!   distance metrics and positional updates are converted from linear Euclidean math
//!   to Möbius additions within the unit disk.
//!
//! **Emergent Phenotype:**
//! Agents navigate a space that expands exponentially towards the boundary. As they move
//! outward, their Euclidean perception is warped. A swarm trying to maintain cohesion near
//! the center will appear drastically compressed and distorted if pushed towards the edge,
//! perfectly visualizing hyperbolic boundary density.

use flocking::{compute_force, FlockingParams};
use locus::Vec2 as LocusVec2;
use poincare_disk::{mobius_add, Point};

/// A single Boid agent existing in the hyperbolic plane.
#[derive(Debug, Clone)]
pub struct HyperbolicBoid {
    /// Position within the unit disk (|p| < 1)
    pub position: Point,
    /// Current velocity (Euclidean vector space tangent to the disk at the current position)
    pub velocity: LocusVec2,
}

/// Simulation environment for hyperbolic flocking.
pub struct HyperbolicFlock {
    pub boids: Vec<HyperbolicBoid>,
    pub params: FlockingParams,
}

impl HyperbolicFlock {
    /// Create a new hyperbolic swarm.
    pub fn new(boids: Vec<HyperbolicBoid>, params: FlockingParams) -> Self {
        Self { boids, params }
    }

    /// Step the simulation forward one frame.
    ///
    /// The critical mutation here is how velocity is applied to position.
    /// In Euclidean space: `pos += vel * dt`
    /// In Hyperbolic space: `pos = mobius_add(pos, vel * dt)`
    pub fn update(&mut self, dt: f64) {
        // Extract positions and velocities for the generic compute_force
        let positions: Vec<LocusVec2> = self
            .boids
            .iter()
            .map(|b| LocusVec2::new(b.position.re, b.position.im))
            .collect();
        let velocities: Vec<LocusVec2> = self.boids.iter().map(|b| b.velocity).collect();

        // Note: Ideally, `compute_force` itself would use `hyperbolic_dist`, but as a first cross,
        // we use Euclidean intent driving hyperbolic motion.
        let forces: Vec<LocusVec2> = (0..self.boids.len())
            .map(|i| compute_force(&positions, &velocities, i, &self.params))
            .collect();

        for (i, force) in forces.iter().enumerate() {
            // Apply Euclidean acceleration
            self.boids[i].velocity += *force * dt;
            self.boids[i].velocity = self.boids[i].velocity.limit(self.params.max_speed);

            // Apply Hyperbolic motion (Möbius addition)
            let vel_step = Point::new(self.boids[i].velocity.x * dt, self.boids[i].velocity.y * dt);

            // To prevent escaping the disk due to large steps, we clamp the step
            let max_step_radius = 0.99;
            let step_radius = vel_step.norm();
            let safe_step = if step_radius > max_step_radius {
                Point::new(
                    vel_step.re * max_step_radius / step_radius,
                    vel_step.im * max_step_radius / step_radius,
                )
            } else {
                vel_step
            };

            // Hyperbolic Translation: `pos = safe_step (+) pos`
            self.boids[i].position = mobius_add(safe_step, self.boids[i].position);
        }
    }
}
