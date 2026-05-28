use glam::Vec3;
use physics_pbd::PbdSystem;

#[test]
fn test_solve_distance_w1_w2_infinite_and_distance_0() {
    let mut system = PbdSystem::new();
    system.add_particle(Vec3::ZERO, 0.0);
    system.add_particle(Vec3::ZERO, 0.0);

    // Test infinite mass returning early
    let _ = system.add_distance_constraint(0, 1, 10.0);
    system.step(0.016, 1);

    // Distance 0 returning early
    let mut system = PbdSystem::new();
    system.add_particle(Vec3::ZERO, 1.0);
    system.add_particle(Vec3::ZERO, 1.0);

    let _ = system.add_distance_constraint(0, 1, 10.0);
    system.step(0.016, 1);
}

#[test]
#[should_panic]
fn test_distance_nan() {
    let mut system = PbdSystem::new();
    system.add_particle(Vec3::ZERO, 1.0);
    system.add_particle(Vec3::ZERO, 1.0);
    let _ = system.add_distance_constraint(0, 1, f32::NAN);
}

#[test]
#[should_panic]
fn test_actuator_nan() {
    let mut system = PbdSystem::new();
    system.add_particle(Vec3::ZERO, 1.0);
    system.add_particle(Vec3::ZERO, 1.0);
    let _ = system.add_actuator_constraint(0, 1, 5.0, 10.0, f32::NAN);
}
