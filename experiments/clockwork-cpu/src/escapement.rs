use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

use crate::components::*;
pub struct EscapementPlugin;

impl Plugin for EscapementPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_escapement);
    }
}

fn spawn_escapement(mut commands: Commands) {
    let escape_wheel_radius = 100.0;
    let escape_wheel_teeth = 12;

    // 1. Escape Wheel (The "Crown Wheel")
    // We need a sawtooth shape.
    // We'll generate a custom path for the collider.
    let wheel_pos = Vec2::new(0.0, 0.0);

    // Custom Sawtooth Path for visuals and physics
    // For physics we might want to approximate with a compound collider if possible, or a polyline.
    // Rapier supports convex decomposition, so a polygon is fine.

    let wheel_path = create_sawtooth_path(escape_wheel_radius, escape_wheel_teeth);

    // Visuals
    let wheel_entity = commands
        .spawn((
            ShapeBundle {
                path: wheel_path,
                spatial: SpatialBundle::from_transform(Transform::from_translation(
                    wheel_pos.extend(0.0),
                )),
                ..default()
            },
            Fill::color(Color::GOLD),
            Stroke::new(Color::BLACK, 2.0),
            Gear {
                teeth: escape_wheel_teeth,
                radius: escape_wheel_radius,
            },
            EscapeWheel,
            RigidBody::Dynamic,
            // We need a collider matching the shape.
            // For simplicity, we can use a Ball for mass, and then add the teeth as sensors or compounds?
            // No, we need actual collision.
            // Let's use a convex hull of the points? No, it's concave.
            // Rapier `Collider::polyline`? Or `Collider::compound`.
            // Let's try `Collider::convex_decomposition` (if available via helper) or just build it manually.
            // For simplicity in this experiment, I'll use a `Ball` for the core and small `Cuboid`s for teeth?
            // Or better: `Collider::polyline` is good for thin walls, but we want solid gears.
            // Let's use `Collider::from_bevy_mesh`? No, 2D.
            // Let's use `Collider::convex_hull` of the points? That will close the gaps.
            // We need the gaps.
            // I will just use a ball for now and add torque, and rely on visuals?
            // NO. "Physically accurate".
            // I will build the collider from the points.
        ))
        .id();

    // Create collider for the wheel
    // We'll approximate the wheel as a central disc + 12 triangular teeth
    let core_radius = escape_wheel_radius * 0.8;

    // Add torque
    commands.entity(wheel_entity).insert(ExternalForce {
        torque: -50000.0, // Constant torque driving the clock
        ..default()
    });
    commands.entity(wheel_entity).insert(Damping {
        angular_damping: 1.0,
        linear_damping: 0.0,
    });

    // Central Hub Collider
    commands.entity(wheel_entity).with_children(|parent| {
        parent.spawn(Collider::ball(core_radius));
    });

    // Add teeth colliders as children
    for i in 0..escape_wheel_teeth {
        let angle = i as f32 * (2.0 * PI / escape_wheel_teeth as f32);
        let tip_pos = Vec2::new(
            angle.cos() * escape_wheel_radius,
            angle.sin() * escape_wheel_radius,
        );

        let tooth = commands
            .spawn((
                TransformBundle::from_transform(Transform {
                    translation: tip_pos.extend(0.0),
                    rotation: Quat::from_rotation_z(angle),
                    ..default()
                }),
                Collider::cuboid(5.0, 5.0), // Simple tooth
            ))
            .id();
        commands.entity(wheel_entity).add_child(tooth);
    }

    // Create ground
    let ground = commands
        .spawn((
            TransformBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
            RigidBody::Fixed,
        ))
        .id();

    // Add Revolute Joint for Wheel (at 0,0)
    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::ZERO)
        .local_anchor2(wheel_pos);

    commands
        .entity(wheel_entity)
        .insert(ImpulseJoint::new(ground, joint));

    // 2. The Anchor (Verge)
    // Positioned above the wheel
    let anchor_pos = Vec2::new(0.0, escape_wheel_radius + 40.0);

    let anchor_entity = commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(anchor_pos.extend(0.1))),
            RigidBody::Dynamic,
            Verge,
            Damping {
                angular_damping: 0.5,
                linear_damping: 0.0,
            }, // Friction
        ))
        .id();

    // Visuals for Anchor (an inverted T shape)
    let anchor_shape = GeometryBuilder::build_as(&shapes::Rectangle {
        extents: Vec2::new(140.0, 10.0),
        origin: RectangleOrigin::Center,
    });

    let anchor_visual = commands
        .spawn((
            ShapeBundle {
                path: anchor_shape,
                ..default()
            },
            Fill::color(Color::SILVER),
            Stroke::new(Color::BLACK, 2.0),
        ))
        .id();
    commands.entity(anchor_entity).add_child(anchor_visual);

    // Pallets (The bits that hit the teeth)
    // Adjusted for 100 radius
    // Pallets need to be carefully placed to catch the teeth.
    let left_pallet_pos = Vec2::new(-40.0, -30.0);
    let left_pallet = commands
        .spawn((
            TransformBundle::from_transform(Transform::from_translation(
                left_pallet_pos.extend(0.0),
            )),
            Collider::cuboid(5.0, 20.0),
        ))
        .id();

    let right_pallet_pos = Vec2::new(40.0, -30.0);
    let right_pallet = commands
        .spawn((
            TransformBundle::from_transform(Transform::from_translation(
                right_pallet_pos.extend(0.0),
            )),
            Collider::cuboid(5.0, 20.0),
        ))
        .id();

    commands.entity(anchor_entity).add_child(left_pallet);
    commands.entity(anchor_entity).add_child(right_pallet);

    // Hinge for Anchor
    let anchor_joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::ZERO)
        .local_anchor2(anchor_pos);

    commands
        .entity(anchor_entity)
        .insert(ImpulseJoint::new(ground, anchor_joint));

    // Restoring force? A pendulum has gravity. A foliott has a hairspring (or just inertia + banking pins).
    // I'll add a slight restoring torque spring (hairspring simulation) or rely on the geometry (pendulum).
    // Let's add a `ExternalForce` or simple spring logic in a system?
    // Or just make it a pendulum by offsetting center of mass?
    // Rapier calculates CoM from colliders. If I add a heavy collider at the bottom, it acts like a pendulum.

    // Add a "Pendulum Bob" to the anchor
    let bob_pos = Vec2::new(0.0, 50.0); // Upwards (unstable) or Downwards?
                                        // Pendulums hang down.
                                        // But the anchor is above the wheel. So the pendulum should hang down *past* the wheel? Or go up?
                                        // Let's make it a Foliot (horizontal bar with weights).
                                        // Foliots don't have a natural period without a verge restoring force (from the tooth shape).
                                        // The tooth pushes the pallet out, the other pallet enters.
                                        // I need to tune the geometry carefully.

    // For this simulation, I will add a "Virtual Hairspring" (Stiffness) to the joint.
    commands.entity(anchor_entity).insert(ExternalForce {
        torque: 0.0,
        ..default()
    });

    // We can use a joint with stiffness?
    // RevoluteJoint doesn't have stiffness built-in in bevy_rapier usually, unless using motors.
    // I'll leave it free and see if the escapement geometry drives it (it should).
}

fn create_sawtooth_path(radius: f32, teeth: u32) -> Path {
    let mut path_builder = PathBuilder::new();
    let angle_step = 2.0 * PI / teeth as f32;

    for i in 0..teeth {
        let angle = i as f32 * angle_step;
        let next_angle = (i + 1) as f32 * angle_step;

        let r_inner = radius * 0.8;
        let r_outer = radius;

        // Sawtooth: ramp up, drop down
        // Point 1: Inner (start of tooth)
        let p1 = Vec2::new(angle.cos() * r_inner, angle.sin() * r_inner);
        // Point 2: Outer (tip of tooth) - slightly skewed
        let tip_angle = next_angle - 0.1;
        let p2 = Vec2::new(tip_angle.cos() * r_outer, tip_angle.sin() * r_outer);

        if i == 0 {
            path_builder.move_to(p1);
        } else {
            path_builder.line_to(p1);
        }
        path_builder.line_to(p2);
    }
    path_builder.close();
    path_builder.build()
}
