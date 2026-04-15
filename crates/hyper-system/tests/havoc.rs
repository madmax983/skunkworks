use hyper_system::physics::*;
use locus::vec4::Vec4;
use proptest::prelude::*;

proptest! {
    #[test]
    #[should_panic]
    fn havoc_test_step_fuzzed(
        dt in proptest::num::f32::ANY,
        iterations in 0..100usize,
        friction in proptest::num::f32::ANY,
    ) {
        let mut system = PbdSystem4D::new();
        let p1 = system.add_particle(Vec4::zero(), 1.0).unwrap();
        let p2 = system.add_particle(Vec4::new(1.0, 0.0, 0.0, 0.0), 1.0).unwrap();
        system.add_distance_constraint(p1, p2, 1.0);

        system.step(dt, iterations, friction);
    }

    #[test]
    #[should_panic]
    fn havoc_test_actuator_nan_stiffness(
        stiff in proptest::num::f32::ANY,
    ) {
        let mut system = PbdSystem4D::new();
        let p1 = system.add_particle(Vec4::zero(), 1.0).unwrap();
        let p2 = system.add_particle(Vec4::new(1.0, 0.0, 0.0, 0.0), 1.0).unwrap();
        system.add_actuator_constraint(p1, p2, 0.5, 1.5, stiff, 0.5);

        system.step(0.1, 1, 0.98);
    }
}
