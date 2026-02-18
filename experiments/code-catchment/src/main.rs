use bevy::prelude::*;
use repo::RepoScanner;
use terrain::{TerrainMap, setup_terrain, update_terrain_mesh};
use erosion::{ErosionConfig, ErosionState, erosion_system, Droplet};

mod repo;
mod terrain;
mod erosion;

#[derive(Resource)]
struct CommitStream {
    files: Vec<repo::FileInfo>,
    current_index: usize,
    timer: Timer,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Code Catchment".into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ErosionConfig::default())
        .insert_resource(ErosionState::default())
        .add_systems(Startup, (setup_app, apply_deferred, setup_terrain).chain())
        .add_systems(Update, (
            simulate_rain,
            erosion_system,
            update_terrain_mesh,
            camera_control,
        ))
        .run();
}

fn setup_app(mut commands: Commands) {
    // 1. Scan Repo
    // We scan current directory "."
    let scanner = RepoScanner::new(".").expect("Failed to open repo");
    let files = scanner.scan_files().unwrap_or_else(|e| {
        eprintln!("Warning: Failed to scan files: {}. Using empty list.", e);
        Vec::new()
    });

    // 2. Init Terrain
    // Grid size from scan
    let count = files.len();
    let width = if count > 0 { (count as f64).sqrt().ceil() as usize } else { 32 };
    let width = width.max(32); // Min size
    let height = width; // Square grid

    let mut terrain = TerrainMap::new(width, height);

    // Populate heights
    for file in &files {
        let (x, y) = file.grid_pos;
        if x < width && y < height {
            let idx = terrain.get_index(x, y);
            // Log scale for height
            let h = (file.line_count as f32).ln() * 3.0;
            terrain.heights[idx] = h.max(1.0); // Minimum height
        }
    }

    commands.insert_resource(terrain);
    commands.insert_resource(CommitStream {
        files: files.clone(),
        current_index: 0,
        timer: Timer::from_seconds(0.05, TimerMode::Repeating),
    });

    // 3. Camera
    commands.spawn(Camera3dBundle {
        transform: Transform::from_xyz(width as f32 * 0.8, width as f32 * 1.2, width as f32 * 0.8)
            .looking_at(Vec3::new(width as f32 * 0.5, 0.0, width as f32 * 0.5), Vec3::Y),
        ..default()
    });

    // 4. Light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 5000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -0.7, -0.5, 0.0)),
        ..default()
    });
}

fn simulate_rain(
    mut stream: ResMut<CommitStream>,
    time: Res<Time>,
    mut erosion_state: ResMut<ErosionState>,
    terrain: Res<TerrainMap>,
) {
    if stream.timer.tick(time.delta()).just_finished() {
        if stream.files.is_empty() { return; }

        // Rain on multiple files per tick
        for _ in 0..5 {
            let (x, y) = stream.files[stream.current_index].grid_pos;
            stream.current_index = (stream.current_index + 1) % stream.files.len();

            if x < terrain.width && y < terrain.height {
                // Spawn droplets around this point
                for _ in 0..20 {
                    let mut rng = rand::thread_rng();
                    use rand::Rng; // Import Rng trait locally
                    let offset = Vec2::new(
                        (rng.gen::<f32>() - 0.5) * 4.0,
                        (rng.gen::<f32>() - 0.5) * 4.0,
                    );
                    let pos = Vec2::new(x as f32, y as f32) + offset;
                    erosion_state.droplets.push(Droplet::new(pos));
                }
            }
        }
    }
}

fn camera_control(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Transform, With<Camera>>,
    time: Res<Time>,
    terrain: Option<Res<TerrainMap>>,
) {
    let mut transform = query.single_mut();
    let speed = 30.0 * time.delta_seconds();

    if input.pressed(KeyCode::KeyW) {
        transform.translation.z -= speed;
    }
    if input.pressed(KeyCode::KeyS) {
        transform.translation.z += speed;
    }
    if input.pressed(KeyCode::KeyA) {
        transform.translation.x -= speed;
    }
    if input.pressed(KeyCode::KeyD) {
        transform.translation.x += speed;
    }
    if input.pressed(KeyCode::KeyQ) {
        transform.translation.y += speed;
    }
    if input.pressed(KeyCode::KeyE) {
        transform.translation.y -= speed;
    }

    // Look at center if holding Space
    if input.pressed(KeyCode::Space) {
        if let Some(terrain) = terrain {
             let center = Vec3::new(terrain.width as f32 * 0.5, 0.0, terrain.height as f32 * 0.5);
             transform.look_at(center, Vec3::Y);
        }
    }
}
