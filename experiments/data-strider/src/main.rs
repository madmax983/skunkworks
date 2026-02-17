mod map;
mod ik;
mod strider;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use map::MapPlugin;
use strider::StriderPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Data Strider".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(MapPlugin)
        .add_plugins(StriderPlugin)
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.08)))
        .add_systems(Startup, setup)
        .add_systems(Update, camera_follow)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Instructions
    commands.spawn(TextBundle::from_section(
        "WASD to Move the Strider\nIt walks on the file system nodes.\nLegs adapt procedurally.",
        TextStyle {
            font_size: 20.0,
            color: Color::WHITE,
            ..default()
        },
    ).with_style(Style {
        position_type: PositionType::Absolute,
        top: Val::Px(10.0),
        left: Val::Px(10.0),
        ..default()
    }));
}

fn camera_follow(
    time: Res<Time>,
    mut camera_q: Query<&mut Transform, With<Camera>>,
    strider_q: Query<&Transform, (With<strider::StriderBody>, Without<Camera>)>,
) {
    let mut cam_transform = camera_q.single_mut();

    if let Ok(strider_transform) = strider_q.get_single() {
        let target = strider_transform.translation;
        let speed = 2.0;

        // Smooth follow
        let current = cam_transform.translation;
        let new_pos = current.lerp(target, time.delta_seconds() * speed);

        cam_transform.translation.x = new_pos.x;
        cam_transform.translation.y = new_pos.y;

        // Keep Z same?
        // Default camera Z is 1000.0 or so. Let's keep it.
        // Wait, lerp affects Z too.
        // If target Z is 10 (strider), camera Z becomes 10?
        // Then camera clips everything (Z=-1 to 10).
        // Camera needs to stay high.
        cam_transform.translation.z = 999.0;
    }
}
