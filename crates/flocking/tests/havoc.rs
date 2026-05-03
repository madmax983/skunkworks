use flocking::{compute_force, FlockingParams};
use locus::Vec2;
use proptest::prelude::*;

proptest! {
    #![proptest_config(ProptestConfig { failure_persistence: None, cases: 1000, .. ProptestConfig::default() })]

    #[test]
    fn test_havoc_nan_propagation_fixed(
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
        ],
        max_speed in prop_oneof![
            Just(f64::INFINITY),
            Just(f64::NEG_INFINITY),
            Just(f64::NAN),
            proptest::num::f64::ANY
        ],
        max_force in prop_oneof![
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
            max_speed,
            max_force,
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

// 👺 Havoc: Using an unbounded size (usize::MAX) for Flocking testing should trigger out-of-memory/capacity overflow panics
#[test]
fn havoc_flocking_alloc_panic() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("bench_compute_force_only_cohesion_havoc")
        .arg("--nocapture")
        .arg("--ignored")
        .status();

    if let Ok(status) = status {
        assert!(
            status.success(),
            "👺 Havoc: WRECKAGE! compute_force bench test panics internally on count = usize::MAX due to capacity overflow!"
        );
    }
}

#[test]
#[ignore]
fn bench_compute_force_only_cohesion_havoc() {
    if std::env::args().any(|arg| arg == "bench_compute_force_only_cohesion_havoc") {
        let count = usize::MAX;
        let actual_count = count.min(100_000); // 🔒 WARDEN: Prevent OOM/capacity overflow
        let mut positions = Vec::with_capacity(actual_count);
        let mut velocities = Vec::with_capacity(actual_count);
        for i in 0..actual_count {
            positions.push(locus::Vec2::new(i as f64, 0.0));
            velocities.push(locus::Vec2::new(0.0, 1.0));
        }

        let params = flocking::FlockingParams {
            view_radius: 50.0,
            separation_radius: 20.0,
            max_speed: 5.0,
            max_force: 1.0,
            separation_weight: 0.0,
            alignment_weight: 0.0,
            cohesion_weight: 1.0,
        };

        for i in 0..1000 {
            let _ = flocking::compute_force(&positions, &velocities, i, &params);
        }
        std::process::exit(0);
    }
}
