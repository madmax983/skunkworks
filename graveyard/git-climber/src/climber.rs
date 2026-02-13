use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Climber;

#[derive(Component)]
#[allow(dead_code)]
pub struct Limb {
    pub limb_type: LimbType,
    pub segment_index: usize, // 0 = upper, 1 = lower
}

#[derive(Component, Clone, Copy, PartialEq, Eq, Debug)]
#[allow(dead_code)]
pub enum LimbType {
    LeftArm,
    RightArm,
    LeftLeg,
    RightLeg,
}

#[derive(Component)]
#[allow(dead_code)]
pub struct HandSensor {
    pub limb_type: LimbType,
}

pub fn spawn_climber(mut commands: Commands) {
    let torso_radius = 15.0;
    let limb_width = 6.0;
    let segment_length = 30.0;

    let start_pos = Vec2::new(0.0, -150.0);

    // Torso
    let torso = commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: torso_radius,
                    ..default()
                }),
                spatial: SpatialBundle::from_transform(Transform::from_translation(
                    start_pos.extend(0.0),
                )),
                ..default()
            },
            Fill::color(Color::srgba(0.2, 0.8, 0.2, 1.0)),
            Stroke::new(Color::BLACK, 2.0),
            RigidBody::Dynamic,
            Collider::ball(torso_radius),
            Restitution::coefficient(0.1),
            ExternalImpulse::default(),
            ExternalForce::default(),
            Climber,
        ))
        .id();

    // Limbs
    // (Type, Anchor relative to torso center)
    let limb_configs = [
        (LimbType::LeftArm, Vec2::new(-torso_radius, 0.0)),
        (LimbType::RightArm, Vec2::new(torso_radius, 0.0)),
        (
            LimbType::LeftLeg,
            Vec2::new(-torso_radius * 0.7, -torso_radius * 0.7),
        ),
        (
            LimbType::RightLeg,
            Vec2::new(torso_radius * 0.7, -torso_radius * 0.7),
        ),
    ];

    for (limb_type, anchor_pos) in limb_configs.iter() {
        let mut parent_entity = torso;
        let mut local_anchor1 = *anchor_pos;

        let _ = start_pos;

        for i in 0..2 {
            let is_hand = i == 1;

            let segment_len = if is_hand {
                segment_length * 0.8
            } else {
                segment_length
            };

            let spawn_pos = if i == 0 {
                start_pos + *anchor_pos + Vec2::new(0.0, -segment_len / 2.0)
            } else {
                start_pos + *anchor_pos + Vec2::new(0.0, -segment_length - segment_len / 2.0)
            };

            let segment_shape = shapes::Rectangle {
                extents: Vec2::new(limb_width, segment_len),
                origin: RectangleOrigin::Center,
            };

            let segment = commands
                .spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&segment_shape),
                        spatial: SpatialBundle::from_transform(Transform::from_translation(
                            spawn_pos.extend(0.0),
                        )),
                        ..default()
                    },
                    Fill::color(if is_hand {
                        Color::srgba(1.0, 0.5, 0.0, 1.0)
                    } else {
                        Color::srgba(0.2, 0.6, 0.2, 1.0)
                    }),
                    RigidBody::Dynamic,
                    Collider::cuboid(limb_width / 2.0, segment_len / 2.0),
                    ExternalImpulse::default(),
                    ExternalForce::default(),
                    Limb {
                        limb_type: *limb_type,
                        segment_index: i,
                    },
                ))
                .id();

            // Connect to parent
            commands.entity(segment).insert(ImpulseJoint::new(
                parent_entity,
                RevoluteJointBuilder::new()
                    .local_anchor1(local_anchor1)
                    .local_anchor2(Vec2::new(0.0, segment_len / 2.0))
                    .motor_position(0.0, 1000.0, 10.0)
                    .limits([-PI / 2.0, PI / 2.0]),
            ));

            if is_hand {
                // Add Sensor at the tip
                commands.entity(segment).with_children(|parent| {
                    parent.spawn((
                        Collider::ball(5.0),
                        Sensor,
                        TransformBundle::from(Transform::from_xyz(0.0, -segment_len / 2.0, 0.0)),
                        HandSensor {
                            limb_type: *limb_type,
                        },
                    ));
                });
            }

            parent_entity = segment;
            local_anchor1 = Vec2::new(0.0, -segment_len / 2.0);
        }
    }
}
