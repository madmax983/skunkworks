use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

pub const TOOTH_DEPTH: f32 = 5.0;
pub const TOOTH_WIDTH: f32 = 5.0;

#[derive(Component)]
pub struct Gear;

#[derive(Component)]
pub struct Rack;

#[derive(Component)]
pub struct OutputIndicator;

pub fn spawn_gear(
    commands: &mut Commands,
    position: Vec2,
    teeth: usize,
    radius: f32,
    color: Color,
) -> Entity {
    let mut colliders = Vec::new();

    // Main body
    colliders.push((
        Vec2::ZERO,
        0.0,
        Collider::ball(radius - TOOTH_DEPTH),
    ));

    // Teeth
    let angle_step = 2.0 * std::f32::consts::PI / teeth as f32;
    for i in 0..teeth {
        let angle = i as f32 * angle_step;
        let tooth_pos = Vec2::new(angle.cos(), angle.sin()) * (radius - TOOTH_DEPTH / 2.0);
        colliders.push((
            tooth_pos,
            angle,
            Collider::cuboid(TOOTH_DEPTH / 2.0, TOOTH_WIDTH / 2.0),
        ));
    }

    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(position.extend(0.0))),
            RigidBody::Dynamic,
            Collider::compound(colliders),
            ColliderMassProperties::Density(1.0),
            Damping { linear_damping: 0.5, angular_damping: 0.5 },
            Gear,
        ))
        .with_children(|parent| {
            // Visuals
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::splat(radius * 2.0)),
                    ..default()
                },
                ..default()
            });
            // Marker to see rotation
            parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color: Color::WHITE,
                    custom_size: Some(Vec2::new(radius, 2.0)),
                    ..default()
                },
                transform: Transform::from_translation(Vec3::new(radius/2.0, 0.0, 1.0)),
                ..default()
            });
        })
        .id()
}

pub fn spawn_rack(
    commands: &mut Commands,
    position: Vec2,
    length: f32,
    teeth_count: usize,
    _vertical: bool,
    color: Color,
    is_input: bool,
) -> Entity {
    let mut colliders = Vec::new();

    let width = 10.0;
    // Main bar
    colliders.push((
        Vec2::ZERO,
        0.0,
        Collider::cuboid(width / 2.0, length / 2.0),
    ));

    // Teeth
    let start_y = -length / 2.0;
    let step_y = length / teeth_count as f32;

    for i in 0..teeth_count {
        let y = start_y + i as f32 * step_y + step_y / 2.0;
        let tooth_pos = Vec2::new(width / 2.0 + TOOTH_DEPTH / 2.0, y);
        colliders.push((
            tooth_pos,
            0.0,
            Collider::cuboid(TOOTH_DEPTH / 2.0, TOOTH_WIDTH / 2.0),
        ));
    }

    let rb = if is_input { RigidBody::KinematicPositionBased } else { RigidBody::Dynamic };

    let id = commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(position.extend(0.0))),
            rb,
            Collider::compound(colliders),
            Rack,
        ))
        .with_children(|parent| {
             parent.spawn(SpriteBundle {
                sprite: Sprite {
                    color,
                    custom_size: Some(Vec2::new(width, length)),
                    ..default()
                },
                ..default()
            });
        })
        .id();

    if !is_input {
        commands.entity(id).insert(OutputIndicator);
    }

    id
}

pub fn spawn_differential_adder(
    commands: &mut Commands,
    pos: Vec2,
) {
    let rack_length = 200.0;
    let teeth = 20;
    let rack_offset = 30.0;

    // Input A (Left Rack)
    spawn_rack(commands, pos + Vec2::new(-rack_offset, 0.0), rack_length, teeth, true, Color::srgb(1.0, 0.0, 0.0), true);

    // Input B (Right Rack)
    let right_rack = spawn_rack(commands, Vec2::ZERO, rack_length, teeth, true, Color::srgb(0.0, 0.0, 1.0), true);
    commands.entity(right_rack).insert(Transform::from_translation((pos + Vec2::new(rack_offset, 0.0)).extend(0.0)).with_rotation(Quat::from_rotation_z(std::f32::consts::PI)));


    // Pinion (Center)
    let pinion_radius = 20.0;
    let pinion_teeth = 12;
    let pinion = spawn_gear(commands, pos, pinion_teeth, pinion_radius, Color::srgb(0.0, 1.0, 0.0));

    // Slider (Output)
    let slider = commands.spawn((
        SpatialBundle::from_transform(Transform::from_translation(pos.extend(-1.0))),
        RigidBody::Dynamic,
        Collider::cuboid(5.0, 5.0),
        LockedAxes::TRANSLATION_LOCKED_X | LockedAxes::ROTATION_LOCKED,
        OutputIndicator,
        Damping { linear_damping: 1.0, angular_damping: 0.0 },
    )).id();

    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::ZERO)
        .local_anchor2(Vec2::ZERO)
        .build();

    commands.entity(slider).with_children(|parent| {
        parent.spawn(SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::splat(10.0)),
                ..default()
            },
            ..default()
        });
    });

    commands.entity(slider).insert(ImpulseJoint::new(pinion, joint));
}
