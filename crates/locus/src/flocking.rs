//! # The Flock's Mind 🕊️
//!
//! Flocking is the art of simulating complex group behavior from simple individual rules.
//! This module implements Craig Reynolds' "Boids" algorithm.
//!
//! ## The Three Laws
//!
//! 1.  **Separation ("Personal Space")**: Steer to avoid crowding local flockmates.
//!     *   *Too close? Back off.*
//! 2.  **Alignment ("Peer Pressure")**: Steer towards the average heading of local flockmates.
//!     *   *Everyone going left? I'll go left too.*
//! 3.  **Cohesion ("Group Hug")**: Steer to move toward the average position of local flockmates.
//!     *   *Don't get left behind.*
//!
//! ## The Minimal Simulation
//!
//! ```rust
//! use locus::flocking::{compute_force, FlockingParams};
//! use locus::Vec2;
//!
//! // 1. Setup the flock
//! let mut positions = vec![Vec2::new(0.0, 0.0), Vec2::new(5.0, 5.0)];
//! let mut velocities = vec![Vec2::new(1.0, 0.0), Vec2::new(0.0, 1.0)];
//!
//! // 2. Define the rules
//! let params = FlockingParams {
//!     view_radius: 50.0,
//!     separation_radius: 10.0,
//!     max_speed: 2.0,
//!     max_force: 0.1,
//!     separation_weight: 1.5, // Strong desire for personal space
//!     alignment_weight: 1.0,  // Moderate desire to align
//!     cohesion_weight: 1.0,   // Moderate desire to stay together
//! };
//!
//! // 3. The Loop (Simulate one frame)
//! // Note: In a real sim, you'd calculate ALL forces before applying them to avoid order bias.
//! let forces: Vec<Vec2> = (0..positions.len())
//!     .map(|i| compute_force(&positions, &velocities, i, &params))
//!     .collect();
//!
//! for (i, force) in forces.iter().enumerate() {
//!     velocities[i] += *force; // Apply steering
//!     velocities[i] = velocities[i].limit(params.max_speed); // Cap speed
//!     positions[i] += velocities[i]; // Move
//! }
//! ```

use crate::vec2::Vec2;

/// Configuration parameters for the flocking simulation.
///
/// These values act as the "DNA" of the flock, determining whether it behaves like
/// a swarm of angry bees, a school of fish, or a herd of sheep.
#[derive(Clone, Copy, Debug)]
#[cfg_attr(feature = "serde", derive(serde::Serialize, serde::Deserialize))]
pub struct FlockingParams {
    /// The radius within which an agent can "see" neighbors.
    ///
    /// *   **Effect**: Controls the scale of the flock. Large values create massive, connected super-flocks.
    ///     Small values create fragmented, local clusters.
    /// *   **Performance**: Smaller is faster (fewer checks per agent).
    pub view_radius: f64,

    /// The radius within which an agent tries to avoid crowding.
    ///
    /// *   **Effect**: Defines the "personal bubble". Agents inside this radius will actively steer away.
    pub separation_radius: f64,

    /// The maximum speed an agent can travel per tick.
    ///
    /// *   **Effect**: Caps the chaos. Without this, agents would accelerate infinitely.
    pub max_speed: f64,

    /// The maximum steering force an agent can apply to change direction.
    ///
    /// *   **Effect**: Agility.
    ///     *   **Low**: Agents turn like battleships (smooth, sweeping arcs).
    ///     *   **High**: Agents turn like flies (twitchy, instant direction changes).
    pub max_force: f64,

    /// The weight multiplier for the Separation force.
    ///
    /// *   **High**: "Gas-like" behavior. Agents spread out to fill space.
    /// *   **Low**: "Liquid-like" behavior. Agents tolerate crowding.
    pub separation_weight: f64,

    /// The weight multiplier for the Alignment force.
    ///
    /// *   **High**: "Rigid" motion. The flock moves as a solid unit.
    /// *   **Low**: "Chaotic" motion. Agents ignore their neighbor's direction.
    pub alignment_weight: f64,

    /// The weight multiplier for the Cohesion force.
    ///
    /// *   **High**: "Solid-like" attraction. The flock collapses into a tight ball.
    /// *   **Low**: The flock is loose and may break apart easily.
    pub cohesion_weight: f64,
}

/// Helper struct to accumulate steering forces.
#[derive(Default)]
struct FlockingAccumulators {
    separation: Vec2,
    alignment: Vec2,
    cohesion: Vec2,
    sep_count: usize,
    ali_count: usize,
    coh_count: usize,
}

struct PrecomputedParams<'a> {
    params: &'a FlockingParams,
    view_sq: f64,
    sep_sq: f64,
    do_sep: bool,
    do_ali: bool,
    do_coh: bool,
}

impl FlockingAccumulators {
    fn accumulate(
        &mut self,
        my_pos: Vec2,
        neighbor_pos: Vec2,
        neighbor_vel: Vec2,
        pre: &PrecomputedParams,
        overlap_bias: Vec2,
    ) {
        let dx = my_pos.x - neighbor_pos.x;
        if dx.abs() > pre.params.view_radius {
            return;
        }

        let dy = my_pos.y - neighbor_pos.y;
        if dy.abs() > pre.params.view_radius {
            return;
        }

        let mut d_sq = dx * dx + dy * dy;

        if d_sq >= pre.view_sq {
            return;
        }

        let diff = if d_sq <= f64::EPSILON {
            // Handle overlap: push away based on bias.
            d_sq = 0.01; // Avoid division by zero
            overlap_bias
        } else {
            Vec2::new(dx, dy)
        };

        if pre.do_sep && d_sq < pre.sep_sq {
            self.separation += diff * (1.0 / d_sq);
            self.sep_count += 1;
        }

        if pre.do_ali {
            self.alignment += neighbor_vel;
            self.ali_count += 1;
        }

        if pre.do_coh {
            self.cohesion += neighbor_pos;
            self.coh_count += 1;
        }
    }
}

/// Calculates a steering force towards a target velocity or position derivative.
///
/// This helper encapsulates the common pattern:
/// 1. Normalize the desired vector.
/// 2. Scale to max speed.
/// 3. Subtract current velocity (to get steering force).
/// 4. Limit the steering force.
fn compute_steering(mut desired: Vec2, current_vel: Vec2, max_speed: f64, max_force: f64) -> Vec2 {
    let d_sq = desired.magnitude_squared();
    if d_sq > 0.0 {
        // Optimization: Normalize and scale in one go: desired * (max_speed / mag)
        // 1 div, 1 sqrt, 2 muls (vs 2 divs, 1 sqrt, 2 muls in standard normalize)
        let mag = d_sq.sqrt();
        desired *= max_speed / mag;

        desired -= current_vel;

        // Optimization: Limit logic inlined to avoid redundant sqrt/divs
        let s_sq = desired.magnitude_squared();
        if s_sq > max_force * max_force {
            let s_mag = s_sq.sqrt();
            desired *= max_force / s_mag;
        }
        desired
    } else {
        Vec2::zero()
    }
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
///
/// # Examples
///
/// ```rust
/// use locus::flocking::{compute_force, FlockingParams};
/// use locus::Vec2;
///
/// let positions = vec![Vec2::new(0.0, 0.0), Vec2::new(1.0, 0.0)];
/// let velocities = vec![Vec2::zero(), Vec2::zero()];
/// let params = FlockingParams {
///     view_radius: 10.0,
///     separation_radius: 5.0,
///     max_speed: 1.0,
///     max_force: 0.1,
///     separation_weight: 1.0,
///     alignment_weight: 0.0,
///     cohesion_weight: 0.0,
/// };
///
/// // Calculate force for the first agent
/// let force = compute_force(&positions, &velocities, 0, &params);
/// assert!(force.x < 0.0); // Should be pushed away from the neighbor at (1,0)
/// ```
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

    let mut acc = FlockingAccumulators::default();

    // Precompute invariants for the hot loop
    let pre = PrecomputedParams {
        params,
        view_sq: params.view_radius * params.view_radius,
        sep_sq: params.separation_radius * params.separation_radius,
        do_sep: params.separation_weight.abs() > 0.0,
        do_ali: params.alignment_weight.abs() > 0.0,
        do_coh: params.cohesion_weight.abs() > 0.0,
    };

    // Loop Splitting: We split the slices at `my_idx` to iterate over left and right neighbors separately.
    // This avoids checking `i == my_idx` inside the hot loop.
    let (left_pos, right_pos) = positions.split_at(my_idx);
    let (left_vel, right_vel) = velocities.split_at(my_idx);

    // Process left neighbors (0..my_idx)
    // Left neighbors have index < my_idx, so we push Right (+x).
    let left_bias = Vec2::new(1.0, 0.0);
    for (pos, vel) in left_pos.iter().zip(left_vel) {
        acc.accumulate(my_pos, *pos, *vel, &pre, left_bias);
    }

    // Process right neighbors (my_idx+1..len)
    // right_pos[0] is self, so skip it.
    // Right neighbors have index > my_idx, so we push Left (-x).
    let right_bias = Vec2::new(-1.0, 0.0);
    if right_pos.len() > 1 {
        for (pos, vel) in right_pos[1..].iter().zip(&right_vel[1..]) {
            acc.accumulate(my_pos, *pos, *vel, &pre, right_bias);
        }
    }

    let mut total = Vec2::zero();

    if acc.sep_count > 0 {
        total += compute_steering(acc.separation, my_vel, params.max_speed, params.max_force)
            * params.separation_weight;
    }

    if acc.ali_count > 0 {
        acc.alignment /= acc.ali_count as f64;
        total += compute_steering(acc.alignment, my_vel, params.max_speed, params.max_force)
            * params.alignment_weight;
    }

    if acc.coh_count > 0 {
        acc.cohesion /= acc.coh_count as f64;
        let desired = acc.cohesion - my_pos;
        total += compute_steering(desired, my_vel, params.max_speed, params.max_force)
            * params.cohesion_weight;
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

    #[test]
    fn bench_compute_force() {
        let count = 1000;
        let mut positions = Vec::with_capacity(count);
        let mut velocities = Vec::with_capacity(count);
        for i in 0..count {
            positions.push(Vec2::new(i as f64, 0.0));
            velocities.push(Vec2::new(0.0, 1.0));
        }

        let params = FlockingParams {
            view_radius: 50.0, // Only nearby neighbors
            separation_radius: 20.0,
            max_speed: 5.0,
            max_force: 1.0,
            separation_weight: 1.0,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        };

        let start = std::time::Instant::now();
        // Compute force for all agents (N^2 roughly)
        // But here we just call it for the first 1000 to save time in test
        for i in 0..1000 {
            // Use black_box equivalent?
            let _ = compute_force(&positions, &velocities, i, &params);
        }
        println!("Time taken: {:?}", start.elapsed());
    }

    #[test]
    fn bench_compute_force_only_cohesion() {
        let count = 1000;
        let mut positions = Vec::with_capacity(count);
        let mut velocities = Vec::with_capacity(count);
        for i in 0..count {
            positions.push(Vec2::new(i as f64, 0.0));
            velocities.push(Vec2::new(0.0, 1.0));
        }

        let params = FlockingParams {
            view_radius: 50.0,
            separation_radius: 20.0,
            max_speed: 5.0,
            max_force: 1.0,
            separation_weight: 0.0, // Disabled
            alignment_weight: 0.0,  // Disabled
            cohesion_weight: 1.0,
        };

        let start = std::time::Instant::now();
        for i in 0..1000 {
            let _ = compute_force(&positions, &velocities, i, &params);
        }
        println!("Time taken (only cohesion): {:?}", start.elapsed());
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

    #[test]
    fn test_overlapping_agents_should_separate() {
        let p1 = Vec2::new(0.0, 0.0);
        let v1 = Vec2::zero();
        let p2 = Vec2::new(0.0, 0.0);
        let v2 = Vec2::zero();

        let positions = vec![p1, p2];
        let velocities = vec![v1, v2];

        let params = FlockingParams {
            separation_weight: 1.0,
            ..default_params()
        };

        // Compute force for agent 0
        let force0 = compute_force(&positions, &velocities, 0, &params);

        // Compute force for agent 1
        let force1 = compute_force(&positions, &velocities, 1, &params);

        // They should push apart. Ideally in opposite directions.
        // At least one should be non-zero.
        assert!(
            force0.magnitude_squared() > 0.0 || force1.magnitude_squared() > 0.0,
            "Agents at exactly same position should separate, but got force0={:?}, force1={:?}",
            force0,
            force1
        );
    }

    #[test]
    #[should_panic]
    fn test_panic_on_mismatched_lengths() {
        let positions = vec![Vec2::zero()];
        let velocities = vec![];
        let params = default_params();
        let _ = compute_force(&positions, &velocities, 0, &params);
    }

    #[test]
    #[should_panic(expected = "assertion `left == right` failed")]
    fn test_compute_force_mismatched_lengths_panic() {
        let params = FlockingParams {
            view_radius: 10.0,
            separation_radius: 5.0,
            max_speed: 1.0,
            max_force: 0.1,
            separation_weight: 1.0,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        };
        let positions = vec![Vec2::zero(), Vec2::new(1.0, 1.0)];
        let velocities = vec![Vec2::zero()];

        let _ = compute_force(&positions, &velocities, 0, &params);
    }
}
