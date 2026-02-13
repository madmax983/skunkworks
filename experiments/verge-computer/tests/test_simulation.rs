use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use verge_computer::{
    mechanism,
    EscapeWheel, VergeComputerPlugin,
};

#[test]
fn test_escapement_moves() {
    let mut app = App::new();

    app.add_plugins(MinimalPlugins);
    app.add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0).in_schedule(Update));

    // We need to add the plugin but avoid ViewPlugin which adds rendering assets/systems that might panic headless?
    // ViewPlugin adds ShapePlugin. ShapePlugin adds assets.
    // MinimalPlugins doesn't have AssetPlugin?
    // If ViewPlugin requires AssetPlugin, it will panic.
    // VergeComputerPlugin adds events and resources. Safe.
    app.add_plugins(VergeComputerPlugin);

    app.add_systems(Startup, setup_simulation);

    // Run for a bit
    for _ in 0..100 {
        app.update();
    }

    // Query EscapeWheel
    let world = app.world_mut();
    let mut query = world.query::<&EscapeWheel>();
    let wheel_comp = query.single(world);

    println!("Cumulative Angle: {}", wheel_comp.cumulative_angle);
    // It should have moved (negative angle due to torque).
    assert!(wheel_comp.cumulative_angle.abs() > 0.0001, "Wheel should move under torque");
}

fn setup_simulation(mut commands: Commands) {
    let wheel_pos = Vec2::new(0.0, 0.0);
    let teeth = 30;
    let radius = 10.0;
    let mass_density = 5.0;

    let wheel = mechanism::spawn_gear(&mut commands, wheel_pos, teeth, radius, mass_density);

    let ground = commands.spawn(RigidBody::Fixed).id();

    commands.entity(wheel)
        .insert(ExternalForce {
            torque: -5000.0,
            ..default()
        })
        .insert(ImpulseJoint::new(
             ground,
             RevoluteJointBuilder::new().local_anchor1(wheel_pos).local_anchor2(Vec2::ZERO)
        ));

    // Anchor setup
    let span_teeth = 7.5;
    let pitch_angle = 2.0 * std::f32::consts::PI / (teeth as f32);
    let half_span_angle = (span_teeth * pitch_angle) / 2.0;
    let pivot_dist = radius / half_span_angle.cos();
    let anchor_pos = Vec2::new(0.0, pivot_dist);

    let anchor = mechanism::spawn_anchor(&mut commands, anchor_pos, radius, teeth, span_teeth);
    commands.entity(anchor).insert(ImpulseJoint::new(
             ground,
             RevoluteJointBuilder::new().local_anchor1(anchor_pos).local_anchor2(Vec2::ZERO).limits([-0.5, 0.5])
    ));
}
