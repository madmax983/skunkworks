use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod walker;
mod terrain;
mod gait;
mod visuals;

use walker::{spawn_walker, walker_system};
use terrain::Terrain;
use visuals::{setup_camera, draw_terrain, draw_walker, camera_follow};

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::srgb(0.0, 0.0, 0.0)))
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Genesis Walker".into(),
                resolution: (1024.0, 600.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .insert_resource(Terrain::new(".")) // Scan current directory
        .add_systems(Startup, (setup_camera, spawn_walker))
        .add_systems(Update, (
            walker_system,
            camera_follow,
            draw_terrain,
            draw_walker
        ))
        .run();
}
