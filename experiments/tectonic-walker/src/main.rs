use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod components;
mod git;
mod ik;
mod terrain;
mod walker;

use components::*;
use ik::*;
use terrain::*;
use walker::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .add_plugins(IKPlugin)
        .add_plugins(TerrainPlugin)
        .add_plugins(WalkerPlugin)
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (camera_follow, body_follow_system))
        .run();
}

#[derive(Component)]
struct BodyFollow {
    target: Entity,
    offset: Vec3,
    speed: f32,
}

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Spawn IK Target (Visualized as small red circle)
    let target_entity = commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Circle { radius: 5.0, center: Vec2::ZERO }),
            spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(-400.0, 0.0, 10.0))),
            ..default()
        },
        Fill::color(Color::RED),
        IKTarget,
    )).id();

    // Spawn Creature Chain Root
    // The root should follow the target with an offset (simulating the body moving)
    let root_pos = Vec3::new(-400.0, -100.0, 0.0);
    let root = commands.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Circle { radius: 10.0, center: Vec2::ZERO }),
            spatial: SpatialBundle::from_transform(Transform::from_translation(root_pos)),
            ..default()
        },
        Fill::color(Color::WHITE),
        ChainRoot,
        BodyFollow {
            target: target_entity,
            offset: Vec3::new(-50.0, 50.0, 0.0), // Body is behind and above? Or just behind.
            speed: 120.0, // Slightly slower than walker?
        },
    )).id();

    // Joints
    let mut current_parent = root;
    let mut joints = Vec::new();
    let num_segments = 4;
    let segment_length = 50.0;

    for i in 0..num_segments {
        let bone_shape = shapes::Line(Vec2::ZERO, Vec2::new(segment_length, 0.0));

        let transform = if i == 0 {
            Transform::from_translation(Vec3::ZERO)
        } else {
            Transform::from_translation(Vec3::new(segment_length, 0.0, 0.0))
        };

        let joint = commands.spawn((
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
            Bone { length: segment_length },
        )).id();

        commands.entity(current_parent).add_child(joint);
        current_parent = joint;
        joints.push(joint);
    }

    // Effector
    let effector = commands.spawn((
        ShapeBundle {
             path: GeometryBuilder::build_as(&shapes::RegularPolygon {
                sides: 3,
                feature: RegularPolygonFeature::Radius(8.0),
                ..default()
            }),
            spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(segment_length, 0.0, 0.0))),
            ..default()
        },
        Fill::color(Color::CYAN),
        Effector,
    )).id();
    commands.entity(current_parent).add_child(effector);

    // Create IKChain component on Root
    commands.entity(root).insert(IKChain {
        joints: joints.clone(),
        effector,
        target: target_entity,
        iterations: 15,
    });

    // Create Walker
    commands.spawn(Walker {
        target_entity,
        current_commit_idx: 0,
        speed: 150.0,
    });
}

fn camera_follow(
    walker_query: Query<&Walker>,
    mut camera_query: Query<&mut Transform, With<Camera>>,
    target_query: Query<&GlobalTransform>,
) {
    if let Ok(walker) = walker_query.get_single() {
        if let Ok(target_tf) = target_query.get(walker.target_entity) {
            let target_pos = target_tf.translation();
            if let Ok(mut cam_tf) = camera_query.get_single_mut() {
                // Smooth follow X
                cam_tf.translation.x = cam_tf.translation.x + (target_pos.x - cam_tf.translation.x) * 0.1;
                // Optional: Follow Y slightly
                cam_tf.translation.y = cam_tf.translation.y + (target_pos.y - cam_tf.translation.y) * 0.05;
            }
        }
    }
}

fn body_follow_system(
    time: Res<Time>,
    mut body_query: Query<(&mut Transform, &BodyFollow)>,
    target_query: Query<&GlobalTransform>,
) {
    for (mut body_tf, follow) in body_query.iter_mut() {
        if let Ok(target_tf) = target_query.get(follow.target) {
            let target_pos = target_tf.translation();
            let desired_pos = target_pos + follow.offset;

            let current = body_tf.translation;
            let dir = desired_pos - current;
            let dist = dir.length();

            if dist > 1.0 {
                // Lerp
                let step = dir.normalize() * follow.speed * time.delta_seconds();
                if step.length() > dist {
                    body_tf.translation = desired_pos;
                } else {
                    body_tf.translation += step;
                }
            }
        }
    }
}
