use glam::Vec3;
use physics_pbd::{Constraint, PbdSystem};

#[test]
fn test_default_impl() {
    let system = PbdSystem::default();
    assert_eq!(system.particles.len(), 0);
    assert_eq!(system.constraints.len(), 0);
}

#[test]
fn test_distance_both_infinite_mass() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 0.0);
    let p2 = system.add_particle(Vec3::new(2.0, 0.0, 0.0), 0.0);

    // This should hit the `(w1 + w2).abs() < f32::EPSILON` check and return early
    system.add_distance_constraint(p1, p2, 1.0);
    system.step(0.1, 10);

    // They shouldn't have moved
    assert_eq!(system.particles[p1].pos, Vec3::ZERO);
    assert_eq!(system.particles[p2].pos, Vec3::new(2.0, 0.0, 0.0));
}

#[test]
fn test_pin_zombie() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);

    // Add pin constraint to p1
    system.add_pin_constraint(p1, Vec3::new(5.0, 5.0, 5.0));

    // Remove the particle
    system.particles.pop();

    // The index p1 is now a zombie.
    // The solver should just skip it, not panic.
    system.step(0.1, 1);
}

#[test]
fn test_step_nan_infinity_dt() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    system.particles[p1].vel = Vec3::new(1.0, 0.0, 0.0);

    // Infinity dt should return early
    system.step(f32::INFINITY, 1);
    assert_eq!(system.particles[p1].pos, Vec3::ZERO);

    // NaN dt should return early
    system.step(f32::NAN, 1);
    assert_eq!(system.particles[p1].pos, Vec3::ZERO);
}

#[test]
fn test_solve_distance_w1_w2_infinite() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::new(2.0, 0.0, 0.0), 1.0);

    system.particles[p1].inv_mass = f32::INFINITY;
    system.particles[p2].inv_mass = f32::INFINITY;

    system.add_distance_constraint(p1, p2, 1.0);

    // step should return early because (w1 + w2).is_finite() is false
    system.step(0.1, 1);

    assert_eq!(system.particles[p1].pos, Vec3::ZERO);
}

#[test]
fn test_add_actuator_factor_nan() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::new(2.0, 0.0, 0.0), 1.0);

    system.add_actuator_constraint(p1, p2, 1.0, 2.0, 1.0);

    // Make factor NaN
    if let Constraint::Actuator { factor, .. } = &mut system.constraints[0] {
        *factor = f32::NAN;
    }
}

#[test]
#[should_panic(expected = "NaN detected - invalid factor")]
fn test_actuator_nan_factor_panic() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);

    system.add_actuator_constraint(p1, p2, 1.0, 2.0, 1.0);

    if let Constraint::Actuator { factor, .. } = &mut system.constraints[0] {
        *factor = f32::NAN;
    }

    // Step should panic
    system.step(0.1, 1);
}

#[test]
#[should_panic(expected = "Stiffness must be finite")]
fn test_add_distance_constraint_nan_stiffness() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);
    system.add_distance_constraint(p1, p2, f32::NAN);
}

#[test]
#[should_panic(expected = "Constraint parameters must be finite")]
fn test_add_actuator_constraint_nan_params() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);
    system.add_actuator_constraint(p1, p2, f32::NAN, 2.0, 1.0);
}
