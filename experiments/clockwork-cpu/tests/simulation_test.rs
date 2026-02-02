use clockwork_cpu::physics::PhysicsWorld;
use clockwork_cpu::mechanism::Clockwork;

#[test]
fn test_clockwork_movement() {
    let mut world = PhysicsWorld::new();
    let clockwork = Clockwork::build(&mut world);

    // Initial check
    let initial_wheel_rot = world.rigid_body_set.get(clockwork.wheel_handle).unwrap().rotation().angle();
    let initial_anchor_rot = world.rigid_body_set.get(clockwork.anchor_handle).unwrap().rotation().angle();

    // Run simulation for a bit
    // We need to apply torque manually as main loop does
    for _ in 0..100 {
        if let Some(wheel) = world.rigid_body_set.get_mut(clockwork.wheel_handle) {
            wheel.reset_torques(true);
            wheel.add_torque(-50.0, true);
        }
        world.step();
    }

    let final_wheel_rot = world.rigid_body_set.get(clockwork.wheel_handle).unwrap().rotation().angle();
    let final_anchor_rot = world.rigid_body_set.get(clockwork.anchor_handle).unwrap().rotation().angle();

    println!("Wheel Rot: {} -> {}", initial_wheel_rot, final_wheel_rot);
    println!("Anchor Rot: {} -> {}", initial_anchor_rot, final_anchor_rot);

    // Assert that the wheel moved
    assert!((final_wheel_rot - initial_wheel_rot).abs() > 0.01, "Wheel should have moved");

    // Assert that the anchor moved (oscillation start)
    // Anchor might be slow to start, but 100 steps should be enough for *some* movement
    assert!((final_anchor_rot - initial_anchor_rot).abs() > 0.001, "Anchor should have moved");
}
