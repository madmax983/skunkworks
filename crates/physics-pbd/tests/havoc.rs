use glam::Vec3;
use physics_pbd::*;
use proptest::prelude::*;

proptest! {
    #[test]
    #[should_panic]
    fn havoc_test_step_fuzzed(
        dt in 0.001f32..0.1f32,
        iterations in 1..100usize,
        stiffness in proptest::num::f32::ANY,
    ) {
        let mut system = PbdSystem::new();
        let p1 = system.add_particle(Vec3::ZERO, 1.0);
        let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);

        system.add_distance_constraint(p1, p2, stiffness);

        system.step(dt, iterations);
    }
}
