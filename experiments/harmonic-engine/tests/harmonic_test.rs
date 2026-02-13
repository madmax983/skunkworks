use harmonic_engine::physics::PhysicsWorld;
use nalgebra::Vector2;

#[test]
fn test_integrator_mechanics() {
    let mut world = PhysicsWorld::new();

    // Add integrator
    let idx = world.add_integrator(Vector2::new(0.0, 0.0));

    // Step simulation
    for _ in 0..10 {
        world.step();
    }

    // Check if output cylinder rotated
    // Since default ball pos is 5.0 and disk speed is 1.0
    // w_cyl = 1.0 * 5.0 * 0.5 = 2.5
    // In 10 steps (default dt 1/60), delta angle should be 2.5 * 10/60 = 0.416

    let output_handle = world.integrators[idx].output_handle;
    let body = world.rigid_body_set.get(output_handle).unwrap();
    let angle = body.rotation().angle();

    assert!(angle.abs() > 0.0, "Cylinder should rotate");
    println!("Angle after 10 steps: {}", angle);
}

#[test]
fn test_oscillator_coupling() {
    let mut world = PhysicsWorld::new();

    // Int 0: y
    let idx_y = world.add_integrator(Vector2::new(-20.0, 0.0));
    // Int 1: v
    let idx_v = world.add_integrator(Vector2::new(20.0, 0.0));

    // Couple: dy/dt = v, dv/dt = -y
    world.add_coupling(idx_v, idx_y, 0.5);
    world.add_coupling(idx_y, idx_v, -0.5);

    // Step
    let mut angles = Vec::new();
    for _ in 0..100 {
        world.step();
        let body = world
            .rigid_body_set
            .get(world.integrators[idx_y].output_handle)
            .unwrap();
        angles.push(body.rotation().angle());
    }

    // Check for change
    assert!(
        angles.last().unwrap() != angles.first().unwrap(),
        "Oscillator should move"
    );

    // Check if ball moved (Value oscillated)
    let ball = world
        .rigid_body_set
        .get(world.integrators[idx_y].ball_handle)
        .unwrap();
    let final_pos = ball.translation().x;
    assert!(
        (final_pos - (-15.0)).abs() > 0.01,
        "Ball should move from initial position (relative to disk center)"
    );
    // Initial: -20.0 + 5.0 = -15.0.
}
