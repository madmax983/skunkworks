use glam::Vec3;
use physics_pbd::PbdSystem;
use proptest::prelude::*;

#[test]
fn test_havoc_dt_nan_poison() {
    // 👺 HAVOC: Poison the simulation via unchecked `dt` parameter.
    //
    // EXPECTATION: The physics system checks for NaN parameters before they infect the state.
    // REALITY: The `step(dt, ...)` method checks `dt <= EPSILON` but does not check `dt.is_finite()`.
    // Passing f32::NAN fails the `<= EPSILON` check (returns false), so execution continues.
    // `p.pos += p.vel * NaN` -> `p.pos` becomes NaN silently.

    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::new(1.0, 1.0, 1.0), 1.0);

    system.particles[p1].vel = Vec3::new(1.0, 1.0, 1.0);

    system.step(f32::NAN, 0);

    // Position should NOT be corrupted to NaN anymore, because it skips update on NaN.
    assert!(
        !system.particles[p1].pos.is_nan(),
        "Warden defense failed: position became NaN!"
    );
}

#[test]
fn test_havoc_dt_infinity() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::new(1.0, 1.0, 1.0), 1.0);

    system.particles[p1].vel = Vec3::new(1.0, 1.0, 1.0);

    system.step(f32::INFINITY, 0);

    // Position should NOT be corrupted to Infinity anymore, because it skips update on Infinity.
    assert!(
        system.particles[p1].pos.is_finite(),
        "Warden defense failed: position became Infinity!"
    );
}

#[test]
#[should_panic(expected = "NaN detected in constraint parameters")]
fn test_havoc_distance_nan() {
    let mut system = physics_pbd::PbdSystem::new();
    let p1 = system.add_particle(glam::Vec3::new(f32::MAX, 0.0, 0.0), 1.0);
    let p2 = system.add_particle(glam::Vec3::new(-f32::MAX, 0.0, 0.0), 1.0);
    // This will calculate `dist = Infinity` and eventually inject a NaN.
    system.add_distance_constraint(p1, p2, 1.0);
    system.step(0.1, 1);
}

#[test]
#[should_panic(expected = "NaN detected in constraint parameters")]
fn test_havoc_distance_nan_2() {
    let mut system = physics_pbd::PbdSystem::new();
    let p1 = system.add_particle(glam::Vec3::new(0.0, 0.0, 0.0), 1.0);
    let p2 = system.add_particle(glam::Vec3::new(0.0, 0.0, 0.0), 1.0);
    // Explicitly injecting NaN
    system.add_distance_constraint(p1, p2, f32::NAN);
    system.step(0.1, 1);
}

// 👺 HAVOC: True property testing of extreme invalid inputs.
// We explicitly use prop_oneof over known non-finite inputs to ensure
// proptest generates failures deterministically without exceeding rejection limits.
proptest! {
    #[test]
    #[should_panic(expected = "NaN detected in constraint parameters")]
    fn test_distance_stiffness_and_dist_fuzz(
        dist in prop_oneof![Just(f32::NAN), Just(f32::INFINITY), Just(f32::NEG_INFINITY)],
        stiffness in prop_oneof![Just(f32::NAN), Just(f32::INFINITY), Just(f32::NEG_INFINITY)]
    ) {
        let mut system = physics_pbd::PbdSystem::new();
        let p1 = system.add_particle(glam::Vec3::new(0.0, 0.0, 0.0), 1.0);
        let p2 = system.add_particle(glam::Vec3::new(dist, 0.0, 0.0), 1.0);
        system.add_distance_constraint(p1, p2, stiffness);

        system.step(0.1, 1);
    }
}
