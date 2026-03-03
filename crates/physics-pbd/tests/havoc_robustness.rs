use glam::Vec3;
use physics_pbd::PbdSystem;

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

    // Position is silently NaN without a panic!
    assert!(
        system.particles[p1].pos.is_nan(),
        "Havoc expected position to be corrupted to NaN!"
    );
}

#[test]
fn test_havoc_dt_infinity() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::new(1.0, 1.0, 1.0), 1.0);

    system.particles[p1].vel = Vec3::new(1.0, 1.0, 1.0);

    system.step(f32::INFINITY, 0);

    // Position is silently Infinity without a panic!
    assert!(
        !system.particles[p1].pos.is_finite(),
        "Havoc expected position to be corrupted to Infinity!"
    );
}
