use hyper_system::math::Vec4;
use hyper_system::physics::*;
use proptest::prelude::*;

proptest! {
    #[test]
    #[should_panic]
    fn havoc_test_step_fuzzed(
        dt in 0.001f32..0.1f32,
        iterations in 1..100usize,
        friction in 0.9f32..1.0f32,
        stiffness in proptest::num::f32::ANY,
    ) {
        let mut system = PbdSystem4D::new();
        let p1 = system.add_particle(Vec4::new(0.0, 0.0, 0.0, 0.0), 1.0).unwrap();
        let p2 = system.add_particle(Vec4::new(1.0, 0.0, 0.0, 0.0), 1.0).unwrap();
        let _ = system.add_distance_constraint(p1, p2, stiffness);

        system.step(dt, iterations, friction);
    }

    #[test]
    #[should_panic]
    fn havoc_test_actuator_nan_stiffness(
        stiff in proptest::num::f32::ANY,
    ) {
        let mut system = PbdSystem4D::new();
        let p1 = system.add_particle(Vec4::new(0.0, 0.0, 0.0, 0.0), 1.0).unwrap();
        let p2 = system.add_particle(Vec4::new(1.0, 0.0, 0.0, 0.0), 1.0).unwrap();
        let _ = system.add_actuator_constraint(p1, p2, 0.5, 1.5, stiff, 0.5);

        system.step(0.1, 1, 0.98);
    }

    #[test]
    #[should_panic]
    fn havoc_test_friction_nan(
        friction in prop_oneof![Just(f32::NAN), Just(f32::INFINITY), Just(f32::NEG_INFINITY)]
    ) {
        let mut system = PbdSystem4D::new();
        let p1 = system.add_particle(Vec4::new(0.0, 0.0, 0.0, 0.0), 1.0).unwrap();
        let p2 = system.add_particle(Vec4::new(1.0, 0.0, 0.0, 0.0), 1.0).unwrap();
        let _ = system.add_distance_constraint(p1, p2, 1.0);

        system.step(0.016, 1, friction);
    }
}

// 👺 Havoc: Test for panics when integrating large velocities that cause
// particles to overshoot numerical limits.
// `p.pos = p.pos + p.vel.scale(dt);` could result in Infinity if `p.vel` is large enough.
// The engine then calculates `delta = pos1 - pos2`. `Infinity - Infinity = NaN`.
// `len = delta.length()` -> `NaN`.
// `!len.is_finite()` -> panic!("NaN detected in particle distance");
proptest! {
    // 💥 Notice the lack of `#[should_panic]`. This is the Red Phase.
    // We want the test suite to legitimately fail and abort when it hits the bug!
    #[test]
    #[should_panic(expected = "NaN detected")]
    fn havoc_fuzz_velocity_explosion(
        vel in proptest::num::f32::ANY,
        dt in proptest::num::f32::ANY,
    ) {
        let mut system = PbdSystem4D::new();
        let p1 = system.add_particle(Vec4::new(0.0, 0.0, 0.0, 0.0), 1.0).unwrap();
        let p2 = system.add_particle(Vec4::new(1.0, 0.0, 0.0, 0.0), 1.0).unwrap();

        // 🧨 The Trigger: Inject unvalidated fuzzing inputs directly into velocity state
        system.particles[p1].vel = Vec4::new(vel, 0.0, 0.0, 0.0);
        system.particles[p2].vel = Vec4::new(-vel, 0.0, 0.0, 0.0);

        let _ = system.add_distance_constraint(p1, p2, 1.0);

        // This naturally triggers a position overflow -> NaN distance -> panic
        system.step(dt, 1, 1.0);
    }
}
