use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use proptest::prelude::*;

proptest! {
    #[test]
    #[should_panic(expected = "👺 Havoc: The flocking engine allowed a NaN/Inf infection to spread to an innocent bystander!")]
    fn havoc_compute_force_nan_propagation(
        x in prop_oneof![Just(f64::NAN), Just(f64::INFINITY), Just(f64::NEG_INFINITY)],
    ) {
        let positions = vec![
            Vec2::new(0.0, 0.0),
            Vec2::new(0.0, 0.0), // The innocent bystander
        ];

        let velocities = vec![
            Vec2::new(x, x), // The poisoned velocity
            Vec2::new(0.0, 1.0),
        ];

        let params = FlockingParams {
            view_radius: f64::MAX, // Extremely large view radius to ensure distance check passes
            separation_radius: 10.0,
            max_speed: 2.0,
            max_force: 0.1,
            separation_weight: 1.5,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        };

        // This asserts that the computed force is finite.
        // It will panic (fail the assert), satisfying `#[should_panic]`
        let force = compute_force(&positions, &velocities, 1, &params);
        assert!(force.x.is_finite() && force.y.is_finite(), "👺 Havoc: The flocking engine allowed a NaN/Inf infection to spread to an innocent bystander!");
    }
}
