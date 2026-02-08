use bevy::prelude::*;
use std::path::Path;

mod lattice;
use lattice::Crystal;

fn main() {
    let args: Vec<String> = std::env::args().collect();
    let path_str = if args.len() > 1 { &args[1] } else { "." };
    let path = Path::new(path_str);

    println!("Building crystal from: {:?}", path);
    match Crystal::build_hopper_crystal(path) {
        Ok(crystal) => {
            println!("Crystal built with {} atoms.", crystal.atoms.len());
            App::new()
                .add_plugins(DefaultPlugins)
                .insert_resource(crystal)
                .add_systems(Startup, (setup, spawn_crystal))
                .add_systems(Update, rotate_camera)
                .run();
        }
        Err(e) => {
            eprintln!("Failed to build crystal: {}", e);
        }
    }
}

fn setup(mut commands: Commands, crystal: Res<Crystal>) {
    // Camera
    // Position it away from center of mass, looking at it
    let center = crystal.center_of_mass;
    let distance = (crystal.atoms.len() as f32).sqrt() * 2.0 + 20.0; // Dynamic distance based on size

    commands.spawn(Camera3dBundle {
        transform: Transform::from_translation(center + Vec3::new(distance, distance, distance))
            .looking_at(center, Vec3::Y),
        ..default()
    });

    // Lights
    commands.spawn(PointLightBundle {
        point_light: PointLight {
            intensity: 1500000.0, // High intensity for PBR
            shadows_enabled: true,
            range: 1000.0,
            ..default()
        },
        transform: Transform::from_translation(center + Vec3::new(0.0, 50.0, 0.0)),
        ..default()
    });

    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: 10000.0,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform::from_rotation(Quat::from_euler(EulerRot::XYZ, -std::f32::consts::FRAC_PI_4, -std::f32::consts::FRAC_PI_4, 0.0)),
        ..default()
    });
}

fn spawn_crystal(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    crystal: Res<Crystal>
) {
    let mesh_handle = meshes.add(Cuboid::new(0.9, 0.9, 0.9));

    // Pre-generate materials for common hues to save improved performance?
    // Nah, individual materials allow for unique gradients.

    for atom in &crystal.atoms {
        // Bismuth Iridescence Logic:
        // Interference colors depend on oxide layer thickness.
        // We simulate thickness using (Depth + Spiral Position).

        let base_hue = match (atom.miller_index.x, atom.miller_index.z) {
             (1, 0) => 330.0, // Pink
            (-1, 0) => 210.0, // Blue
             (0, 1) => 60.0,  // Gold
             (0, -1) => 120.0, // Green
             _ => 0.0,
        };

        // Modulate hue by depth to create gradients across the spiral
        let hue_shift = (atom.depth as f32 * 15.0) % 360.0;
        let final_hue = (base_hue + hue_shift) % 360.0;

        let material = materials.add(StandardMaterial {
            base_color: Color::hsl(final_hue, 0.9, 0.6),
            metallic: 0.9,
            perceptual_roughness: 0.1,
            ..default()
        });

        commands.spawn(PbrBundle {
            mesh: mesh_handle.clone(),
            material,
            transform: Transform::from_translation(atom.position.as_vec3()),
            ..default()
        });
    }
}

fn rotate_camera(
    time: Res<Time>,
    mut query: Query<&mut Transform, With<Camera>>,
    crystal: Res<Crystal>
) {
    let center = crystal.center_of_mass;
    for mut transform in &mut query {
        // Orbit around center
        let _distance = transform.translation.distance(center);
        let rotation_speed = 0.2;
        let angle = time.delta_seconds() * rotation_speed;

        // Rotate around Y axis at center
        let mut rel_pos = transform.translation - center;

        // Simple 2D rotation for X/Z
        let s = angle.sin();
        let c = angle.cos();
        let new_x = rel_pos.x * c - rel_pos.z * s;
        let new_z = rel_pos.x * s + rel_pos.z * c;

        rel_pos.x = new_x;
        rel_pos.z = new_z;

        transform.translation = center + rel_pos;
        transform.look_at(center, Vec3::Y);
    }
}
