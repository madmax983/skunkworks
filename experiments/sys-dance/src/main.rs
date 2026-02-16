use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

mod choreographer;
mod ik;
mod laban;
mod monitor;
mod skeleton;

use choreographer::{choreograph_system, DancerLimb, LimbType};
use ik::ik_system;
use laban::{update_laban_from_monitor, LabanState};
use monitor::{update_system_stats, SystemMonitor};
use skeleton::{Bone, IKChain, SkeletonRoot};

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_plugins(ShapePlugin)
        .init_resource::<SystemMonitor>()
        .init_resource::<LabanState>()
        .add_systems(Startup, setup)
        .add_systems(
            Update,
            (
                update_system_stats,
                update_laban_from_monitor,
                choreograph_system,
                ik_system,
            )
                .chain(),
        )
        .run();
}

fn setup(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    // Spawn Skeleton
    // Torso
    let torso = commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: Vec2::new(40.0, 100.0),
                    origin: RectangleOrigin::Center,
                }),
                ..default()
            },
            Fill::color(Color::GRAY),
            Stroke::new(Color::BLACK, 2.0),
            SkeletonRoot,
            SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.0)),
        ))
        .id();

    // Head
    let head = commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: 25.0,
                    center: Vec2::ZERO,
                }),
                spatial: SpatialBundle::from_transform(Transform::from_xyz(0.0, 65.0, 0.1)),
                ..default()
            },
            Fill::color(Color::WHITE),
            Stroke::new(Color::BLACK, 2.0),
        ))
        .id();
    commands.entity(torso).add_child(head);

    // Helper to spawn limb chain
    fn spawn_limb(
        commands: &mut Commands,
        parent: Entity,
        pos: Vec2,
        len1: f32,
        len2: f32,
        limb_type: LimbType,
        side: f32,
    ) {
        // Bone 1 (Upper)
        let bone1 = commands
            .spawn((
                SpatialBundle::from_transform(Transform::from_xyz(pos.x, pos.y, 0.0)), // Relative to parent
                Bone {
                    length: len1,
                    thickness: 5.0,
                    color: Color::RED,
                },
            ))
            .id();

        // Visual for Bone 1
        let visual1 = commands
            .spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Line(
                        Vec2::ZERO,
                        Vec2::new(0.0, -len1), // Points down by default
                    )),
                    ..default()
                },
                Stroke::new(Color::RED, 6.0),
            ))
            .id();
        commands.entity(bone1).add_child(visual1);
        commands.entity(parent).add_child(bone1); // Attach to Torso

        // Bone 2 (Lower)
        let bone2 = commands
            .spawn((
                SpatialBundle::from_transform(Transform::from_xyz(0.0, -len1, 0.0)), // End of Bone 1
                Bone {
                    length: len2,
                    thickness: 4.0,
                    color: Color::BLUE,
                },
            ))
            .id();

        let visual2 = commands
            .spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Line(
                        Vec2::ZERO,
                        Vec2::new(0.0, -len2),
                    )),
                    ..default()
                },
                Stroke::new(Color::BLUE, 4.0),
            ))
            .id();
        commands.entity(bone2).add_child(visual2);
        commands.entity(bone1).add_child(bone2); // Attach to Bone 1

        // IK Chain & Dancer Logic
        commands.entity(bone1).insert((
            IKChain {
                target: Vec2::new(side * 20.0, -len1 - len2), // Initial target
                bone1: bone1,
                bone2: bone2,
                len1,
                len2,
                bend_dir: side, // Bend outward
            },
            DancerLimb {
                side,
                limb_type,
                phase: 0.0,
                base_offset: Vec2::new(side * 20.0, -len1 - len2),
            },
        ));
    }

    // Arms
    // Left (-1.0)
    spawn_limb(
        &mut commands,
        torso,
        Vec2::new(-25.0, 40.0),
        50.0,
        50.0,
        LimbType::Arm,
        -1.0,
    );
    // Right (1.0)
    spawn_limb(
        &mut commands,
        torso,
        Vec2::new(25.0, 40.0),
        50.0,
        50.0,
        LimbType::Arm,
        1.0,
    );

    // Legs
    // Left (-1.0)
    spawn_limb(
        &mut commands,
        torso,
        Vec2::new(-15.0, -50.0),
        60.0,
        60.0,
        LimbType::Leg,
        -1.0,
    );
    // Right (1.0)
    spawn_limb(
        &mut commands,
        torso,
        Vec2::new(15.0, -50.0),
        60.0,
        60.0,
        LimbType::Leg,
        1.0,
    );
}
