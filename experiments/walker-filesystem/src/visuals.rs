use crate::terrain::{Terrain, NODE_GAP, NODE_WIDTH, START_X};
use crate::walker::Walker;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

pub fn camera_follow(
    mut camera_query: Query<&mut Transform, With<Camera>>,
    walker_query: Query<&Walker>,
) {
    if let Ok(walker) = walker_query.get_single() {
        if let Ok(mut transform) = camera_query.get_single_mut() {
            // Smooth follow? Or strict? Strict for now.
            // Keep walker at 1/3 of screen (approx -200 offset from center if screen is ~1000 wide)
            transform.translation.x = walker.hip_pos.x + 200.0;
        }
    }
}

pub fn draw_terrain(mut commands: Commands, terrain: Res<Terrain>, mut scanned: Local<bool>) {
    if *scanned || terrain.nodes.is_empty() {
        return;
    }

    let mut path_builder = PathBuilder::new();
    let mut x = START_X;

    for node in &terrain.nodes {
        let width = NODE_WIDTH;

        let y_level = match node.file_type {
            crate::terrain::FileType::Directory => -50.0,
            crate::terrain::FileType::File => -100.0,
        };

        path_builder.move_to(Vec2::new(x, y_level));
        path_builder.line_to(Vec2::new(x + width, y_level));

        x += width + NODE_GAP;
    }

    let path = path_builder.build();

    commands.spawn((
        ShapeBundle {
            path,
            spatial: SpatialBundle::default(),
            ..default()
        },
        Stroke::new(Color::WHITE, 2.0),
        Fill::color(Color::NONE),
    ));

    *scanned = true;
}

pub fn draw_walker(mut gizmos: Gizmos, query: Query<&Walker>) {
    let red = Color::srgb(1.0, 0.0, 0.0);
    let green = Color::srgb(0.0, 1.0, 0.0);
    let blue = Color::srgb(0.0, 0.0, 1.0);

    for walker in &query {
        // Hip
        gizmos.circle_2d(walker.hip_pos, 5.0, red);

        // Left Leg (Green)
        gizmos.line_2d(walker.hip_pos, walker.left_knee, green);
        gizmos.line_2d(walker.left_knee, walker.left_foot, green);
        gizmos.circle_2d(walker.left_foot, 3.0, green);

        // Right Leg (Blue)
        gizmos.line_2d(walker.hip_pos, walker.right_knee, blue);
        gizmos.line_2d(walker.right_knee, walker.right_foot, blue);
        gizmos.circle_2d(walker.right_foot, 3.0, blue);

        // Target debug
        gizmos.circle_2d(walker.left_foot_target, 2.0, green.with_alpha(0.5));
        gizmos.circle_2d(walker.right_foot_target, 2.0, blue.with_alpha(0.5));
    }
}
