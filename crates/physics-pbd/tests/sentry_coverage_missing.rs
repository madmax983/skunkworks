use glam::Vec3;
use physics_pbd::{Constraint, PbdSystem};

#[test]
fn test_actuator_nan() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::ZERO, 1.0);
    let _ = system.add_actuator_constraint(p1, p2, 5.0, 10.0, 1.0);
    if let Constraint::Actuator { factor, .. } = &mut system.constraints[0] {
        *factor = f32::NAN;
    }
    system.step(0.016, 1);
}

#[test]
fn test_pin_missing() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let _ = system.add_pin_constraint(p1, Vec3::ZERO);

    if let Constraint::Pin { p, .. } = &mut system.constraints[0] {
        *p = 100; // Out of bounds
    }
    system.step(0.016, 1);
}

#[test]
fn test_distance_out_of_bounds() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::ZERO, 1.0);

    let _ = system.add_distance_constraint(p1, p2, 10.0);

    if let Constraint::Distance { p1: p1_ref, .. } = &mut system.constraints[0] {
        *p1_ref = 100;
    }
    system.step(0.016, 1);

    if let Constraint::Distance {
        p1: p1_ref,
        p2: p2_ref,
        ..
    } = &mut system.constraints[0]
    {
        *p1_ref = p1;
        *p2_ref = 100;
    }
    system.step(0.016, 1);
}

#[test]
fn test_distance_target_len_nan() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::ZERO, 1.0);

    let _ = system.add_distance_constraint(p1, p2, 10.0);
    if let Constraint::Distance { rest_length, .. } = &mut system.constraints[0] {
        *rest_length = f32::NAN;
    }
    system.step(0.016, 1);
}

#[test]
fn test_distance_stiffness_nan() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::ZERO, 1.0);

    let _ = system.add_distance_constraint(p1, p2, 10.0);
    if let Constraint::Distance { stiffness, .. } = &mut system.constraints[0] {
        *stiffness = f32::NAN;
    }
    system.step(0.016, 1);
}

#[test]
fn test_distance_w1_w2_infinite() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::ZERO, 1.0);

    let _ = system.add_distance_constraint(p1, p2, 10.0);
    system.particles[p1].inv_mass = f32::NAN;
    system.step(0.016, 1);
}

#[test]
fn test_distance_w1_w2_zero() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 0.0);
    let p2 = system.add_particle(Vec3::ZERO, 0.0);

    let _ = system.add_distance_constraint(p1, p2, 10.0);
    system.step(0.016, 1);
}

#[test]
fn test_distance_delta_nan() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::ZERO, 1.0);
    let _ = system.add_distance_constraint(p1, p2, 10.0);
    system.particles[p1].pos.x = f32::NAN;
    system.step(0.016, 1);
}

#[test]
fn test_distance_length_nan() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::ZERO, 1.0);
    let _ = system.add_distance_constraint(p1, p2, 10.0);
    // Use MAX values to cause delta.length() to overflow and return INF/NAN
    system.particles[p1].pos = Vec3::new(f32::MAX, f32::MAX, f32::MAX);
    system.particles[p2].pos = Vec3::new(-f32::MAX, -f32::MAX, -f32::MAX);
    system.step(0.016, 1);
}

#[test]
fn test_actuator_nan_factor_solve() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::ZERO, 1.0);
    let _ = system.add_actuator_constraint(p1, p2, 1.0, 2.0, 1.0);
    if let Constraint::Actuator { factor, .. } = &mut system.constraints[0] {
        *factor = f32::NAN;
    }
    system.step(0.016, 1);
}

#[test]
fn test_solve_w1_gt_0_w2_zero() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);
    let p2 = system.add_particle(Vec3::ZERO, 0.0);
    let _ = system.add_distance_constraint(p1, p2, 2.0);
    system.step(0.016, 1);
}

#[test]
fn test_solve_w1_zero_w2_gt_0() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 0.0);
    let p2 = system.add_particle(Vec3::ZERO, 1.0);
    let _ = system.add_distance_constraint(p1, p2, 2.0);
    system.step(0.016, 1);
}
