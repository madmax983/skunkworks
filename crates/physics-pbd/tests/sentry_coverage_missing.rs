use physics_pbd::{PbdSystem, Constraint};
use glam::Vec3;

#[test]
fn test_default_pbd_system() {
    let mut system = PbdSystem::default();
    assert_eq!(system.particles.len(), 0);
}

#[test]
fn test_solve_actuator_nan_factor() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);

    system.add_actuator_constraint(p1, p2, 0.5, 1.5, 1.0).unwrap();

    if let Constraint::Actuator { factor, .. } = &mut system.constraints[0] {
        *factor = f32::NAN;
    }

    // Since factor is NaN, solve_actuator should return early
    system.step(0.1, 1);

    assert!(system.particles[p1].pos.is_finite());
}

#[test]
fn test_solve_pin() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    system.add_pin_constraint(p1, Vec3::new(5.0, 5.0, 5.0)).unwrap();

    system.step(0.1, 1);
    assert_eq!(system.particles[p1].pos, Vec3::new(5.0, 5.0, 5.0));
}

#[test]
fn test_solve_distance_nan_params() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);

    system.add_distance_constraint(p1, p2, 1.0).unwrap();

    if let Constraint::Distance { rest_length, .. } = &mut system.constraints[0] {
        *rest_length = f32::NAN;
    }

    system.step(0.1, 1);

    if let Constraint::Distance { rest_length, stiffness, .. } = &mut system.constraints[0] {
        *rest_length = 1.0;
        *stiffness = f32::NAN;
    }

    system.step(0.1, 1);
}

#[test]
fn test_solve_distance_invalid_particle() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);

    system.add_distance_constraint(p1, p2, 1.0).unwrap();

    if let Constraint::Distance { p2: p2_ref, .. } = &mut system.constraints[0] {
        *p2_ref = 999;
    }

    system.step(0.1, 1);
}

