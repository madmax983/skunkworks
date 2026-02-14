use locus::Vec2;

#[derive(Clone, Copy, Debug, Default, PartialEq)]
pub struct PhysicsState {
    pub position: Vec2,
    pub velocity: Vec2,
    pub acceleration: Vec2,
}

impl PhysicsState {
    pub fn new(x: f64, y: f64) -> Self {
        Self {
            position: Vec2::new(x, y),
            velocity: Vec2::zero(),
            acceleration: Vec2::zero(),
        }
    }

    pub fn apply_force(&mut self, force: Vec2) {
        self.acceleration += force;
    }

    pub fn update(&mut self, max_speed: f64) {
        self.velocity += self.acceleration;
        self.velocity = self.velocity.limit(max_speed);
        self.position += self.velocity;
        self.acceleration = Vec2::zero();
    }
}

#[derive(Clone, Copy, Debug)]
pub struct FlockingParams {
    pub view_radius: f64,
    pub separation_radius: f64,
    pub max_speed: f64,
    pub max_force: f64,
    pub separation_weight: f64,
    pub alignment_weight: f64,
    pub cohesion_weight: f64,
}

/// Computes the Reynolds flocking force (Separation, Alignment, Cohesion).
///
/// # Arguments
///
/// * `others` - A slice of all agents (including self).
/// * `my_idx` - The index of the current agent in the `others` slice.
/// * `params` - The flocking parameters.
pub fn compute_force(others: &[PhysicsState], my_idx: usize, params: &FlockingParams) -> Vec2 {
    let me = &others[my_idx];
    let mut separation = Vec2::zero();
    let mut alignment = Vec2::zero();
    let mut cohesion = Vec2::zero();

    let mut sep_count = 0;
    let mut ali_count = 0;
    let mut coh_count = 0;

    let view_sq = params.view_radius * params.view_radius;
    let sep_sq = params.separation_radius * params.separation_radius;

    for (i, other) in others.iter().enumerate() {
        if i == my_idx {
            continue;
        }

        let d_sq = me.position.distance_squared(other.position);

        if d_sq > 0.0 && d_sq < view_sq {
            // Separation
            if d_sq < sep_sq {
                let diff = me.position - other.position;
                separation += diff / d_sq;
                sep_count += 1;
            }

            // Alignment
            alignment += other.velocity;
            ali_count += 1;

            // Cohesion
            cohesion += other.position;
            coh_count += 1;
        }
    }

    let mut total = Vec2::zero();

    if sep_count > 0 {
        if separation.magnitude_squared() > 0.0 {
            separation = separation.normalize() * params.max_speed;
            separation -= me.velocity;
            separation = separation.limit(params.max_force);
            total += separation * params.separation_weight;
        }
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
