use locus::*;
use locus::flocking::FlockingParams;
use proptest::prelude::*;

proptest! {
    #[test]
    #[should_panic]
    fn havoc_compute_force_length_mismatch(len1 in 0usize..10, len2 in 0usize..10) {
        // 🧨 The Trigger:
        // `compute_force` asserts that `positions` and `velocities` have the same length
        // but does not gracefully return an error if they mismatch.
        // It panics!
        prop_assume!(len1 != len2);
        let pos = vec![Vec2::zero(); len1];
        let vel = vec![Vec2::zero(); len2];
        let params = FlockingParams {
            view_radius: 50.0,
            separation_radius: 10.0,
            cohesion_weight: 1.0,
            alignment_weight: 1.0,
            separation_weight: 1.5,
            max_speed: 3.0,
            max_force: 0.1,
        };
        let _ = flocking::compute_force(&pos, &vel, 0, &params);
    }
}
