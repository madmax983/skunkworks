use bevy::prelude::*;

mod audio;
mod camera;
mod scanner;
mod interaction;

use audio::{AudioRecorder, AudioString};
use camera::{fly_camera_system, grab_mouse, FlyCam};
use scanner::{scan_directory, get_frequency_from_size, get_color_from_ext, CodeString};
use interaction::interaction_system;

#[derive(Resource)]
struct ScanConfig {
    root: String,
}

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Holographic Strings".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .insert_resource(ScanConfig { root: ".".to_string() })
        .insert_resource(AudioRecorder { recording: true, ..default() })
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (
            spawn_strings_delayed,
            fly_camera_system,
            grab_mouse,
            rotate_strings,
            audio::advance_audio_physics,
            interaction_system,
            save_on_exit,
        ))
        .run();
}

fn save_on_exit(mut events: EventReader<AppExit>, recorder: Res<AudioRecorder>) {
    for _ in events.read() {
        if let Err(e) = audio::save_recording(&recorder) {
            eprintln!("Failed to save audio: {}", e);
        }
    }
}

fn setup_scene(mut commands: Commands, mut meshes: ResMut<Assets<Mesh>>, mut materials: ResMut<Assets<StandardMaterial>>) {
    // Camera
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 2.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        FlyCam::default(),
    ));

    // Ground
    commands.spawn(PbrBundle {
        mesh: meshes.add(Mesh::from(Cuboid::new(100.0, 0.1, 100.0))),
        material: materials.add(Color::srgb(0.1, 0.1, 0.1)),
        ..default()
    });

    // Light
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1_500_000.0,
            range: 100.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_xyz(4.0, 8.0, 4.0),
        ..default()
    });
}

// Run once
fn spawn_strings_delayed(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    config: Res<ScanConfig>,
    mut spawned: Local<bool>,
) {
    if *spawned { return; }
    *spawned = true;

    if let Ok(files) = scan_directory(&config.root) {
        let mut angle = 0.0f32;
        let mut radius = 2.0f32;

        for file in files.iter() {
            // Spiral layout
            let x = radius * angle.cos();
            let z = radius * angle.sin();

            angle += 0.5; // radians
            radius += 0.05;

            let color = get_color_from_ext(&file.extension);
            let freq = get_frequency_from_size(file.size);

            // Calculate height based on size (log scale)
            let height = (file.size as f32 + 1.0).log2().max(1.0) * 0.5;

            commands.spawn((
                PbrBundle {
                    mesh: meshes.add(Mesh::from(Cylinder::new(0.05, height * 2.0))),
                    material: materials.add(StandardMaterial {
                        base_color: color,
                        emissive: LinearRgba::from(color) * 2.0, // Glow
                        ..default()
                    }),
                    transform: Transform::from_xyz(x, height, z),
                    ..default()
                },
                CodeString {
                    path: file.path.clone(),
                    size: file.size,
                },
                AudioString::new(freq, 0.995), // High damping for musicality
            ));
        }
    }
}

fn rotate_strings() {
    // Placeholder for animation
}
