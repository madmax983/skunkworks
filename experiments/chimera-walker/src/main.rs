use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod gait;
mod terrain;
mod visuals;
mod walker;
mod brain;

use terrain::Terrain;
use visuals::{camera_follow, draw_terrain, draw_walker, setup_camera};
use walker::{spawn_walker, walker_system};
use brain::brain_input_system;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chimera Walker".into(),
                resolution: (1024.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .insert_resource(Terrain::new(".")) // Scan current directory
        .add_systems(Startup, (setup_camera, spawn_walker))
        .add_systems(
            Update,
            (walker_system, camera_follow, draw_terrain, draw_walker, brain_input_system),
        )
        .run();
}
