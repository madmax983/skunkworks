use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use clockwork_concerto::{mechanism, cpu, audio, view};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Clockwork Concerto".into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(20.0)) // Zoom out a bit
        // .add_plugins(RapierDebugRenderPlugin::default()) // Enable for debugging physics
        .add_plugins(ShapePlugin)
        .add_plugins((
            mechanism::MechanismPlugin,
            cpu::CpuPlugin,
            audio::AudioPlugin,
            view::ViewPlugin,
        ))
        .add_systems(Startup, setup)
        .add_systems(Update, handle_torque)
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Ground for pinning if needed (currently using LockedAxes)

    // Spawn Escape Wheel
    let wheel_pos = Vec2::new(0.0, 0.0);
    let teeth = 12;
    let radius = 8.0;
    let density = 2.0;

    mechanism::spawn_gear(&mut commands, wheel_pos, teeth, radius, density);

    // Spawn Anchor
    // Position needs tuning.
    // For radius 8, teeth 12, gap between teeth is large.
    // Anchor spans roughly 90 degrees?
    // Let's put anchor at Y=12.0
    mechanism::spawn_anchor(&mut commands, Vec2::new(0.0, 12.0));

    // Spawn CPU
    commands.spawn(cpu::CpuState::default());

    // Load Program (Music Box)
    // C Major Scale with rhythm
    let notes = vec![60, 62, 64, 65, 67, 69, 71, 72];
    let mut program = Vec::new();

    for note in notes {
        program.push(cpu::Instruction::Note(note));
        // Add wait instructions (NOPs)
        for _ in 0..4 {
             program.push(cpu::Instruction::Load(0, 0)); // NOP effectively
        }
    }
    program.push(cpu::Instruction::Jmp(0)); // Loop

    commands.insert_resource(cpu::Program(program));
}

fn handle_torque(
    input: Res<ButtonInput<KeyCode>>,
    mut query: Query<&mut ExternalForce, With<mechanism::EscapeWheel>>,
) {
    let torque_mag = 50000.0; // High torque due to high mass/inertia
    for mut force in &mut query {
        if input.pressed(KeyCode::ArrowUp) {
            force.torque = -torque_mag; // Clockwise
        } else if input.pressed(KeyCode::ArrowDown) {
            force.torque = torque_mag; // CCW (Brake)
        } else {
            force.torque = -10000.0; // Base torque
        }
    }
}
