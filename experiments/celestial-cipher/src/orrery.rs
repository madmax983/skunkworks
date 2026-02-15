use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::f32::consts::PI;

pub const GEAR_TOOTH_SIZE: f32 = 10.0;

#[derive(Component)]
pub struct Planet {
    pub name: &'static str,
    pub period_days: f32,
}

#[derive(Component)]
pub struct MainShaft;

#[derive(Component)]
pub struct PlanetArm;

// Gear Ratios (Driver Teeth, Driven Teeth, Visual Distance, Color)
pub const RATIOS: &[(&str, usize, usize, f32, Color)] = &[
    ("Mercury", 83, 20, 50.0, Color::srgb(0.5, 0.5, 0.5)),
    ("Venus", 13, 8, 80.0, Color::srgb(0.9, 0.9, 0.5)),
    ("Earth", 40, 40, 110.0, Color::srgb(0.0, 0.0, 1.0)),
    ("Mars", 32, 60, 150.0, Color::srgb(1.0, 0.0, 0.0)),
    ("Jupiter", 7, 83, 220.0, Color::srgb(0.8, 0.5, 0.2)),
    ("Saturn", 3, 88, 300.0, Color::srgb(0.8, 0.7, 0.4)),
];

pub fn spawn_orrery(mut commands: Commands) {
    // 1. Main Drive Shaft (The "Sun" axle)
    let main_shaft_radius = 20.0;

    let main_id = commands
        .spawn((
            TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
            RigidBody::Dynamic,
            Collider::ball(main_shaft_radius),
            ColliderMassProperties::Density(10.0),
            Damping { linear_damping: 0.0, angular_damping: 2.0 },
            ExternalImpulse::default(),
            MainShaft,
        ))
        .with_children(|parent| {
             // Sun Visual
             parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Circle {
                        radius: 15.0,
                        ..default()
                    }),
                    ..default()
                },
                Fill::color(Color::srgb(1.0, 1.0, 0.0)),
                Stroke::new(Color::srgb(1.0, 0.5, 0.0), 2.0),
             ));
        })
        .id();

    // Pin Main Shaft to World Center
    let world_anchor = commands.spawn((
        TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
        RigidBody::Fixed,
    )).id();

    commands.entity(main_id).insert(ImpulseJoint::new(
        world_anchor,
        RevoluteJointBuilder::new().local_anchor1(Vec2::ZERO).local_anchor2(Vec2::ZERO)
    ));

    // For each planet, create the friction drive mechanism
    let mut group_idx = 0;

    for (name, driver_teeth, driven_teeth, visual_dist, color) in RATIOS.iter() {
        // Calculate Radii
        let driver_r = (*driver_teeth as f32 * GEAR_TOOTH_SIZE) / (2.0 * PI);
        let driven_r = (*driven_teeth as f32 * GEAR_TOOTH_SIZE) / (2.0 * PI);

        // Idler Logic:
        let min_idler_b_teeth = 10;
        let idler_a_teeth_needed = if *driver_teeth + min_idler_b_teeth > *driven_teeth {
            20
        } else {
            *driven_teeth + min_idler_b_teeth - *driver_teeth
        };

        let idler_a_teeth = idler_a_teeth_needed.max(20);

        let idler_a_r = (idler_a_teeth as f32 * GEAR_TOOTH_SIZE) / (2.0 * PI);
        let idler_b_r = driver_r + idler_a_r - driven_r;

        let center_dist = driver_r + idler_a_r;

        // Collision Groups
        let group_a = Group::from_bits_truncate(1 << (group_idx));
        let group_b = Group::from_bits_truncate(1 << (group_idx + 1));
        group_idx += 2;

        // 1. Driver Gear (Attached to Main Shaft)
        commands.entity(main_id).with_children(|parent| {
            parent.spawn((
                TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
                Collider::ball(driver_r),
                CollisionGroups::new(group_a, group_a),
                Friction { coefficient: 100.0, combine_rule: CoefficientCombineRule::Max },
            ));
        });

        // 2. Idler Gear (Compound, spins freely at fixed location)
        let idler_pos = Vec2::new(center_dist, 0.0);
        let idler_anchor = commands.spawn((
            TransformBundle::from(Transform::from_translation(idler_pos.extend(0.0))),
            RigidBody::Fixed,
        )).id();

        let idler_body = commands.spawn((
            TransformBundle::from(Transform::from_translation(idler_pos.extend(0.0))),
            RigidBody::Dynamic,
            Damping { linear_damping: 0.0, angular_damping: 0.1 },
        )).id();

        commands.entity(idler_body).insert(ImpulseJoint::new(
            idler_anchor,
            RevoluteJointBuilder::new().local_anchor1(Vec2::ZERO).local_anchor2(Vec2::ZERO)
        ));

        // Add Colliders to Idler Body (as children)
        commands.entity(idler_body).with_children(|parent| {
            // Idler A (Meshes with Driver)
            parent.spawn((
                TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
                Collider::ball(idler_a_r),
                CollisionGroups::new(group_a, group_a),
                Friction { coefficient: 100.0, combine_rule: CoefficientCombineRule::Max },
            ));

            // Idler B (Meshes with Driven)
            parent.spawn((
                TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
                Collider::ball(idler_b_r),
                CollisionGroups::new(group_b, group_b),
                Friction { coefficient: 100.0, combine_rule: CoefficientCombineRule::Max },
            ));

            // Visual for Idler
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Circle { radius: idler_a_r.max(idler_b_r), ..default() }),
                    ..default()
                },
                Fill::color(Color::srgb(0.2, 0.2, 0.2)),
            ));
        });

        // 3. Driven Gear + Planet Arm (Rotates around center)
        let planet_arm_id = commands.spawn((
            TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
            RigidBody::Dynamic,
            Damping { linear_damping: 0.0, angular_damping: 0.5 },
            Planet { name, period_days: 0.0 },
            PlanetArm,
        )).id();

        // Pin to Center
        commands.entity(planet_arm_id).insert(ImpulseJoint::new(
            world_anchor,
            RevoluteJointBuilder::new().local_anchor1(Vec2::ZERO).local_anchor2(Vec2::ZERO)
        ));

        // Add Collider (Driven Gear)
        commands.entity(planet_arm_id).with_children(|parent| {
            parent.spawn((
                TransformBundle::from(Transform::from_xyz(0.0, 0.0, 0.0)),
                Collider::ball(driven_r),
                CollisionGroups::new(group_b, group_b),
                Friction { coefficient: 100.0, combine_rule: CoefficientCombineRule::Max },
            ));

            // Visual Arm
            parent.spawn((
                 ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Line(Vec2::ZERO, Vec2::new(*visual_dist, 0.0))),
                    ..default()
                },
                Stroke::new(Color::WHITE, 1.0),
            ));

            // Visual Planet
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Circle { radius: 5.0, ..default() }),
                    spatial: SpatialBundle::from_transform(Transform::from_xyz(*visual_dist, 0.0, 0.0)),
                    ..default()
                },
                Fill::color(*color),
                Stroke::new(Color::WHITE, 1.0),
            ));
        });
    }
}
