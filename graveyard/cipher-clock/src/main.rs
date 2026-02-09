use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

mod mechanism;
use mechanism::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(ShapePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, input_control)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Spawn Rotor
    let rotor_pos = Vec2::ZERO;
    let radius = 100.0;
    let teeth = 26; // 26 letters!
    let rotor = spawn_rotor(&mut commands, rotor_pos, radius, teeth);

    // Add Velocity component explicitly if not added by Rapier (it usually is added when needed, but good to be safe)
    commands.entity(rotor).insert(Velocity::default());

    // Spawn Holding Pawl (Top)
    // Pivot needs to be placed such that the tip falls into the teeth.
    // The rotor has radius 100.
    // We want the pawl tip to rest on top.
    // Let's place the pivot to the left and up.
    let pawl_len = 100.0;
    let pivot_pos = Vec2::new(-pawl_len + 20.0, radius + 40.0);
    // This places the pivot. The pawl extends right by `pawl_len`.
    // Tip X ~ 20.0. Tip Y ~ radius + 40.0.
    // It will fall down under gravity onto the gear.

    spawn_pawl(&mut commands, pivot_pos, pawl_len);

    // Instructions
    commands.spawn(
        TextBundle::from_section(
            "Press SPACE to Spin Rotor (CCW)\nObserve the Pawl clicking.",
            TextStyle {
                font_size: 20.0,
                color: Color::WHITE,
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

fn input_control(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut Velocity, With<Rotor>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        for mut vel in query.iter_mut() {
            // Spin CCW
            vel.angvel += 5.0;
        }
    }
}
