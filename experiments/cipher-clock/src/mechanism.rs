use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Rotor {
    #[allow(dead_code)]
    pub teeth: usize,
}

#[derive(Component)]
pub struct Pawl;

pub fn spawn_rotor(commands: &mut Commands, position: Vec2, radius: f32, teeth: usize) -> Entity {
    let angle_step = std::f32::consts::TAU / teeth as f32;

    // 1. Create the main body (The Hub)
    let rotor = commands
        .spawn((
            Name::new("Rotor"),
            Rotor { teeth },
            SpatialBundle::from_transform(Transform::from_translation(position.extend(0.0))),
            RigidBody::Dynamic,
            Collider::ball(radius * 0.8), // Inner hub collision
            Restitution::coefficient(0.1),
            Friction::coefficient(0.5),
            Damping {
                linear_damping: 0.0,
                angular_damping: 1.0,
            }, // Damping to stabilize
            // Visuals
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: radius * 0.8,
                    center: Vec2::ZERO,
                }),
                ..default()
            },
            Fill::color(Color::srgb(0.8, 0.6, 0.2)),
            Stroke::new(Color::BLACK, 2.0),
        ))
        .id();

    // 2. Add Teeth (Colliders + Visuals)
    for i in 0..teeth {
        let angle = i as f32 * angle_step;
        let next_angle = (i + 1) as f32 * angle_step;

        // Ratchet Tooth Geometry
        // Vertical face at 'angle', Slope to 'next_angle'
        let r_inner = radius * 0.8;
        let r_outer = radius;

        let p0 = Vec2::new(angle.cos() * r_inner, angle.sin() * r_inner);
        let p1 = Vec2::new(angle.cos() * r_outer, angle.sin() * r_outer);
        let p2 = Vec2::new(next_angle.cos() * r_inner, next_angle.sin() * r_inner);

        let triangle = vec![p0, p1, p2];

        // Collider Child
        commands
            .spawn((
                TransformBundle::default(),
                Collider::convex_hull(&triangle).unwrap(),
                Friction::coefficient(0.2),
            ))
            .set_parent(rotor);

        // Visual Child
        let shape = shapes::Polygon {
            points: triangle.clone(),
            closed: true,
        };

        commands
            .spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
                        0.0, 0.0, 0.1,
                    ))),
                    ..default()
                },
                Fill::color(Color::srgb(0.7, 0.5, 0.1)),
                Stroke::new(Color::BLACK, 1.0),
            ))
            .set_parent(rotor);
    }

    // 3. Pin to background (Axle)
    let axle = commands
        .spawn((
            TransformBundle::from_transform(Transform::from_translation(position.extend(-1.0))),
            RigidBody::Fixed,
        ))
        .id();

    commands.entity(rotor).insert(ImpulseJoint::new(
        axle,
        RevoluteJointBuilder::new()
            .local_anchor1(Vec2::ZERO)
            .local_anchor2(Vec2::ZERO),
    ));

    rotor
}

pub fn spawn_pawl(commands: &mut Commands, pivot_pos: Vec2, length: f32) -> Entity {
    let thickness = 10.0;

    // The Pawl Arm
    let pawl = commands
        .spawn((
            Name::new("Pawl"),
            Pawl,
            SpatialBundle::from_transform(Transform::from_translation(
                (pivot_pos + Vec2::new(length / 2.0, 0.0)).extend(0.0),
            )),
            RigidBody::Dynamic,
            Collider::cuboid(length / 2.0, thickness / 2.0),
            Restitution::coefficient(0.0),
            AdditionalMassProperties::Mass(1.0),
            // Visual
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Rectangle {
                    extents: Vec2::new(length, thickness),
                    origin: shapes::RectangleOrigin::Center,
                }),
                ..default()
            },
            Fill::color(Color::srgb(0.5, 0.5, 0.5)),
            Stroke::new(Color::BLACK, 1.0),
        ))
        .id();

    // The Pivot Point (Static)
    let pivot = commands
        .spawn((
            TransformBundle::from_transform(Transform::from_translation(pivot_pos.extend(0.0))),
            RigidBody::Fixed,
        ))
        .id();

    // Joint
    commands.entity(pawl).insert(ImpulseJoint::new(
        pivot,
        RevoluteJointBuilder::new()
            .local_anchor1(Vec2::new(-length / 2.0, 0.0)) // Pivot at left end of pawl
            .local_anchor2(Vec2::ZERO),
    ));

    pawl
}
