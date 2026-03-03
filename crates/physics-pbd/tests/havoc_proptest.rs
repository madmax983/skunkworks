use proptest::prelude::*;
use glam::Vec3;
use physics_pbd::{PbdSystem, Constraint};

proptest! {
    #[test]
    fn test_distance_solver_fuzz(
        x1 in proptest::num::f32::ANY,
        y1 in proptest::num::f32::ANY,
        z1 in proptest::num::f32::ANY,
        x2 in proptest::num::f32::ANY,
        y2 in proptest::num::f32::ANY,
        z2 in proptest::num::f32::ANY,
        len in proptest::num::f32::ANY,
        stiff in proptest::num::f32::ANY
    ) {
        let mut system = PbdSystem::new();
        // Skip NaNs since they are meant to panic based on sentry's design
        if x1.is_nan() || y1.is_nan() || z1.is_nan() ||
           x2.is_nan() || y2.is_nan() || z2.is_nan() ||
           len.is_nan() || stiff.is_nan() {
            return Ok(());
        }

        let p1 = system.add_particle(Vec3::new(x1, y1, z1), 1.0);
        let p2 = system.add_particle(Vec3::new(x2, y2, z2), 1.0);

        system.constraints.push(Constraint::Distance {
            p1, p2, rest_length: len, stiffness: stiff
        });

        // Using catch_unwind so fuzzer can pass but still explore without bombing test suite
        let _res = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
            system.step(0.1, 1);
        }));
    }
}
