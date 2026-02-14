use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Climber;

#[derive(Component)]
pub struct Hand {
    pub side: Side,
}

#[derive(Component)]
pub struct Limb {
    pub side: Side,
    pub bone_type: BoneType,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum Side {
    Left,
    Right,
}

#[derive(Debug, Clone, Copy, PartialEq)]
pub enum BoneType {
    Upper,
    Lower,
}

pub fn spawn_climber(mut commands: Commands) {
    let start_pos = Vec2::new(0.0, -50.0);

    // Torso
    let torso_w = 20.0;
    let torso_h = 40.0;
    let torso = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.9, 0.2, 0.5), // Pink
                custom_size: Some(Vec2::new(torso_w, torso_h)),
                ..default()
            },
            transform: Transform::from_xyz(start_pos.x, start_pos.y, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(torso_w / 2.0, torso_h / 2.0),
        Climber,
        Name::new("Torso"),
    )).id();

    // Head
    let head_r = 12.0;
    let head = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.9, 0.2, 0.5),
                custom_size: Some(Vec2::new(head_r * 2.0, head_r * 2.0)),
                ..default()
            },
            transform: Transform::from_xyz(start_pos.x, start_pos.y + torso_h/2.0 + head_r, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::ball(head_r),
        Name::new("Head"),
    )).id();

    commands.entity(head).insert(ImpulseJoint::new(torso, RevoluteJointBuilder::new().local_anchor1(Vec2::new(0.0, torso_h/2.0)).local_anchor2(Vec2::new(0.0, -head_r))));

    // Arms
    spawn_limb(&mut commands, torso, start_pos, Side::Left, true);
    spawn_limb(&mut commands, torso, start_pos, Side::Right, true);

    // Legs
    spawn_limb(&mut commands, torso, start_pos, Side::Left, false);
    spawn_limb(&mut commands, torso, start_pos, Side::Right, false);
}

fn spawn_limb(commands: &mut Commands, torso: Entity, torso_pos: Vec2, side: Side, is_arm: bool) {
    let side_mul = if side == Side::Left { -1.0 } else { 1.0 };
    let limb_w = 6.0;
    let limb_l = 25.0; // Length of one segment

    // Anchor on torso
    let anchor_y = if is_arm { 15.0 } else { -15.0 };
    let anchor_x = side_mul * 10.0;

    let upper_center_x = torso_pos.x + anchor_x + side_mul * limb_l / 2.0;
    let upper_center_y = torso_pos.y + anchor_y;

    let upper = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.9, 0.4, 0.6),
                custom_size: Some(Vec2::new(limb_l, limb_w)),
                ..default()
            },
            transform: Transform::from_xyz(upper_center_x, upper_center_y, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(limb_l / 2.0, limb_w / 2.0),
        ColliderMassProperties::Density(0.5),
        Limb { side, bone_type: BoneType::Upper },
    )).id();

    let joint_anchor2_x = -side_mul * limb_l / 2.0;

    let joint = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(anchor_x, anchor_y))
        .local_anchor2(Vec2::new(joint_anchor2_x, 0.0))
        .motor_position(0.0, 10000.0, 100.0);

    commands.entity(upper).insert(ImpulseJoint::new(torso, joint));

    // Lower Limb
    let lower_center_x = upper_center_x + side_mul * limb_l;
    let lower_center_y = upper_center_y;

    let lower = commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(0.9, 0.6, 0.7),
                custom_size: Some(Vec2::new(limb_l, limb_w)),
                ..default()
            },
            transform: Transform::from_xyz(lower_center_x, lower_center_y, 0.0),
            ..default()
        },
        RigidBody::Dynamic,
        Collider::cuboid(limb_l / 2.0, limb_w / 2.0),
        ColliderMassProperties::Density(0.5),
        Limb { side, bone_type: BoneType::Lower },
    )).id();

    let joint2 = RevoluteJointBuilder::new()
        .local_anchor1(Vec2::new(side_mul * limb_l / 2.0, 0.0))
        .local_anchor2(Vec2::new(-side_mul * limb_l / 2.0, 0.0))
        .motor_position(0.0, 10000.0, 100.0);

    commands.entity(lower).insert(ImpulseJoint::new(upper, joint2));

    // Hand/Foot
    if is_arm {
         commands.entity(lower).insert(Hand { side });
    }
}
