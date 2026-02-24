use physics_pbd::PbdSystem;
use macroquad::prelude::Vec3;

#[test]
fn test_actuator_nan_injection_robustness() {
    let mut system = PbdSystem::new();
    let p1 = system.add_particle(Vec3::new(0.0, 0.0, 0.0), 1.0);
    let p2 = system.add_particle(Vec3::new(1.0, 0.0, 0.0), 1.0);

    // Inject NaN into min_len (via add_actuator_constraint)
    // This should be safely ignored by the solver.
    system.add_actuator_constraint(p1, p2, f32::NAN, 2.0, 0.5);

    // Step the simulation
    // Previously this would propagate NaN to particle positions.
    system.step(0.1, 1);

    // Check if the system survived
    let pos1 = system.particles[p1].pos;
    let pos2 = system.particles[p2].pos;

    println!("Positions after step: p1={:?}, p2={:?}", pos1, pos2);

    assert!(pos1.is_finite(), "Particle 1 position became NaN!");
    assert!(pos2.is_finite(), "Particle 2 position became NaN!");
}
