use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod choreographer;
mod code_graph;
mod components;
mod ik;

use choreographer::*;
use code_graph::*;
use components::*;
use ik::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_plugins(IKPlugin)
        .add_plugins(CodeGraphPlugin)
        .add_plugins(ChoreographerPlugin)
        .add_systems(Startup, setup_scene)
        .run();
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Spawn IK Target (Visualized as small red circle)
    let target_entity = commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: 5.0,
                    center: Vec2::ZERO,
                }),
                spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
                    100.0, 100.0, 10.0,
                ))),
                ..default()
            },
            Fill::color(Color::RED),
            IKTarget,
        ))
        .id();

    // Spawn Creature Chain
    // Root at (0, 0)
    let root_pos = Vec3::new(0.0, -200.0, 0.0);
    let root = commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: 8.0,
                    center: Vec2::ZERO,
                }),
                spatial: SpatialBundle::from_transform(Transform::from_translation(root_pos)),
                ..default()
            },
            Fill::color(Color::WHITE),
            ChainRoot,
        ))
        .id();

    // Joints
    let mut current_parent = root;
    let mut joints = Vec::new();
    let num_segments = 5;
    let segment_length = 60.0;

    for i in 0..num_segments {
        let bone_shape = shapes::Line(Vec2::ZERO, Vec2::new(segment_length, 0.0));

        let transform = if i == 0 {
            Transform::from_translation(Vec3::ZERO)
        } else {
            Transform::from_translation(Vec3::new(segment_length, 0.0, 0.0))
        };

        let joint = commands
            .spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&bone_shape),
                    spatial: SpatialBundle::from_transform(transform),
                    ..default()
                },
                Stroke::new(Color::WHITE, 5.0),
                Joint {
                    angle: 0.0,
                    min_angle: -std::f32::consts::PI,
                    max_angle: std::f32::consts::PI,
                },
                Bone {
                    length: segment_length,
                },
            ))
            .id();

        commands.entity(current_parent).add_child(joint);
        current_parent = joint;
        joints.push(joint);
    }

    // Effector (Visualized as a claw or hand)
    let effector = commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::RegularPolygon {
                    // Triangle claw
                    sides: 3,
                    feature: RegularPolygonFeature::Radius(8.0),
                    ..default()
                }),
                spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
                    segment_length,
                    0.0,
                    0.0,
                ))),
                ..default()
            },
            Fill::color(Color::CYAN),
            Effector,
        ))
        .id();
    commands.entity(current_parent).add_child(effector);

    // Create IKChain component on Root
    commands.entity(root).insert(IKChain {
        joints: joints.clone(),
        effector,
        target: target_entity,
        iterations: 10,
    });

    // Create Choreographer
    commands.spawn(Choreographer {
        target_entity,
        current_target_node: None,
        next_target_time: 2.0,
        speed: 200.0,
    });
}
