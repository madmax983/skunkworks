use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use celestial_cipher::orrery::{Planet, MainShaft, PlanetArm, spawn_orrery};
use celestial_cipher::cipher::{generate_key, encrypt_decrypt};

#[derive(Resource)]
struct CipherState {
    original_message: String,
    current_key: [u8; 32],
}

#[derive(Component)]
struct KeyText;

#[derive(Component)]
struct MessageText;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Celestial Cipher".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        //.add_plugins(RapierDebugRenderPlugin::default()) // Optional for debug
        .add_plugins(ShapePlugin)
        .insert_resource(CipherState {
            original_message: "THE PLANETS ALIGN FOR TRUTH".to_string(),
            current_key: [0; 32],
        })
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (setup_camera, setup_ui, spawn_orrery))
        .add_systems(Update, (control_input, update_cipher_ui))
        .run();
}

fn setup_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
}

fn setup_ui(mut commands: Commands) {
    commands.spawn(NodeBundle {
        style: Style {
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            flex_direction: FlexDirection::Column,
            justify_content: JustifyContent::SpaceBetween,
            padding: UiRect::all(Val::Px(20.0)),
            ..default()
        },
        ..default()
    }).with_children(|parent| {
        // Header
        parent.spawn(TextBundle::from_section(
            "Celestial Cipher Mechanism",
            TextStyle {
                font_size: 30.0,
                color: Color::WHITE,
                ..default()
            },
        ));

        // Info Panel
        parent.spawn(NodeBundle {
            style: Style {
                flex_direction: FlexDirection::Column,
                ..default()
            },
            ..default()
        }).with_children(|panel| {
            panel.spawn((
                TextBundle::from_section(
                    "Key: ...",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::srgb(1.0, 0.84, 0.0),
                        ..default()
                    },
                ),
                KeyText,
            ));

            panel.spawn((
                TextBundle::from_section(
                    "Decrypted: ...",
                    TextStyle {
                        font_size: 20.0,
                        color: Color::srgb(0.0, 1.0, 0.0),
                        ..default()
                    },
                ),
                MessageText,
            ));
        });

        // Controls
        parent.spawn(TextBundle::from_section(
            "Controls: Left/Right Arrow to Wind Time",
            TextStyle {
                font_size: 15.0,
                color: Color::srgb(0.5, 0.5, 0.5),
                ..default()
            },
        ));
    });
}

fn control_input(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut ExternalImpulse, With<MainShaft>>,
) {
    for mut impulse in query.iter_mut() {
        let torque = 1000000.0; // Massive torque for heavy gears
        if input.pressed(KeyCode::ArrowRight) {
            impulse.torque_impulse = -torque;
        } else if input.pressed(KeyCode::ArrowLeft) {
            impulse.torque_impulse = torque;
        } else {
            impulse.torque_impulse = 0.0;
        }
    }
}

fn update_cipher_ui(
    mut cipher_state: ResMut<CipherState>,
    planet_query: Query<(&Transform, &Planet), With<PlanetArm>>,
    mut key_text_query: Query<&mut Text, (With<KeyText>, Without<MessageText>)>,
    mut msg_text_query: Query<&mut Text, (With<MessageText>, Without<KeyText>)>,
) {
    // 1. Collect Angles
    // Sort planets by distance/period to ensure consistent key order?
    // We rely on stable sort or explicit order.
    // Query iteration order is not guaranteed.
    // We should collect into a vector and sort by Planet Name or Period.

    let mut planets: Vec<(String, f32)> = Vec::new();
    for (transform, planet) in planet_query.iter() {
        let angle = transform.rotation.to_euler(EulerRot::XYZ).2; // Z-rotation
        planets.push((planet.name.to_string(), angle));
    }

    // Sort by name to be deterministic
    planets.sort_by(|a, b| a.0.cmp(&b.0));

    let angles: Vec<f32> = planets.iter().map(|(_, a)| *a).collect();

    if angles.is_empty() { return; }

    // 2. Generate Key
    let key = generate_key(&angles);
    cipher_state.current_key = key;

    // 3. Encrypt/Decrypt
    // For demo, we assume the "Original Message" is the plaintext we WANT to hide.
    // And we show the "Result" of encrypting/decrypting it with current key.
    // Wait, usually we want to find the key that decrypts a ciphertext.
    // Let's assume there is a HIDDEN Ciphertext (encrypted with a specific key).
    // And we try to decrypt it.

    // Target Configuration: Angles = 0.0 (Alignment).
    // Key at 0.0...
    let target_key = generate_key(&vec![0.0; angles.len()]);
    let hidden_ciphertext = encrypt_decrypt(cipher_state.original_message.as_bytes(), &target_key);

    // Try to decrypt with CURRENT key
    let decrypted_bytes = encrypt_decrypt(&hidden_ciphertext, &key);
    let decrypted_string = String::from_utf8(decrypted_bytes).unwrap_or("???".to_string());

    // 4. Update UI
    for mut text in key_text_query.iter_mut() {
        text.sections[0].value = format!("Key: {}", hex::encode(&key[0..8])); // Show first 8 bytes
    }

    for mut text in msg_text_query.iter_mut() {
        // If key is close (or exact), show text.
        // Since we quantize, exact match is possible.
        text.sections[0].value = format!("Decrypted: {}", decrypted_string);

        if key == target_key {
            text.sections[0].style.color = Color::srgb(0.0, 1.0, 0.0);
        } else {
            text.sections[0].style.color = Color::srgb(1.0, 0.0, 0.0);
        }
    }
}
