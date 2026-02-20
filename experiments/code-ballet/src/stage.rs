use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use walkdir::WalkDir;

#[derive(Component)]
pub struct Platform {
    pub file_path: String,
    pub size: f32,
}

#[derive(Resource, Default)]
pub struct StageMap {
    pub platforms: Vec<Entity>,
}

pub struct StagePlugin;

impl Plugin for StagePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<StageMap>()
           .add_systems(Startup, spawn_stage);
    }
}

fn spawn_stage(
    mut commands: Commands,
    mut stage_map: ResMut<StageMap>,
) {
    let mut x_cursor = -500.0;

    // Spawn a starting platform
    let start_platform = commands.spawn((
        Platform {
            file_path: "START".to_string(),
            size: 1000.0,
        },
        Collider::cuboid(250.0, 10.0), // Half-extents
        SpatialBundle::from_transform(Transform::from_xyz(x_cursor, -50.0, 0.0)),
    )).id();
    stage_map.platforms.push(start_platform);
    x_cursor += 300.0;

    // Limit to 50 files to avoid infinite world for now
    for entry in WalkDir::new(".").max_depth(2).into_iter().filter_map(|e| e.ok()).take(50) {
        let path = entry.path();
        if path.is_dir() { continue; }

        let metadata = entry.metadata().ok();
        let size = metadata.map(|m| m.len()).unwrap_or(100) as f32;

        // Map size to width
        let width = (size.max(100.0) as f32).log10() * 50.0; // Log scale
        let height = 10.0;

        // Pseudorandom Y based on file name hash
        let hash = path.to_string_lossy().chars().fold(0, |acc, c| acc + c as u32);
        let y_pos = -100.0 + (hash % 200) as f32;

        let entity = commands.spawn((
            Platform {
                file_path: path.to_string_lossy().to_string(),
                size,
            },
            Collider::cuboid(width / 2.0, height),
            SpatialBundle::from_transform(Transform::from_xyz(x_cursor, y_pos, 0.0)),
        )).id();

        stage_map.platforms.push(entity);

        x_cursor += width + 150.0; // Gap
    }
}
