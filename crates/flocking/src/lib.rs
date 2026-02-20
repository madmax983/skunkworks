//! # Flocking 🕊️
//!
//! A lightweight implementation of Reynolds' Flocking algorithm (Boids).
//!
//! This module provides the `PhysicsState` for agents and the `compute_force` function
//! to calculate steering vectors based on Separation, Alignment, and Cohesion rules.

use locus::Vec2;

/// Represents the physical properties of an autonomous agent.
///
/// This struct holds the kinematic state (position, velocity, acceleration) required
/// for the physics update loop.
///
/// # Examples
///
/// ```
/// use flocking::PhysicsState;
/// use locus::Vec2;
///
/// let mut agent = PhysicsState::new(10.0, 20.0);
/// agent.apply_force(Vec2::new(1.0, 0.0));
/// agent.update(5.0); // Update with max speed of 5.0
/// ```
#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PhysicsState {
    /// The current position in 2D space.
    pub position: Vec2,
    /// The current velocity vector.
    pub velocity: Vec2,
    /// The current acceleration vector (reset after each update).
    pub acceleration: Vec2,
}

impl PhysicsState {
    /// Creates a new agent at the specified coordinates.
    ///
    /// The initial velocity and acceleration are set to zero.
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::zero(),
            acceleration: Vec2::zero(),
        }
    }

    /// Applies a force to the agent, accumulating acceleration.
    ///
    /// Following Newton's second law (F=ma), assuming unit mass (m=1),
    /// force is directly added to acceleration.
    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    /// Updates the agent's position and velocity based on accumulated acceleration.
    ///
    /// This method performs the following steps:
    /// 1. Adds acceleration to velocity.
    /// 2. Limits velocity to `max_speed`.
    /// 3. Adds velocity to position.
    /// 4. Resets acceleration to zero (ready for the next frame).
    ///
    /// # Arguments
    ///
    /// * `max_speed` - The maximum magnitude of the velocity vector.
    pub fn update(&mut self, max_speed: f64) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.limit(max_speed);
        self.position += self.velocity;
        self.acceleration = Vec2::zero();
    }
}

/// Configuration parameters for the flocking simulation.
///
/// These values control the behavior and emergence of the flock.
#[derive(Clone, Copy, Debug)]
pub struct FlockingParams {
    /// The radius within which an agent can "see" neighbors.
    /// Only neighbors within this distance influence Cohesion and Alignment.
    pub view_radius: f64,
    /// The radius within which an agent tries to avoid crowding.
    /// Only neighbors within this distance influence Separation.
    pub separation_radius: f64,
    /// The maximum speed an agent can travel per tick.
    pub max_speed: f64,
    /// The maximum steering force an agent can apply to change direction.
    /// This limits how sharply an agent can turn.
    pub max_force: f64,
    /// The weight multiplier for the Separation force.
    /// Higher values make agents spread out more aggressively.
    pub separation_weight: f64,
    /// The weight multiplier for the Alignment force.
    /// Higher values make agents move in the same direction as neighbors.
    pub alignment_weight: f64,
    /// The weight multiplier for the Cohesion force.
    /// Higher values make agents clump together more tightly.
    pub cohesion_weight: f64,
}

/// Computes the Reynolds flocking force (Separation, Alignment, Cohesion).
///
/// This function calculates the steering force required to satisfy the three rules of flocking:
/// 1. **Separation**: Steer to avoid crowding local flockmates.
/// 2. **Alignment**: Steer towards the average heading of local flockmates.
/// 3. **Cohesion**: Steer to move toward the average position of local flockmates.
///
/// # Arguments
///
/// * `others` - A slice of all agents (including the current one).
/// * `my_idx` - The index of the current agent in the `others` slice.
/// * `params` - The flocking configuration parameters.
///
/// # Returns
///
/// A `Vec2` representing the total steering force to be applied to the agent.
///
/// # Performance
///
/// This function iterates over the entire `others` slice to find neighbors.
/// For large flocks, consider using a spatial partition structure to provide a
/// pre-filtered list of potential neighbors, or accept the O(N) cost per agent (O(N^2) total).
#[must_use]
pub fn compute_force(others: &[PhysicsState], my_idx: usize, params: &FlockingParams) -> Vec2 {
    if my_idx >= others.len() {
        return Vec2::zero();
    }

    let me = &others[my_idx];
    let mut separation = Vec2::zero();
    let mut alignment = Vec2::zero();
    let mut cohesion = Vec2::zero();

    let mut sep_count = 0;
    let mut ali_count = 0;
    let mut coh_count = 0;

    let view_sq = params.view_radius * params.view_radius;
    let sep_sq = params.separation_radius * params.separation_radius;

    // Helper closure for processing neighbors
    // Note: We use a closure here to capture the accumulators.
    // We rely on the compiler to inline this for performance.
    // If performance regresses, check if inlining failed.
    let mut process_neighbor = |other: &PhysicsState| {
        let diff = me.position - other.position;
        let d_sq = diff.magnitude_squared();

        if d_sq > 0.0 && d_sq < view_sq {
            // Separation
            if d_sq < sep_sq {
                separation += diff * (1.0 / d_sq);
                sep_count += 1;
            }

            // Alignment
            alignment += other.velocity;
            ali_count += 1;

            // Cohesion
            cohesion += other.position;
            coh_count += 1;
        }
    };

    let (before, after) = others.split_at(my_idx);
    for other in before {
        process_neighbor(other);
    }
    for other in &after[1..] {
        process_neighbor(other);
    }

    let mut total = Vec2::zero();

    if sep_count > 0 && separation.magnitude_squared() > 0.0 {
        separation = separation.normalize() * params.max_speed;
        separation -= me.velocity;
        separation = separation.limit(params.max_force);
        total += separation * params.separation_weight;
    }

    if ali_count > 0 {
        alignment /= ali_count as f64;
        if alignment.magnitude_squared() > 0.0 {
            alignment = alignment.normalize() * params.max_speed;
            alignment -= me.velocity;
            alignment = alignment.limit(params.max_force);
            total += alignment * params.alignment_weight;
        }
    }

    if coh_count > 0 {
        cohesion /= coh_count as f64;
        let mut desired = cohesion - me.position;
        if desired.magnitude_squared() > 0.0 {
            desired = desired.normalize() * params.max_speed;
            desired -= me.velocity;
            desired = desired.limit(params.max_force);
            total += desired * params.cohesion_weight;
        }
    }

    total
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_compute_force_safe_on_invalid_index() {
        let params = FlockingParams {
            view_radius: 10.0,
            separation_radius: 5.0,
            max_speed: 1.0,
            max_force: 0.1,
            separation_weight: 1.0,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        };
        // This should not panic anymore
        let force = compute_force(&[], 0, &params);
        assert_eq!(force, Vec2::zero());
    }

    #[test]
    fn test_physics_update() {
        let mut p = PhysicsState::new(0.0, 0.0);
        p.apply_force(Vec2::new(1.0, 0.0));
        p.update(10.0);
        assert_eq!(p.velocity, Vec2::new(1.0, 0.0));
        assert_eq!(p.position, Vec2::new(1.0, 0.0));
    }

    #[test]
    fn test_flocking_force_zero() {
        let p1 = PhysicsState::new(0.0, 0.0);
        let params = FlockingParams {
            view_radius: 10.0,
            separation_radius: 5.0,
            max_speed: 1.0,
            max_force: 0.1,
            separation_weight: 1.0,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        };
        let force = compute_force(&[p1], 0, &params);
        assert_eq!(force, Vec2::zero());
    }
}

#[cfg(test)]
mod extended_tests {
    use super::*;

    // Default params for testing isolation
    fn default_params() -> FlockingParams {
        FlockingParams {
            view_radius: 100.0,
            separation_radius: 20.0,
            max_speed: 5.0,
            max_force: 1.0,
            separation_weight: 0.0,
            alignment_weight: 0.0,
            cohesion_weight: 0.0,
        }
    }

    #[test]
    fn test_singularity_behavior() {
        // Two agents at exact same position (0,0)
        // Current logic: d_sq == 0, so loop condition `d_sq > 0.0` fails.
        // Result: No force computed from neighbor.
        let p1 = PhysicsState::new(0.0, 0.0);
        let p2 = PhysicsState::new(0.0, 0.0);

        let params = FlockingParams {
            separation_weight: 1.0,
            ..default_params()
        };

        let force = compute_force(&[p1, p2], 0, &params);

        // This confirms current behavior prevents panic but also prevents separation
        assert_eq!(force, Vec2::zero());
    }

    #[test]
    fn test_separation_force() {
        let p1 = PhysicsState::new(0.0, 0.0);
        // Neighbor to the right, inside separation radius (20.0)
        let p2 = PhysicsState::new(10.0, 0.0);

        let params = FlockingParams {
            separation_weight: 1.0,
            ..default_params()
        };

        let force = compute_force(&[p1, p2], 0, &params);

        // Force should be pushing LEFT (-x)
        assert!(force.x < 0.0);
        assert_eq!(force.y, 0.0);
    }

    #[test]
    fn test_cohesion_force() {
        let p1 = PhysicsState::new(0.0, 0.0);
        // Neighbor to the right, outside separation (20.0) but inside view (100.0)
        let p2 = PhysicsState::new(50.0, 0.0);

        let params = FlockingParams {
            cohesion_weight: 1.0,
            ..default_params()
        };

        let force = compute_force(&[p1, p2], 0, &params);

        // Force should be pulling RIGHT (+x) towards neighbor
        assert!(force.x > 0.0);
        assert_eq!(force.y, 0.0);
    }

    #[test]
    fn test_alignment_force() {
        let p1 = PhysicsState::new(0.0, 0.0);
        let mut p2 = PhysicsState::new(10.0, 0.0);
        // Neighbor moving UP (0, 1)
        p2.velocity = Vec2::new(0.0, 1.0);

        let params = FlockingParams {
            alignment_weight: 1.0,
            ..default_params()
        };

        let force = compute_force(&[p1, p2], 0, &params);

        // Force should be steering UP (+y) to match velocity
        assert_eq!(force.x, 0.0);
        assert!(force.y > 0.0);
    }

    #[test]
    fn test_view_radius_cutoff() {
        let p1 = PhysicsState::new(0.0, 0.0);
        // Neighbor just outside view radius (100.0)
        let p2 = PhysicsState::new(100.1, 0.0);

        let mut params = default_params();
        params.cohesion_weight = 1.0;
        params.separation_weight = 1.0;
        params.alignment_weight = 1.0;

        let force = compute_force(&[p1, p2], 0, &params);

        // Should be ignored
        assert_eq!(force, Vec2::zero());
    }

    #[test]
    fn test_separation_radius_cutoff() {
        let p1 = PhysicsState::new(0.0, 0.0);
        // Neighbor inside view (100) but outside separation (20)
        let p2 = PhysicsState::new(21.0, 0.0);

        let mut params = default_params();
        params.separation_weight = 1.0;
        // Ensure other weights are 0 to isolate separation check

        let force = compute_force(&[p1, p2], 0, &params);

        // Should calculate NO separation force
        assert_eq!(force, Vec2::zero());
    }

    #[test]
    fn test_lone_wolf() {
        let p1 = PhysicsState::new(0.0, 0.0);
        let params = default_params();
        let force = compute_force(&[p1], 0, &params);
        assert_eq!(force, Vec2::zero());
    }

    #[test]
    fn test_force_accumulation_limits() {
        let p1 = PhysicsState::new(0.0, 0.0);
        let p2 = PhysicsState::new(5.0, 0.0); // Close neighbor

        let mut params = default_params();
        params.max_force = 0.5;
        params.separation_weight = 2.0;

        // Separation logic:
        // 1. diff = (-5, 0)
        // 2. separation += (-5, 0) / 25 = (-0.2, 0)
        // 3. separation normalize -> (-1, 0) * max_speed(5) -> (-5, 0)
        // 4. - velocity(0) -> (-5, 0)
        // 5. limit(max_force=0.5) -> (-0.5, 0)
        // 6. * weight(2.0) -> (-1.0, 0)

        let force = compute_force(&[p1, p2], 0, &params);

        // Expect (-1.0, 0)
        assert!((force.x - -1.0).abs() < 1e-6);
        assert_eq!(force.y, 0.0);
    }
}
