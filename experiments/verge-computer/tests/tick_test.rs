use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use verge_computer::{
    cpu,
    cpu::{Instruction, Program, TickEvent},
    mechanism, EscapeWheel, VergeComputerPlugin,
};

#[test]
fn test_tick_mechanism_and_cpu() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins)
        .add_plugins(TransformPlugin)
        .add_plugins(HierarchyPlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0).in_schedule(Update))
        .add_plugins(VergeComputerPlugin);

    app.add_systems(Startup, setup_test_scene);
    app.add_systems(Update, apply_torque_test);

    app.insert_resource(TickCounter(0));
    app.add_systems(Update, count_ticks);

    println!("Starting simulation...");
    for i in 0..500 {
        app.update();
        if i % 100 == 0 {
            let ticks = app.world().resource::<TickCounter>().0;
            println!("Frame {}: Ticks: {}", i, ticks);
        }
    }

    let final_ticks = app.world().resource::<TickCounter>().0;
    println!("Final ticks: {}", final_ticks);

    assert!(final_ticks > 0, "Mechanism failed to tick!");

    // Check CPU State
    let mut cpu_query = app.world_mut().query::<&cpu::CpuState>();
    let cpu_state = cpu_query.single(app.world());

    println!("CPU Registers: {:?}", cpu_state.registers);
    println!("CPU PC: {}", cpu_state.pc);
    println!("CPU Instructions: {}", cpu_state.instructions);

    // Verify Fibonacci sequence execution
    // Registers[1] should hold a Fibonacci number (1, 1, 2, 3, 5, 8...)
    // It starts at 0.
    assert!(
        cpu_state.registers[1] >= 1,
        "R1 should be at least 1 after execution"
    );
    assert!(
        cpu_state.instructions > 0,
        "CPU should have executed instructions"
    );
}

#[derive(Resource)]
struct TickCounter(usize);

fn count_ticks(mut counter: ResMut<TickCounter>, mut events: EventReader<TickEvent>) {
    for _ in events.read() {
        counter.0 += 1;
    }
}

fn setup_test_scene(mut commands: Commands) {
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

    // CPU State
    commands.spawn(cpu::CpuState::default());

    // Program (Fibonacci)
    // 0: LOAD R0, 0
    // 1: LOAD R1, 1
    // Loop:
    // 2: LOAD R2, 0
    // 3: ADD R2, R0
    // 4: ADD R2, R1
    // 5: MOV R0, R1
    // 6: MOV R1, R2
    // 7: JMP 2
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

#[derive(Component)]
struct MainSpring;

fn apply_torque_test(mut query: Query<&mut ExternalForce, With<MainSpring>>) {
    for mut force in &mut query {
        force.torque = -150.0;
    }
}
