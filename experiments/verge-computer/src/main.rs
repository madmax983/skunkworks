use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;
use verge_computer::{
    cpu::{self, Instruction, Program},
    mechanism,
    view::ViewPlugin,
    VergeComputerPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        // .add_plugins(RapierDebugRenderPlugin::default())
        .add_plugins(VergeComputerPlugin)
        .add_plugins(ViewPlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (apply_torque, update_cpu_text))
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
    let teeth = 30;
    let radius = 10.0;
    let mass_density = 5.0; // Heavy wheel

    let wheel = mechanism::spawn_gear(&mut commands, wheel_pos, teeth, radius, mass_density);

    commands
        .entity(wheel)
        .insert(MainSpring)
        .insert(ExternalForce::default())
        .insert(ImpulseJoint::new(
            ground,
            RevoluteJointBuilder::new()
                .local_anchor1(wheel_pos)
                .local_anchor2(Vec2::ZERO),
        ));

    // 2. Anchor
    let span_teeth = 7.5;
    let pitch_angle = 2.0 * PI / (teeth as f32);
    let half_span_angle = (span_teeth * pitch_angle) / 2.0;
    // Calculate pivot distance for tangent pallets
    let pivot_dist = radius / half_span_angle.cos();

    let anchor_pos = Vec2::new(0.0, pivot_dist);
    let anchor = mechanism::spawn_anchor(&mut commands, anchor_pos, radius, teeth, span_teeth);

    commands.entity(anchor).insert(ImpulseJoint::new(
        ground,
        RevoluteJointBuilder::new()
            .local_anchor1(anchor_pos)
            .local_anchor2(Vec2::ZERO)
            .limits([-0.5, 0.5]),
    ));

    // 3. CPU Visualization
    commands.spawn((
        cpu::CpuState::default(),
        TextBundle::from_section(
            "CPU State: HALTED",
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

    // 4. Program (Fibonacci)
    commands.insert_resource(Program(vec![
        Instruction::Load(0, 0),
        Instruction::Load(1, 1),
        Instruction::Load(2, 0),
        Instruction::Add(2, 0),
        Instruction::Add(2, 1),
        Instruction::Mov(0, 1),
        Instruction::Mov(1, 2),
        Instruction::Jmp(2),
    ]));
}

fn apply_torque(mut query: Query<&mut ExternalForce, With<MainSpring>>) {
    for mut force in &mut query {
        force.torque = -5000.0; // Strong spring for 30 teeth
    }
}

fn update_cpu_text(
    cpu_query: Query<&cpu::CpuState>,
    mut text_query: Query<&mut Text, With<cpu::CpuState>>,
) {
    if let Ok(state) = cpu_query.get_single() {
        if let Ok(mut text) = text_query.get_single_mut() {
            text.sections[0].value = format!(
                "PC: {}\nPhase: {:?}\nInstr: {}\nRegisters: {:?}",
                state.pc, state.phase, state.instructions, state.registers
            );
        }
    }
}
