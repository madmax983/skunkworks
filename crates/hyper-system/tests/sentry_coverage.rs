use hyper_system::physics::PbdSystem4D;

#[test]
fn test_default_impl_coverage() {
    let system = PbdSystem4D::default();
    assert_eq!(system.particles.len(), 0);
}

#[test]
#[should_panic]
fn test_solve_distance_nan_params() {
    let mut system = PbdSystem4D::new();
    let p1 = system.add_particle(hyper_system::math::Vec4::zero(), 1.0).unwrap();
    let p2 = system.add_particle(hyper_system::math::Vec4::zero(), 1.0).unwrap();

    // Test poison
    system.add_distance_constraint(p1, p2, f32::NAN);
    system.step(0.1, 1, 0.99);
}

#[test]
#[should_panic]
fn test_solve_actuator_nan_factor() {
    let mut system = PbdSystem4D::new();
    let p1 = system.add_particle(hyper_system::math::Vec4::zero(), 1.0).unwrap();
    let p2 = system.add_particle(hyper_system::math::Vec4::zero(), 1.0).unwrap();

    system.add_actuator_constraint(p1, p2, 1.0, 2.0, f32::NAN, 1.0);
    system.step(0.1, 1, 0.99);
}

#[test]
fn test_solve_distance_invalid_particle() {
    let mut system = PbdSystem4D::new();
    let p1 = system.add_particle(hyper_system::math::Vec4::zero(), 1.0).unwrap();

    // Add distance constraint to non-existent particle
    system.add_distance_constraint(p1, 999, 1.0);
    // step should return safely without panic
    system.step(0.1, 1, 0.99);
}

#[test]
fn test_solve_pin() {
    let mut system = PbdSystem4D::new();
    let p1 = system.add_particle(hyper_system::math::Vec4::zero(), 1.0).unwrap();
    system.add_pin_constraint(p1, hyper_system::math::Vec4::new(1.0, 1.0, 1.0, 1.0)).unwrap();
    system.step(0.1, 1, 0.99);
    assert_eq!(system.particles[p1].pos, hyper_system::math::Vec4::new(1.0, 1.0, 1.0, 1.0));
}
