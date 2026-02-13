use crate::Anchor;
use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

pub struct MechanismPlugin;

impl Plugin for MechanismPlugin {
    fn build(&self, _app: &mut App) {
        // No systems here yet
    }
}

pub fn get_tooth_points(height: f32, width: f32) -> Vec<Vec2> {
    vec![
        Vec2::new(0.0, width / 2.0),
        Vec2::new(0.0, -width / 2.0),
        Vec2::new(height, -width / 2.0),
    ]
}

pub fn get_tooth_collider(height: f32, width: f32) -> Collider {
    let points = get_tooth_points(height, width);
    Collider::convex_hull(&points).unwrap()
}

pub fn get_anchor_colliders(radius: f32, span_teeth: f32, tooth_pitch_angle: f32) -> Vec<(Vect, f32, Collider)> {
    let mut shapes = Vec::new();

    let half_span_angle = (span_teeth * tooth_pitch_angle) / 2.0;
    // Pivot distance for tangent pallets
    // pivot_dist = radius / cos(half_span_angle)
    // arm_length = radius * tan(half_span_angle)

    // Safety clamp to avoid division by zero or weird angles
    let safe_angle = half_span_angle.clamp(0.1, 1.5);
    let arm_length = radius * safe_angle.tan();

    let pallet_size = Vec2::new(0.4, 0.8);
    let pallet_collider = Collider::cuboid(pallet_size.x / 2.0, pallet_size.y / 2.0);

    // Arm angle from vertical
    // In the tangent triangle, angle at pivot = PI/2 - angle at center
    let arm_angle = PI / 2.0 - safe_angle;

    // Calculate pallet positions relative to anchor pivot (0,0)
    // Left Pallet: -px, py
    // Right Pallet: px, py
    // Where px = arm_length * sin(arm_angle), py = -arm_length * cos(arm_angle)
    let px = arm_length * arm_angle.sin();
    let py = -arm_length * arm_angle.cos();

    // Left Pallet
    shapes.push((
        Vect::new(-px, py),
        0.5, // Tilt
        pallet_collider.clone(),
    ));

    // Right Pallet
    shapes.push((
        Vect::new(px, py),
        -0.5, // Tilt
        pallet_collider.clone(),
    ));

    // Arms
    let arm_shape = Collider::cuboid(0.1, arm_length / 2.0);
    // Left Arm
    shapes.push((
        Vect::new(-px/2.0, py/2.0),
        arm_angle,
        arm_shape.clone()
    ));
    // Right Arm
    shapes.push((
        Vect::new(px/2.0, py/2.0),
        -arm_angle,
        arm_shape.clone()
    ));

    shapes
}

pub fn spawn_gear(
    commands: &mut Commands,
    position: Vec2,
    teeth: usize,
    radius: f32,
    mass_density: f32,
) -> Entity {
    let mut shapes = Vec::new();
    shapes.push((Vect::ZERO, 0.0, Collider::ball(radius - 0.5)));

    let circumference = 2.0 * PI * radius;
    let pitch = circumference / (teeth as f32);
    let tooth_width = pitch * 0.8;
    let tooth_height = 1.2;

    let tooth_shape = get_tooth_collider(tooth_height, tooth_width);

    for i in 0..teeth {
        let angle = (i as f32) * 2.0 * PI / (teeth as f32);
        let dist = radius - 0.2;
        let x = dist * angle.cos();
        let y = dist * angle.sin();

        shapes.push((
            Vect::new(x, y),
            angle,
            tooth_shape.clone(),
        ));
    }

    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(position.extend(0.0))),
            RigidBody::Dynamic,
            Collider::compound(shapes),
            ColliderMassProperties::Density(mass_density),
            Damping {
                linear_damping: 0.1,
                angular_damping: 0.5,
            },
            LockedAxes::TRANSLATION_LOCKED,
            crate::EscapeWheel {
                last_angle: 0.0,
                teeth,
                cumulative_angle: 0.0,
                radius,
            }
        ))
        .id()
}

pub fn spawn_anchor(
    commands: &mut Commands,
    position: Vec2,
    wheel_radius: f32,
    wheel_teeth: usize,
    span_teeth: f32
) -> Entity {
    let mut shapes = Vec::new();

    let pitch_angle = 2.0 * PI / (wheel_teeth as f32);
    let pallet_shapes = get_anchor_colliders(wheel_radius, span_teeth, pitch_angle);
    shapes.extend(pallet_shapes);

    // Pendulum
    shapes.push((Vect::new(0.0, -6.0), 0.0, Collider::cuboid(0.2, 6.0)));
    shapes.push((Vect::new(0.0, -12.0), 0.0, Collider::ball(1.5)));

    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(position.extend(0.0))),
            RigidBody::Dynamic,
            Collider::compound(shapes),
            ColliderMassProperties::Density(2.0),
            Damping {
                linear_damping: 0.1,
                angular_damping: 0.1,
            },
            LockedAxes::TRANSLATION_LOCKED,
            Anchor {
                wheel_radius,
                span_teeth,
                wheel_teeth,
            },
        ))
        .id()
}
