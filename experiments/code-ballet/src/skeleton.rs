use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct Dancer;

#[derive(Component)]
pub struct DancerPart {
    pub name: String,
}

#[derive(Component)]
pub struct Motor {
    pub target_angle: f32,
    pub stiffness: f32,
    pub damping: f32,
    pub max_torque: f32,
}

pub struct SkeletonPlugin;

impl Plugin for SkeletonPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_dancer);
    }
}

fn spawn_dancer(mut commands: Commands) {
    let start_pos = Vec2::new(-500.0, 50.0);

    // Torso (Main Body)
    let torso_size = Vec2::new(20.0, 40.0);
    let head_size = 15.0;
    let arm_size = Vec2::new(8.0, 25.0);
    let leg_size = Vec2::new(10.0, 30.0);

    let torso = commands.spawn((
        Dancer,
        DancerPart { name: "Torso".into() },
        RigidBody::Dynamic,
        Collider::cuboid(torso_size.x / 2.0, torso_size.y / 2.0),
        SpatialBundle::from_transform(Transform::from_translation(start_pos.extend(1.0))),
        ExternalImpulse::default(),
        ReadMassProperties::default(),
        Velocity::default(),
        Damping { linear_damping: 0.5, angular_damping: 1.0 },
    )).id();

    // Head
    let head = commands.spawn((
        DancerPart { name: "Head".into() },
        RigidBody::Dynamic,
        Collider::ball(head_size),
        SpatialBundle::from_transform(Transform::from_translation((start_pos + Vec2::new(0.0, 35.0)).extend(1.0))),
        Damping { linear_damping: 0.5, angular_damping: 0.5 },
    )).id();

    let joint = ImpulseJoint::new(
        torso,
        RevoluteJointBuilder::new()
            .local_anchor1(Vec2::new(0.0, 25.0))
            .local_anchor2(Vec2::new(0.0, -15.0))
            .motor_position(0.0, 1000.0, 100.0) // Keep head upright
    );
    commands.entity(head).insert(joint);


    // Helper to spawn limb
    let mut spawn_limb = |parent: Entity, anchor_parent: Vec2, size: Vec2, anchor_child: Vec2, angle_limit: (f32, f32), name: &str| -> Entity {
        let child = commands.spawn((
            DancerPart { name: name.into() },
            RigidBody::Dynamic,
            Collider::cuboid(size.x / 2.0, size.y / 2.0),
            SpatialBundle::from_transform(Transform::from_translation((start_pos + anchor_parent - anchor_child).extend(1.0))),
            Motor {
                target_angle: 0.0,
                stiffness: 50000.0, // Increased stiffness for testing
                damping: 1000.0,
                max_torque: 100000.0,
            },
            Damping { linear_damping: 0.5, angular_damping: 0.5 },
        )).id();

        let joint = ImpulseJoint::new(
            parent,
            RevoluteJointBuilder::new()
                .local_anchor1(anchor_parent)
                .local_anchor2(anchor_child)
                .limits([angle_limit.0, angle_limit.1])
        );
        commands.entity(child).insert(joint);
        child
    };

    // Legs
    // Left Thigh
    let l_thigh = spawn_limb(torso, Vec2::new(-10.0, -20.0), leg_size, Vec2::new(0.0, 15.0), (-1.5, 1.5), "L_Thigh");
    // Left Shin
    let _l_shin = spawn_limb(l_thigh, Vec2::new(0.0, -15.0), leg_size, Vec2::new(0.0, 15.0), (-0.1, 2.5), "L_Shin"); // Knee bends backward
    // Right Thigh
    let r_thigh = spawn_limb(torso, Vec2::new(10.0, -20.0), leg_size, Vec2::new(0.0, 15.0), (-1.5, 1.5), "R_Thigh");
    // Right Shin
    let _r_shin = spawn_limb(r_thigh, Vec2::new(0.0, -15.0), leg_size, Vec2::new(0.0, 15.0), (-0.1, 2.5), "R_Shin");

    // Arms
    // Left Upper Arm
    let l_arm = spawn_limb(torso, Vec2::new(-15.0, 15.0), arm_size, Vec2::new(0.0, 12.0), (-3.0, 3.0), "L_Arm");
    // Left Forearm
    let _l_forearm = spawn_limb(l_arm, Vec2::new(0.0, -12.0), arm_size, Vec2::new(0.0, 12.0), (-3.0, 3.0), "L_Forearm");

    // Right Upper Arm
    let r_arm = spawn_limb(torso, Vec2::new(15.0, 15.0), arm_size, Vec2::new(0.0, 12.0), (-3.0, 3.0), "R_Arm");
    // Right Forearm
    let _r_forearm = spawn_limb(r_arm, Vec2::new(0.0, -12.0), arm_size, Vec2::new(0.0, 12.0), (-3.0, 3.0), "R_Forearm");
}
