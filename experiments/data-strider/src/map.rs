use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use std::path::{Path, PathBuf};
use std::fs;

#[derive(Component)]
pub struct GraphNode {
    pub path: PathBuf,
    pub is_dir: bool,
    pub radius: f32,
}

#[derive(Component)]
pub struct MapRoot;

pub struct MapPlugin;

impl Plugin for MapPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_map);
    }
}

fn spawn_map(mut commands: Commands) {
    // Start scanning from the experiments directory
    let root_path = PathBuf::from("experiments"); // Relative to repo root
    // But since we run the binary from target/debug/deps, or via cargo run, the CWD matters.
    // Usually cargo run sets CWD to the workspace root if run from there, or package root.
    // We'll assume CWD is repo root for now, or fallback.

    let root_path = if root_path.exists() {
        root_path
    } else if PathBuf::from("../experiments").exists() {
        PathBuf::from("../experiments")
    } else {
        // Fallback for when running inside the crate dir
        PathBuf::from(".")
    };

    let nodes = scan_directory(&root_path, 0, Vec2::ZERO, 2000.0);

    for (node_data, position) in nodes {
        let radius = node_data.radius;
        let is_dir = node_data.is_dir;

        let shape = shapes::Circle {
            radius,
            center: Vec2::ZERO,
        };

        let color = if is_dir {
            Color::rgb(0.1, 0.1, 0.2)
        } else {
            Color::rgb(0.3, 0.3, 0.4)
        };

        let stroke_color = if is_dir {
            Color::rgb(0.4, 0.6, 0.8)
        } else {
            Color::rgb(0.6, 0.6, 0.6)
        };

        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                spatial: SpatialBundle {
                    transform: Transform::from_translation(position.extend(-1.0)), // Z=-1 for background
                    ..default()
                },
                ..default()
            },
            Fill::color(color),
            Stroke::new(stroke_color, 2.0),
            Collider::ball(radius),
            node_data,
        ));
    }
}

// Returns a flat list of nodes and their calculated positions
fn scan_directory(path: &Path, depth: u32, center: Vec2, available_radius: f32) -> Vec<(GraphNode, Vec2)> {
    let mut results = Vec::new();

    // Add self
    let is_dir = path.is_dir();
    let node_radius = if is_dir { 40.0 } else { 15.0 };

    results.push((
        GraphNode {
            path: path.to_path_buf(),
            is_dir,
            radius: node_radius,
        },
        center,
    ));

    if depth > 3 {
        return results;
    }

    if is_dir {
        if let Ok(entries) = fs::read_dir(path) {
            let entries: Vec<_> = entries.filter_map(|e| e.ok()).collect();
            let count = entries.len();

            if count == 0 {
                return results;
            }

            // Arrange children in a circle around center
            // The radius of this ring depends on how many children
            let ring_radius = if count < 5 {
                node_radius * 3.0
            } else {
                node_radius * (count as f32) * 1.5
            };

            // Limit ring radius to available space
            let ring_radius = ring_radius.min(available_radius * 0.8);

            let angle_step = std::f32::consts::TAU / (count as f32);

            for (i, entry) in entries.iter().enumerate() {
                let angle = i as f32 * angle_step;
                let child_pos = center + Vec2::new(angle.cos(), angle.sin()) * ring_radius;

                // Recursively scan children
                // Give them a smaller available radius
                let child_available = available_radius / 3.0;

                let child_nodes = scan_directory(&entry.path(), depth + 1, child_pos, child_available);
                results.extend(child_nodes);
            }
        }
    }

    results
}
