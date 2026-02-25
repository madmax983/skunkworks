use glam::Vec3;
use physics_pbd::PbdSystem;

#[test]
#[should_panic(expected = "NaN detected")]
fn test_nan_position_panic() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);

    // Add constraint so solver runs
    system.add_distance_constraint(p1, p2, 1.0);

    // Inject NaN directly into position
    // This bypasses add_particle checks
    system.particles[p1].pos = Vec3::NAN;

    // Step should panic when it encounters the NaN in solver
    system.step(0.1, 1);
}

#[test]
#[should_panic(expected = "NaN detected")]
fn test_nan_actuator_panic() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);

    // Inject NaN into actuator min_len
    system.add_actuator_constraint(p1, p2, f32::NAN, 2.0, 1.0);

    system.step(0.1, 1);
}
