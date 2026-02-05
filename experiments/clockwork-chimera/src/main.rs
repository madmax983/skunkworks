use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use chimera_lang::{
    ast::{Dna, Gene, Helix, Nucleotide, Strand},
    opcode::OpCode,
    vm::ChimeraVM,
};

mod mechanism;
mod view;
mod vm_bridge;

use mechanism::{spawn_anchor, spawn_gear, EscapeWheel, MechanismPlugin};
use view::ViewPlugin;
use vm_bridge::{ChimeraComponent, TickEvent, VmBridgePlugin};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        .add_plugins(MechanismPlugin)
        .add_plugins(ViewPlugin)
        .add_plugins(VmBridgePlugin)
        .add_systems(Startup, setup)
        .add_systems(Update, (apply_torque, detect_tick_system))
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

    let wheel = spawn_gear(&mut commands, wheel_pos, teeth, radius, 0.5);

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
    let anchor = spawn_anchor(&mut commands, anchor_pos);

    commands.entity(anchor).insert(ImpulseJoint::new(
        ground,
        RevoluteJointBuilder::new()
            .local_anchor1(anchor_pos)
            .local_anchor2(Vec2::ZERO),
    ));

    // 3. Chimera VM (The "Brain" of the clock)
    let dna = create_fibonacci_dna();
    let vm = ChimeraVM::new(dna);

    commands.spawn(ChimeraComponent { vm });
}

fn apply_torque(mut query: Query<&mut ExternalForce, With<MainSpring>>) {
    for mut force in &mut query {
        force.torque = -150.0;
    }
}

fn detect_tick_system(
    mut events: EventWriter<TickEvent>,
    mut query: Query<(&Transform, &mut EscapeWheel)>,
) {
    for (transform, mut wheel) in &mut query {
        let angle = transform.rotation.to_euler(EulerRot::XYZ).2; // Z rotation

        // Handle wrapping -PI to PI
        let mut delta = angle - wheel.last_angle;

        if delta > std::f32::consts::PI {
            delta -= 2.0 * std::f32::consts::PI;
        } else if delta < -std::f32::consts::PI {
            delta += 2.0 * std::f32::consts::PI;
        }

        wheel.last_angle = angle;
        wheel.cumulative_angle += delta;

        // Check if we passed a tooth
        let tooth_angle = 2.0 * std::f32::consts::PI / (wheel.teeth as f32);

        if wheel.cumulative_angle.abs() >= tooth_angle {
            // Reset by one tooth worth (keeping the remainder)
            if wheel.cumulative_angle < 0.0 {
                wheel.cumulative_angle += tooth_angle;
            } else {
                wheel.cumulative_angle -= tooth_angle;
            }
            events.send(TickEvent);
        }
    }
}

fn create_fibonacci_dna() -> Dna {
    // Strand 0: Init
    // [ push(1) push(0) push(0) g_write(), push(1) push(0) push(1) g_write(), jump(1) ]
    let init_genes = vec![
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::GWrite,
            args: vec![],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::GWrite,
            args: vec![],
        },
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(1)],
        },
    ];

    // Strand 1: Loop
    let loop_genes = vec![
        // Read a (0,0)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::GRead,
            args: vec![],
        },
        // Read b (0,1)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::GRead,
            args: vec![],
        },
        // Add
        Gene {
            op: OpCode::Add,
            args: vec![],
        },
        // Dup result (c)
        Gene {
            op: OpCode::Dup,
            args: vec![],
        },
        // Read b again (to move to a)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::GRead,
            args: vec![],
        },
        // Write b to (0,0)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::GWrite,
            args: vec![],
        },
        // Write c to (0,1)
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::GWrite,
            args: vec![],
        },
        // Read c to show on stack
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(0)],
        },
        Gene {
            op: OpCode::Push,
            args: vec![Nucleotide::Number(1)],
        },
        Gene {
            op: OpCode::GRead,
            args: vec![],
        },
        // Jump to start of loop (1, 0)
        Gene {
            op: OpCode::Jump,
            args: vec![Nucleotide::Number(1)],
        },
    ];

    Dna {
        helix: Helix {
            strands: vec![Strand { genes: init_genes }, Strand { genes: loop_genes }],
        },
    }
}
