use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Gear {
    pub teeth: usize,
    pub radius: f32,
    pub module: f32,
}

#[derive(Component)]
pub struct EscapementAnchor;

#[derive(Component)]
pub struct EscapeWheel;

const DEFAULT_MODULE: f32 = 2.0;

/// Spawns a gear with N teeth.
pub fn spawn_gear(
    commands: &mut Commands,
    position: Vec2,
    teeth: usize,
    density: f32,
) -> Entity {
    // Module m = Diameter / Teeth. D = m * N. R = m * N / 2.
    let radius = (teeth as f32) * DEFAULT_MODULE / 2.0;

    let mut shapes = Vec::new();

    // Main disk (rim)
    shapes.push((Vec2::ZERO, 0.0, Collider::ball(radius * 0.85)));

    // Teeth
    // Circular Pitch p = PI * m.
    let pitch = PI * DEFAULT_MODULE;
    let tooth_width = pitch * 0.45;
    let tooth_height = DEFAULT_MODULE * 0.8;

    let tooth_shape = Collider::cuboid(tooth_height / 2.0, tooth_width / 2.0);

    for i in 0..teeth {
        let angle = (i as f32) * 2.0 * PI / (teeth as f32);

        let dist = radius;
        let x = dist * angle.cos();
        let y = dist * angle.sin();

        shapes.push((
            Vec2::new(x, y),
            angle,
            tooth_shape.clone(),
        ));
    }

    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(position.extend(0.0))),
            RigidBody::Dynamic,
            Collider::compound(shapes),
            ColliderMassProperties::Density(density),
            Damping {
                linear_damping: 0.1,
                angular_damping: 0.2,
            },
            LockedAxes::TRANSLATION_LOCKED,
            Gear {
                teeth,
                radius,
                module: DEFAULT_MODULE,
            },
        ))
        .id()
}

pub fn spawn_escapement(
    commands: &mut Commands,
    pos: Vec2,
) {
    // 1. Escape Wheel
    let teeth = 15;
    let radius = (teeth as f32) * DEFAULT_MODULE / 2.0;

    // Custom escape wheel shape
    let mut wheel_shapes = Vec::new();
    wheel_shapes.push((Vec2::ZERO, 0.0, Collider::ball(radius * 0.8)));

    let tooth_len = radius * 0.25;
    let tooth_width = radius * 0.08;
    let tooth_shape = Collider::cuboid(tooth_len / 2.0, tooth_width / 2.0);

    for i in 0..teeth {
        let angle = (i as f32) * 2.0 * PI / (teeth as f32);
        let dist = radius * 0.95;
        let x = dist * angle.cos();
        let y = dist * angle.sin();

        // Tilt the tooth for "ratchet" shape
        wheel_shapes.push((
            Vec2::new(x, y),
            angle - 0.4,
            tooth_shape.clone(),
        ));
    }

    commands.spawn((
        SpatialBundle::from_transform(Transform::from_translation(pos.extend(0.0))),
        RigidBody::Dynamic,
        Collider::compound(wheel_shapes),
        ColliderMassProperties::Density(1.0),
        ExternalForce {
            torque: -15000000.0, // Significant torque
            ..default()
        },
        Damping {
            linear_damping: 0.0,
            angular_damping: 0.1,
        },
        LockedAxes::TRANSLATION_LOCKED,
        EscapeWheel,
        Gear { teeth, radius, module: DEFAULT_MODULE },
    ));

    // 2. Anchor
    let dist = radius * 1.35;
    let anchor_pos = pos + Vec2::new(0.0, dist);

    let mut shapes = Vec::new();

    // Pivot arms
    // Visualizing/Simulating the pallet arms
    let arm_len = dist * 0.9;

    // Left Pallet (Exit)
    let spread_angle: f32 = 0.5; // rad
    let l_pos = Vec2::new(-arm_len * spread_angle.sin(), -arm_len * spread_angle.cos());
    shapes.push((
        l_pos,
        -spread_angle + 0.5, // Angle of attack
        Collider::cuboid(0.8, 1.5)
    ));

    // Right Pallet (Entry)
    let r_pos = Vec2::new(arm_len * spread_angle.sin(), -arm_len * spread_angle.cos());
    shapes.push((
        r_pos,
        spread_angle - 0.5,
        Collider::cuboid(0.8, 1.5)
    ));

    // Pendulum Rod
    shapes.push((Vec2::new(0.0, 15.0), 0.0, Collider::cuboid(0.4, 15.0)));
    // Bob
    shapes.push((Vec2::new(0.0, 30.0), 0.0, Collider::ball(3.0)));

    commands.spawn((
        SpatialBundle::from_transform(Transform::from_translation(anchor_pos.extend(0.0))),
        RigidBody::Dynamic,
        Collider::compound(shapes),
        ColliderMassProperties::Density(2.0),
        Damping { linear_damping: 0.0, angular_damping: 0.02 },
        LockedAxes::TRANSLATION_LOCKED,
        EscapementAnchor,
    ));
}

/// Spawns a gear that meshes with a parent gear
#[allow(dead_code)]
pub fn spawn_meshed_gear(
    commands: &mut Commands,
    parent_pos: Vec2,
    parent_teeth: usize,
    teeth: usize,
    angle_offset: f32, // Where around the parent to place it (radians)
) -> Entity {
    // Module must be consistent
    let r_parent = (parent_teeth as f32) * DEFAULT_MODULE / 2.0;
    let r_new = (teeth as f32) * DEFAULT_MODULE / 2.0;

    // Center distance = r1 + r2
    let dist = r_parent + r_new + 0.5; // +0.5 for slight clearance

    let x = parent_pos.x + dist * angle_offset.cos();
    let y = parent_pos.y + dist * angle_offset.sin();

    spawn_gear(commands, Vec2::new(x, y), teeth, 1.0)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gear_radius() {
        // Module = 2.0
        // Radius = Teeth * Module / 2.0
        // 15 teeth -> 15.0 radius
        let teeth = 15;
        let radius = (teeth as f32) * DEFAULT_MODULE / 2.0;
        assert_eq!(radius, 15.0);
    }
}
