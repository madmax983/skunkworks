use glam::Vec3;
use physics_pbd::{Constraint, PbdSystem};

#[test]
#[should_panic]
fn test_havoc_distance_constraint_nan_stiffness() {
    // 👺 HAVOC: If stiffness is NaN, distance constraint should blow up in solve_distance.
    // But the constraint addition doesn't check it, so the solver takes it and explodes.
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::new(0.0, 0.0, 0.0), 1.0);
    let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);

    system.constraints.push(Constraint::Distance {
        p1,
        p2,
        rest_length: 1.0,
        stiffness: f32::NAN,
    });

    // Trigger the explosion
    system.step(0.1, 1);
}

#[test]
#[should_panic]
fn test_havoc_actuator_nan_factor() {
    // 👺 HAVOC: If factor is NaN, actuator constraint panics immediately in solve_actuator.
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::new(0.0, 0.0, 0.0), 1.0);
    let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);

    system.constraints.push(Constraint::Actuator {
        p1,
        p2,
        min_len: 1.0,
        max_len: 2.0,
        factor: f32::NAN,
        stiffness: 1.0,
    });

    // Trigger the explosion
    system.step(0.1, 1);
}
