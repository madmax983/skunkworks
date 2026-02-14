use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use clockwork_cipher::gear;
use std::f32::consts::PI;

#[test]
fn test_gear_ratio() {
    let mut app = App::new();
    app.add_plugins(MinimalPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0));

    // Setup
    app.add_systems(Startup, setup_scene);

    // Initial update to run Startup
    app.update();

    for _ in 0..2000 {
        app.update();
        std::thread::sleep(std::time::Duration::from_micros(100));
    }

    // Check results
    let world = app.world_mut();
    let mut query = world.query::<(&Name, &Transform)>();
    let mut driver_angle: f32 = 0.0;
    let mut follower_angle: f32 = 0.0;

    let initial_follower_angle = PI / 12.0;

    for (name, transform) in query.iter(world) {
        let angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        if name.as_str() == "Driver" {
            driver_angle = angle;
        } else if name.as_str() == "Follower" {
            follower_angle = angle;
        }
    }

    // Calculate delta
    let follower_delta = follower_angle - initial_follower_angle;
    let driver_delta = driver_angle;

    println!(
        "Driver Delta: {}, Follower Delta: {}",
        driver_delta, follower_delta
    );

    if driver_delta.abs() < 0.001 {
        println!("WARNING: Gears did not move significantly.");
        panic!("Gears did not rotate.");
    }

    let ratio = follower_delta / driver_delta;
    println!("Measured Ratio: {}", ratio);

    // Check if they rotate in opposite directions
    // Driver and Follower delta should have opposite signs.
    if ratio > 0.0 {
        println!("WARNING: Gears rotating in SAME direction! Mesh failure.");
    }

    // Ratio should be around -5.0
    assert!(
        (ratio - (-5.0)).abs() < 1.0,
        "Expected ratio ~ -5.0, got {}",
        ratio
    );
}

fn setup_scene(mut commands: Commands) {
    let module = 1.0; // Smaller module
    let pressure_angle = 20.0;

    // Driver (60 teeth)
    let driver_teeth = 60;
    let driver_radius = (driver_teeth as f32 * module) / 2.0;
    let driver_pos = Vec2::ZERO;

    commands.spawn((
        SpatialBundle::from_transform(Transform::from_translation(driver_pos.extend(0.0))),
        RigidBody::Dynamic,
        gear::generate_gear_collider(driver_teeth, module, pressure_angle),
        ExternalForce {
            torque: 20000.0,
            ..default()
        },
        LockedAxes::TRANSLATION_LOCKED,
        Damping {
            linear_damping: 0.1,
            angular_damping: 0.1,
        },
        ColliderMassProperties::Mass(1.0),
        Name::new("Driver"),
    ));

    // Follower (12 teeth)
    let follower_teeth = 12;
    let follower_radius = (follower_teeth as f32 * module) / 2.0;
    let center_dist = driver_radius + follower_radius + 0.5; // Gap
    let follower_pos = Vec2::new(center_dist, 0.0);

    let phase_offset = PI / follower_teeth as f32;

    commands.spawn((
        SpatialBundle::from_transform(
            Transform::from_translation(follower_pos.extend(0.0))
                .with_rotation(Quat::from_rotation_z(phase_offset)),
        ),
        RigidBody::Dynamic,
        gear::generate_gear_collider(follower_teeth, module, pressure_angle),
        ColliderMassProperties::Mass(0.2),
        LockedAxes::TRANSLATION_LOCKED,
        Damping {
            linear_damping: 0.1,
            angular_damping: 0.1,
        },
        Name::new("Follower"),
    ));
}
