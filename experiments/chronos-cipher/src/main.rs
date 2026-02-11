use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::f32::consts::PI;

mod mechanism;
mod cipher;
mod view;

use mechanism::*;
use cipher::*;
use view::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Chronos Cipher ⚛️".into(),
                resolution: (1200.0, 800.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(20.0))
        // .add_plugins(RapierDebugRenderPlugin::default()) // Debug physics if needed
        .add_plugins(ShapePlugin)
        .add_plugins(ViewPlugin)
        .init_resource::<CipherState>()
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (read_cipher_system, update_ui_system))
        .run();
}

#[derive(Component)]
struct CipherText;

fn setup_scene(mut commands: Commands) {
    // Camera
    commands.spawn(Camera2dBundle::default());

    // UI
    commands.spawn(NodeBundle {
        style: Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        parent.spawn((
            TextBundle::from_section(
                "Cipher Stream: ",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ),
            CipherText,
        ));
    });

    // --- Mechanism ---

    // Center of screen roughly
    let escapement_pos = Vec2::new(-200.0, 100.0);

    // Spawn Escapement (The ticking heart)
    spawn_escapement(&mut commands, escapement_pos);

    // Helper to calculate position for next gear
    // Spawns a chain of prime-numbered gears
    let mut current_pos = escapement_pos;
    let mut current_teeth = 15; // Escape Wheel
    let mut angle_dir = -PI / 2.0; // Start going down

    let gear_sequence = [30, 23, 19, 17, 13, 7]; // Primes for max period

    for &next_teeth in &gear_sequence {
        // Calculate module-based radii
        // Defined in mechanism.rs as const DEFAULT_MODULE = 2.0
        let module = 2.0;
        let r_curr = (current_teeth as f32) * module / 2.0;
        let r_next = (next_teeth as f32) * module / 2.0;
        let dist = r_curr + r_next + 1.0; // slight gap

        let next_pos = current_pos + Vec2::new(dist * angle_dir.cos(), dist * angle_dir.sin());

        spawn_gear(&mut commands, next_pos, next_teeth, 1.0);

        // Update for next iteration
        current_pos = next_pos;
        current_teeth = next_teeth;
        angle_dir += PI / 3.0; // Snake around
    }
}

fn update_ui_system(
    cipher: Res<CipherState>,
    mut query: Query<&mut Text, With<CipherText>>,
) {
    for mut text in &mut query {
        let stream_str: String = cipher.key_stream.iter()
            .map(|b| format!("{:02X} ", b))
            .collect();
        text.sections[0].value = format!("Cipher Stream: {}", stream_str);
    }
}
