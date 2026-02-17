use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

mod audio;
mod cpu;
mod mechanism;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Sonata Cycle".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(ShapePlugin)
        .add_plugins(mechanism::MechanismPlugin)
        .add_plugins(cpu::CpuPlugin)
        .add_plugins(audio::AudioPlugin)
        .add_systems(Startup, setup)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Ground
    commands
        .spawn(SpriteBundle {
            transform: Transform::from_xyz(0.0, -300.0, 0.0),
            sprite: Sprite {
                color: Color::srgb(0.2, 0.2, 0.2),
                custom_size: Some(Vec2::new(1000.0, 50.0)),
                ..default()
            },
            ..default()
        })
        .insert(RigidBody::Fixed)
        .insert(Collider::cuboid(500.0, 25.0));
}
