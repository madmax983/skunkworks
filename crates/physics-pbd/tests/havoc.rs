use glam::Vec3;
use physics_pbd::{Constraint, PbdSystem};
use proptest::prelude::*;

// Havoc: I'm throwing unconstrained NORMAL floats at your delicate Distance constraint.
// You assumed `len.is_finite()` would protect you, but what happens when `pos1 - pos2`
// overflows during vector length calculation because the distance is larger than f32::MAX?
// The engine panics with 'NaN detected in particle distance' or 'NaN detected in constraint parameters'.
proptest! {
    #![proptest_config(ProptestConfig { failure_persistence: None, cases: 10000, .. ProptestConfig::default() })]
    #[test]
    fn test_distance_constraint_fuzzing(
        p1_pos_x in proptest::num::f32::NORMAL,
        p1_pos_y in proptest::num::f32::NORMAL,
        p1_pos_z in proptest::num::f32::NORMAL,
        p1_mass in 0.0f32..f32::MAX,
        p2_pos_x in proptest::num::f32::NORMAL,
        p2_pos_y in proptest::num::f32::NORMAL,
        p2_pos_z in proptest::num::f32::NORMAL,
        p2_mass in 0.0f32..f32::MAX,
        target_len in proptest::num::f32::NORMAL,
        stiffness in proptest::num::f32::NORMAL,
        dt in proptest::num::f32::NORMAL,
        iterations in 1usize..100usize,
    ) {
        let mut system = PbdSystem::new();

        let p1 = system.add_particle(Vec3::new(p1_pos_x, p1_pos_y, p1_pos_z), p1_mass);
        let p2 = system.add_particle(Vec3::new(p2_pos_x, p2_pos_y, p2_pos_z), p2_mass);

        let _ = system.add_distance_constraint(p1, p2, target_len);

        if let Some(Constraint::Distance { stiffness: s, .. }) = system.constraints.first_mut() {
            *s = stiffness;
        }

        system.step(dt, iterations);
    }
}

// 👺 HAVOC: Proving that `add_distance_constraint` panics if given invalid indices.
// We use a custom runner thread to isolate the overflow/panic so the whole test suite doesn't abort.
#[test]
fn havoc_physics_oob_panic() {
    let status = std::process::Command::new(std::env::current_exe().unwrap())
        .arg("--exact")
        .arg("havoc_physics_oob_panic_inner")
        .arg("--nocapture")
        .arg("--ignored")
        .status();

    if let Ok(status) = status {
        assert!(
            status.success(),
            "👺 Havoc: WRECKAGE! System failed to prevent Out of Bounds indexing in add_distance_constraint!"
        );
    }
}

#[test]
#[ignore]
fn havoc_physics_oob_panic_inner() {
    // Only run this test if explicitly requested, as it is designed to panic.
    if std::env::args().any(|arg| arg == "havoc_physics_oob_panic_inner") {
        let mut system = PbdSystem::new();
        // Add just a couple of particles.
        let _ = system.add_particle(Vec3::ZERO, 1.0);
        let _ = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);

        // Boom. index out of bounds. The len is 2, we ask for 100.
        // It's going to access self.particles[100] and panic if bounds are not checked.
        let _ = system.add_distance_constraint(100, 200, 1.0);

        // Return 0 if successful
        std::process::exit(0);
    }
}
//
