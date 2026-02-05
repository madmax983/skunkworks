use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use clockwork_chimera::{
    cpu::ChimeraState, mechanism, view::ViewPlugin, ClockworkChimeraPlugin, EscapeWheel,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        // .add_plugins(RapierDebugRenderPlugin::default()) // Disabled for nicer visuals
        .add_plugins(ClockworkChimeraPlugin)
        .add_plugins(ViewPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (apply_torque, update_chimera_text))
        .run();
}

#[derive(Component)]
struct MainSpring;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Ground
    let ground = commands
        .spawn((TransformBundle::default(), RigidBody::Fixed))
        .id();

    // 1. Escape Wheel
    let wheel_pos = Vec2::new(0.0, 0.0);
    let teeth = 12;
    let radius = 3.0;

    let wheel = mechanism::spawn_gear(&mut commands, wheel_pos, teeth, radius, 0.5);

    commands
        .entity(wheel)
        .insert(EscapeWheel {
            last_angle: 0.0,
            teeth,
            cumulative_angle: 0.0,
        })
        .insert(MainSpring)
        .insert(ExternalForce::default())
        .insert(ImpulseJoint::new(
            ground,
            RevoluteJointBuilder::new()
                .local_anchor1(wheel_pos)
                .local_anchor2(Vec2::ZERO),
        ));

    // 2. Anchor
    let anchor_pos = Vec2::new(0.0, 5.0);
    let anchor = mechanism::spawn_anchor(&mut commands, anchor_pos);

    commands.entity(anchor).insert(ImpulseJoint::new(
        ground,
        RevoluteJointBuilder::new()
            .local_anchor1(anchor_pos)
            .local_anchor2(Vec2::ZERO),
    ));

    // 3. Chimera VM Visualization
    commands.spawn((
        ChimeraState::default(),
        TextBundle::from_section(
            "Chimera State: INCUBATING",
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
    ));
}

fn apply_torque(mut query: Query<&mut ExternalForce, With<MainSpring>>) {
    for mut force in &mut query {
        force.torque = -150.0;
    }
}

fn update_chimera_text(
    query: Query<&ChimeraState>,
    mut text_query: Query<&mut Text, With<ChimeraState>>,
) {
    if let Ok(state) = query.get_single() {
        if let Ok(mut text) = text_query.get_single_mut() {
            let stack_preview: Vec<String> = state
                .vm
                .stack
                .iter()
                .rev()
                .take(5)
                .map(|v| v.to_string())
                .collect();

            text.sections[0].value = format!(
                "Energy: {}\nIP: {:?}\nStack (Top 5): {:?}",
                state.vm.energy, state.vm.ip, stack_preview
            );

            if state.vm.halted {
                text.sections[0].value.push_str("\n[HALTED]");
            }
        }
    }
}
