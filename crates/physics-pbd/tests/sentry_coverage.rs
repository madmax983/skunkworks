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
    system.constraints.push(Constraint::Distance {
        p1,
        p2,
        rest_length: 1.0,
        stiffness: 1.0,
    });
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

    system.constraints.push(Constraint::Distance {
        p1,
        p2,
        rest_length: 1.0,
        stiffness: 1.0,
    });

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

#[test]
fn test_solve_pin_static() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 0.0);
    system.add_pin_constraint(p1, Vec3::new(5.0, 5.0, 5.0));

    system.step(0.1, 1);

    // The particle was a static mass but should have moved to (5,5,5) due to pin constraints overriding dynamics
    assert_eq!(system.particles[p1].pos, Vec3::new(5.0, 5.0, 5.0));
}

#[test]
fn test_solve_distance_zero_len() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::ZERO, 1.0);

    system.constraints.push(Constraint::Distance {
        p1,
        p2,
        rest_length: 1.0,
        stiffness: 1.0,
    });

    system.step(0.1, 1);

    assert_eq!(system.particles[p1].pos, Vec3::ZERO);
    assert_eq!(system.particles[p2].pos, Vec3::ZERO);
}

#[test]
fn test_solve_distance_nan_len() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::ZERO, 1.0);

    // Bypassing normal add_particle checks
    system.particles[p2].pos = Vec3::NAN;

    system.constraints.push(Constraint::Distance {
        p1,
        p2,
        rest_length: 1.0,
        stiffness: 1.0,
    });

    // This should panic
    let result = std::panic::catch_unwind(std::panic::AssertUnwindSafe(|| {
        system.step(0.1, 1);
    }));

    assert!(result.is_err());
}

#[test]
fn test_solve_pin_get_mut() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    // Pin it
    system.add_pin_constraint(p1, Vec3::new(1.0, 1.0, 1.0));

    // Simulate
    system.step(0.1, 1);

    assert_eq!(system.particles[p1].pos, Vec3::new(1.0, 1.0, 1.0));
}

#[test]
fn test_distance_stiffness_clamping() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::new(2.0, 0.0, 0.0), 1.0);

    // Stiffness > 1.0 should be clamped to 1.0
    system.add_distance_constraint(p1, p2, 2.0);

    system.step(0.1, 1);
}

#[test]
fn test_distance_w1_gt_0_w2_gt_0() {
    let mut system = PbdSystem::new();
    // One static, one dynamic
    let p1 = system.add_particle(Vec3::ZERO, 0.0);
    let p2 = system.add_particle(Vec3::new(2.0, 0.0, 0.0), 1.0);

    system.constraints.push(Constraint::Distance {
        p1,
        p2,
        rest_length: 1.0,
        stiffness: 1.0,
    });

    system.step(0.1, 1);

    // Only p2 should have moved
    assert_eq!(system.particles[p1].pos, Vec3::ZERO);
    assert_eq!(system.particles[p2].pos, Vec3::new(1.0, 0.0, 0.0));
}

#[test]
fn test_distance_w1_gt_0_w2_gt_0_rev() {
    let mut system = PbdSystem::new();
    // One static, one dynamic
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::new(2.0, 0.0, 0.0), 0.0);

    system.constraints.push(Constraint::Distance {
        p1,
        p2,
        rest_length: 1.0,
        stiffness: 1.0,
    });

    system.step(0.1, 1);

    // Only p1 should have moved
    assert_eq!(system.particles[p1].pos, Vec3::new(1.0, 0.0, 0.0));
    assert_eq!(system.particles[p2].pos, Vec3::new(2.0, 0.0, 0.0));
}

#[test]
fn test_solve_pin_get_mut_none() {
    let mut system = PbdSystem::new();
    let _p1 = system.add_particle(Vec3::ZERO, 1.0);
    // Pin a particle that doesn't exist
    system.add_pin_constraint(999, Vec3::new(1.0, 1.0, 1.0));

    // Simulate
    system.step(0.1, 1);
}

#[test]
fn test_solve_distance_w1_w2_neg_inf() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    let p2 = system.add_particle(Vec3::new(2.0, 0.0, 0.0), 1.0);
    system.particles[p1].inv_mass = f32::NEG_INFINITY;
    system.particles[p2].inv_mass = f32::NEG_INFINITY;
    system.add_distance_constraint(p1, p2, 1.0);
    system.step(0.1, 1);
}

#[test]
fn test_solve_distance_w1_w2_zero() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 0.0);
    let p2 = system.add_particle(Vec3::new(2.0, 0.0, 0.0), 0.0);
    system.add_distance_constraint(p1, p2, 1.0);
    system.step(0.1, 1);
}

#[test]
fn test_solve_distance_p1_out_of_bounds() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    // Directly push constraint to bypass checks
    system.constraints.push(Constraint::Distance {
        p1: 999,
        p2: p1,
        rest_length: 1.0,
        stiffness: 1.0,
    });
    system.step(0.1, 1);
}

#[test]
fn test_solve_distance_p2_out_of_bounds() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::ZERO, 1.0);
    system.constraints.push(Constraint::Distance {
        p1,
        p2: 999,
        rest_length: 1.0,
        stiffness: 1.0,
    });
    system.step(0.1, 1);
}
