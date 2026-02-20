use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

mod stage;
mod skeleton;
mod kinematics;
mod choreographer;
mod visuals;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default()) // Keep debug for verification
        .add_plugins(stage::StagePlugin)
        .add_plugins(skeleton::SkeletonPlugin)
        .add_plugins(kinematics::KinematicsPlugin)
        .add_plugins(choreographer::ChoreographerPlugin)
        .add_plugins(visuals::VisualsPlugin)
        .add_systems(Startup, setup_camera)
        .add_systems(Update, camera_follow)
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn camera_follow(
    mut camera_query: Query<&mut Transform, (With<Camera>, Without<skeleton::Dancer>)>,
    dancer_query: Query<&Transform, With<skeleton::Dancer>>,
) {
    if let Ok(dancer_transform) = dancer_query.get_single() {
        if let Ok(mut camera_transform) = camera_query.get_single_mut() {
            camera_transform.translation.x = dancer_transform.translation.x;
            camera_transform.translation.y = dancer_transform.translation.y;
        }
    }
}
