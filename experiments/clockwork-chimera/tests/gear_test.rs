use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use clockwork_chimera::{mechanism, EscapeWheel, ClockworkChimeraPlugin};

#[test]
fn test_gear_rotation() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0));
    app.add_plugins(ClockworkChimeraPlugin);

    app.add_systems(Startup, setup_test_scene);

    // Run enough frames for physics to accelerate and move
    // MinimalPlugins might run very fast, so dt is small.
    // We can manually set time if needed, but let's just run more frames.
    for _ in 0..100 {
        app.update();
        std::thread::sleep(std::time::Duration::from_millis(1)); // Ensure some time passes for TimePlugin
    }

    // Query the wheel angle
    let mut query = app.world_mut().query::<(&Transform, &EscapeWheel)>();
    let (transform, _wheel) = query.single(app.world());

    let angle = transform.rotation.to_euler(EulerRot::XYZ).2;
    println!("Final Angle: {}", angle);

    // Since we apply torque, it should have rotated (negative Z usually)
    // -1000 torque -> negative angle.
    assert!(
        angle.abs() > 0.001,
        "Wheel should have rotated. Angle: {}",
        angle
    );
}

fn setup_test_scene(mut commands: Commands) {
    let wheel = mechanism::spawn_gear(&mut commands, Vec2::ZERO, 12, 3.0, 1.0);
    commands
        .entity(wheel)
        .insert(ExternalForce {
            torque: -1000.0,
            ..default()
        })
        .insert(EscapeWheel {
            last_angle: 0.0,
            teeth: 12,
            cumulative_angle: 0.0,
        });
}
