use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::git::GitScanner;
use crate::components::CommitNode;

pub struct TerrainPlugin;

impl Plugin for TerrainPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_terrain);
    }
}

fn setup_terrain(mut commands: Commands) {
    if let Ok(commits) = GitScanner::scan() {
        if commits.is_empty() { return; }

        let scale_x = 20.0; // Distance between commits
        let scale_y = 30.0; // Height per stress unit

        let mut points = Vec::new();

        // Start from a bit left
        let start_x = -400.0;

        for (i, commit) in commits.iter().enumerate() {
            let x = start_x + (i as f32 * scale_x);
            // Base height + stress
            // Clamp stress visual to avoid going off screen too much
            let y = -150.0 + (commit.stress_level as f32 * scale_y).clamp(0.0, 300.0);

            let pos = Vec2::new(x, y);
            points.push(pos);

            // Spawn node for walker target
            commands.spawn((
                CommitNode {
                    hash: commit.hash.clone(),
                    stress_level: commit.stress_level as f32,
                    index: i,
                },
                SpatialBundle::from_transform(Transform::from_xyz(x, y, 0.0)),
            ));
        }

        // Draw Path (Strata)
        if points.len() > 1 {
            let mut path_builder = PathBuilder::new();
            path_builder.move_to(points[0]);
            for p in points.iter().skip(1) {
                path_builder.line_to(*p);
            }
            let path = path_builder.build();

            commands.spawn((
                ShapeBundle {
                    path,
                    spatial: SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, -1.0)),
                    ..default()
                },
                Stroke::new(Color::rgb(0.5, 0.5, 0.5), 3.0),
            ));
        }

        // Draw Fissures (High Stress Points)
        for (i, p) in points.iter().enumerate() {
            if commits[i].stress_level > 1.0 {
                 commands.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&shapes::Circle { radius: 4.0, center: Vec2::ZERO }),
                        spatial: SpatialBundle::from_transform(Transform::from_xyz(p.x, p.y, 1.0)),
                        ..default()
                    },
                    Fill::color(Color::RED),
                ));
            }
        }
    } else {
        warn!("Failed to scan git history");
    }
}
