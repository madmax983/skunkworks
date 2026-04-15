use physics_pbd::*;
use glam::Vec3;
use proptest::prelude::*;

proptest! {
    #[test]
    #[should_panic]
    fn havoc_test_step_fuzzed(
        dt in proptest::num::f32::ANY,
        iterations in 0..100usize,
    ) {
        let mut system = PbdSystem::new();
        let p1 = system.add_particle(Vec3::ZERO, 1.0);
        let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);
        system.add_distance_constraint(p1, p2, 1.0);

        system.step(dt, iterations);
    }
}
