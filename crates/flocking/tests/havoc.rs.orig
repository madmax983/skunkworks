use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig { failure_persistence: None, cases: 1000, .. ProptestConfig::default() })]

    #[test]
    #[should_panic]
    fn test_havoc_nan_propagation(
        v_x in prop_oneof![
            Just(f64::INFINITY),
            Just(f64::NEG_INFINITY),
            Just(f64::NAN),
            proptest::num::f64::ANY
        ],
        v_y in prop_oneof![
            Just(f64::INFINITY),
            Just(f64::NEG_INFINITY),
            Just(f64::NAN),
            proptest::num::f64::ANY
        ]
    ) {
        let p1 = Vec2::new(0.0, 0.0);
        let v1 = Vec2::new(0.0, 0.0);

        // 👺 Havoc: Keep the positions overlapping to bypass view_radius checks,
        // but inject pure entropy into the neighbor's velocity.
        let p2 = Vec2::new(0.0, 0.0);
        let v2 = Vec2::new(v_x, v_y);

        let positions = vec![p1, p2];
        let velocities = vec![v1, v2];

        let params = FlockingParams {
            view_radius: 100.0,
            separation_radius: 20.0,
            max_speed: 5.0,
            max_force: 1.0,
            separation_weight: 1.0,
            alignment_weight: 1.0,
            cohesion_weight: 1.0,
        };

        let force = compute_force(&positions, &velocities, 0, &params);

        // 🧨 Havoc's Trap: We assert the output remains valid.
        // When proptest injects NaN/Infinity, this will panic and prove the system is fragile!
        assert!(!force.x.is_nan(), "Havoc 👺: The infection spread! Force X is NaN");
        assert!(!force.y.is_nan(), "Havoc 👺: The infection spread! Force Y is NaN");
    }
}
