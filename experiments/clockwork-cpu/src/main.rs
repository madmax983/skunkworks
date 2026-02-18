use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

mod components;
mod cpu;
mod escapement;
mod gear_gen;

use components::MainCamera;
use cpu::CpuPlugin;
use escapement::EscapementPlugin;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Clockwork CPU - Verge Escapement".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(EscapementPlugin)
        .add_plugins(CpuPlugin)
        .add_systems(Startup, setup_camera)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn((
        Camera2dBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0), // Center on the escapement
            projection: OrthographicProjection {
                scale: 1.0,
                ..default()
            },
            ..default()
        },
        MainCamera,
    ));
}
