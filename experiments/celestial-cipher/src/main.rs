use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use celestial_cipher::physics::PhysicsPlugin;
use celestial_cipher::cipher::CipherPlugin;
use celestial_cipher::view::ViewPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Celestial Cipher".into(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(10.0))
        .add_plugins(PhysicsPlugin)
        .add_plugins(CipherPlugin)
        .add_plugins(ViewPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}
