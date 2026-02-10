use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use logic_gears::mechanism::{spawn_differential_adder, Rack, OutputIndicator};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Logic Gears: Mechanical Adder".into(),
                resolution: (800., 600.).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_systems(Startup, setup)
        .add_systems(Update, (control_inputs, print_output))
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());
    spawn_differential_adder(&mut commands, Vec2::ZERO);
    commands.spawn(
        TextBundle::from_section(
            "Controls:\nLeft/Right Arrows: Move Input A\nA/D Keys: Move Input B\nObserve the Center Pinion (Output)",
            TextStyle {
                font_size: 20.0,
                color: Color::srgb(1.0, 1.0, 1.0),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        }),
    );
}

fn control_inputs(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut Transform, &Rack)>,
    time: Res<Time>,
) {
    let speed = 100.0 * time.delta_seconds();

    for (mut transform, _rack) in query.iter_mut() {
        if transform.translation.x < 0.0 {
            if input.pressed(KeyCode::ArrowUp) { transform.translation.y += speed; }
            if input.pressed(KeyCode::ArrowDown) { transform.translation.y -= speed; }
        } else {
            if input.pressed(KeyCode::KeyW) { transform.translation.y += speed; }
            if input.pressed(KeyCode::KeyS) { transform.translation.y -= speed; }
        }
    }
}

fn print_output(
    query: Query<(&Transform, &OutputIndicator)>,
) {
    for (_transform, _) in query.iter() {
        // info!("Output Position: {:.2}", transform.translation.y);
    }
}
