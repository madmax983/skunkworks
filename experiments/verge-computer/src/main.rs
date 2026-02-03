use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use verge_computer::{cpu, mechanism, EscapeWheel, VergeComputerPlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(VergeComputerPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (apply_torque, update_cpu_text))
        .run();
}

#[derive(Component)]
struct MainSpring;

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Ground (Fixed point for joints)
    let ground = commands
        .spawn((TransformBundle::default(), RigidBody::Fixed))
        .id();

    // 1. Escape Wheel
    let wheel_pos = Vec2::new(0.0, 0.0);
    let teeth = 12;
    let radius = 3.0;

    let wheel = mechanism::spawn_gear(&mut commands, wheel_pos, teeth, radius, 0.5);

    // Add components
    commands
        .entity(wheel)
        .insert(EscapeWheel {
            last_angle: 0.0,
            teeth,
            cumulative_angle: 0.0,
        })
        .insert(MainSpring) // We will apply torque to this
        .insert(ExternalForce::default())
        .insert(ImpulseJoint::new(
            ground,
            RevoluteJointBuilder::new()
                .local_anchor1(wheel_pos)
                .local_anchor2(Vec2::ZERO), // Joint relative to body 2 (wheel) center
        ));

    // 2. Anchor (Escapement)
    // Position it above the wheel.
    // The pallets need to interact with the teeth.
    // Distance depends on radius + tooth length.
    // Radius 3.0. Teeth stick out a bit? spawn_gear puts teeth centered at radius.
    // So outer radius is approx 3.0 + 0.5 (half height) = 3.5.
    // Anchor pivot should be around 5.0 units away?
    let anchor_pos = Vec2::new(0.0, 5.0);
    let anchor = mechanism::spawn_anchor(&mut commands, anchor_pos);

    commands.entity(anchor).insert(ImpulseJoint::new(
        ground,
        RevoluteJointBuilder::new()
            .local_anchor1(anchor_pos)
            .local_anchor2(Vec2::ZERO),
    ));

    // 3. CPU Visualization
    commands.spawn((
        cpu::CpuState::default(),
        // Visuals
        TextBundle::from_section(
            "CPU State: HALTED",
            TextStyle {
                font_size: 30.0,
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
    // Continuous torque to drive the clock
    for mut force in &mut query {
        force.torque = -150.0; // Clockwise
    }
}

fn update_cpu_text(
    cpu_query: Query<&cpu::CpuState>,
    mut text_query: Query<&mut Text, With<cpu::CpuState>>,
) {
    if let Ok(state) = cpu_query.get_single() {
        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = format!(
                "PC: {}\nPhase: {:?}\nInstr: {}",
                state.pc, state.phase, state.instructions
            );
        }
    }
}

// We need to add ExternalTorque component to the wheel, otherwise query fails?
// Or we can query RigidBody and apply force.
// Better: Add ExternalTorque component at spawn.
// I'll update spawn logic in setup or add it here if it's not present.
// Actually, `spawn_gear` does not add `ExternalTorque`.
// I should add it in setup.
