use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use verge_computer::{
    cpu::{self, Instruction, Program},
    mechanism,
    view::ViewPlugin,
    EscapeWheel, VergeComputerPlugin,
};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        // .add_plugins(RapierDebugRenderPlugin::default()) // Disabled for nicer visuals
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
        force.torque = -150.0;
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
