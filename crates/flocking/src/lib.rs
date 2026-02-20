//! # Flocking 🕊️
//!
//! A lightweight implementation of Reynolds' Flocking algorithm (Boids).
//!
//! This module provides the `compute_force` function to calculate steering vectors
//! based on Separation, Alignment, and Cohesion rules.
//!
//! Unlike previous versions, this crate is stateless and data-oriented.
//! It operates on raw slices of positions and velocities, allowing the consumer
//! to manage their own physics state (e.g., using specific integration methods or
//! data layouts).

use locus::Vec2;

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
/// * `positions` - A slice of all agent positions.
/// * `velocities` - A slice of all agent velocities.
/// * `my_idx` - The index of the current agent in the slices.
/// * `params` - The flocking configuration parameters.
///
/// # Returns
///
/// A `Vec2` representing the total steering force to be applied to the agent.
///
/// # Panics
///
/// Panics if `positions` and `velocities` have different lengths.
#[must_use]
pub fn compute_force(
    positions: &[Vec2],
    velocities: &[Vec2],
    my_idx: usize,
    params: &FlockingParams,
) -> Vec2 {
    assert_eq!(positions.len(), velocities.len());
    if my_idx >= positions.len() {
        return Vec2::zero();
    }

    let my_pos = positions[my_idx];
    let my_vel = velocities[my_idx];

    let mut separation = Vec2::zero();
    let mut alignment = Vec2::zero();
    let mut cohesion = Vec2::zero();

    let mut sep_count = 0;
    let mut ali_count = 0;
    let mut coh_count = 0;

    let view_sq = params.view_radius * params.view_radius;
    let sep_sq = params.separation_radius * params.separation_radius;

    // Process neighbors
    // We split to avoid comparing with self (optimization)
    let (pos_before, pos_after) = positions.split_at(my_idx);
    let (vel_before, vel_after) = velocities.split_at(my_idx);

    let mut process = |pos: Vec2, vel: Vec2| {
        let diff = my_pos - pos;
        let d_sq = diff.magnitude_squared();

        if d_sq > 0.0 && d_sq < view_sq {
            // Separation
            if d_sq < sep_sq {
                separation += diff * (1.0 / d_sq);
                sep_count += 1;
            }

            // Alignment
            alignment += vel;
            ali_count += 1;

            // Cohesion
            cohesion += pos;
            coh_count += 1;
        }
    };

    for (&pos, &vel) in pos_before.iter().zip(vel_before.iter()) {
        process(pos, vel);
    }
    // Skip self (index 0 of 'after')
    if !pos_after.is_empty() {
        for (&pos, &vel) in pos_after[1..].iter().zip(vel_after[1..].iter()) {
            process(pos, vel);
        }
    }

    let mut total = Vec2::zero();

    if sep_count > 0 && separation.magnitude_squared() > 0.0 {
        separation = separation.normalize() * params.max_speed;
        separation -= my_vel;
        separation = separation.limit(params.max_force);
        total += separation * params.separation_weight;
    }

    if ali_count > 0 {
        alignment /= ali_count as f64;
        if alignment.magnitude_squared() > 0.0 {
            alignment = alignment.normalize() * params.max_speed;
            alignment -= my_vel;
            alignment = alignment.limit(params.max_force);
            total += alignment * params.alignment_weight;
        }
    }

    if coh_count > 0 {
        cohesion /= coh_count as f64;
        let mut desired = cohesion - my_pos;
        if desired.magnitude_squared() > 0.0 {
            desired = desired.normalize() * params.max_speed;
            desired -= my_vel;
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
        let force = compute_force(&[], &[], 0, &params);
        assert_eq!(force, Vec2::zero());
    }

    #[test]
    fn test_flocking_force_zero() {
        let p1 = Vec2::zero();
        let v1 = Vec2::zero();
        let params = FlockingParams {
            view_radius: 10.0,
            separation_radius: 5.0,
            max_speed: 1.0,
            max_force: 0.1,
            separation_weight: 1.0,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        };
        let force = compute_force(&[p1], &[v1], 0, &params);
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
    fn test_separation_force() {
        let p1 = Vec2::new(0.0, 0.0);
        let v1 = Vec2::zero();
        // Neighbor to the right, inside separation radius (20.0)
        let p2 = Vec2::new(10.0, 0.0);
        let v2 = Vec2::zero();

        let positions = vec![p1, p2];
        let velocities = vec![v1, v2];

        let params = FlockingParams {
            separation_weight: 1.0,
            ..default_params()
        };

        let force = compute_force(&positions, &velocities, 0, &params);

        // Force should be pushing LEFT (-x)
        assert!(force.x < 0.0);
        assert_eq!(force.y, 0.0);
    }

    #[test]
    fn test_cohesion_force() {
        let p1 = Vec2::new(0.0, 0.0);
        let v1 = Vec2::zero();
        // Neighbor to the right, outside separation (20.0) but inside view (100.0)
        let p2 = Vec2::new(50.0, 0.0);
        let v2 = Vec2::zero();

        let positions = vec![p1, p2];
        let velocities = vec![v1, v2];

        let params = FlockingParams {
            cohesion_weight: 1.0,
            ..default_params()
        };

        let force = compute_force(&positions, &velocities, 0, &params);

        // Force should be pulling RIGHT (+x) towards neighbor
        assert!(force.x > 0.0);
        assert_eq!(force.y, 0.0);
    }

    #[test]
    fn test_alignment_force() {
        let p1 = Vec2::new(0.0, 0.0);
        let v1 = Vec2::zero();
        let p2 = Vec2::new(10.0, 0.0);
        // Neighbor moving UP (0, 1)
        let v2 = Vec2::new(0.0, 1.0);

        let positions = vec![p1, p2];
        let velocities = vec![v1, v2];

        let params = FlockingParams {
            alignment_weight: 1.0,
            ..default_params()
        };

        let force = compute_force(&positions, &velocities, 0, &params);

        // Force should be steering UP (+y) to match velocity
        assert_eq!(force.x, 0.0);
        assert!(force.y > 0.0);
    }
}
