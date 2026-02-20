use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

pub struct MechanismPlugin;

impl Plugin for MechanismPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<TickEvent>()
           .add_systems(Update, detect_tick_system);
    }
}

#[derive(Event)]
pub struct TickEvent;

#[derive(Component)]
pub struct EscapeWheel {
    pub teeth: usize,
    pub radius: f32,
    pub angle: f32,
    pub last_angle: f32,
    pub cumulative_angle: f32,
}

#[derive(Component)]
pub struct Anchor;

pub fn spawn_gear(
    commands: &mut Commands,
    position: Vec2,
    teeth: usize,
    radius: f32,
    mass_density: f32,
) -> Entity {
    let mut shapes = Vec::new();

    // Main disk (rim)
    // Reduce radius slightly to allow teeth to sit on it
    shapes.push((Vect::ZERO, 0.0, Collider::ball(radius - 0.5)));

    // Teeth
    // Calculate tooth dimensions based on circumference
    // Circumference = 2 * PI * radius
    // Pitch = Circumference / teeth
    let pitch = (2.0 * PI * radius) / (teeth as f32);
    let tooth_width = pitch * 0.5; // Fill 50% of pitch with tooth
    let tooth_height = 1.0;

    // Rapier cuboid uses half-extents
    let tooth_shape = Collider::cuboid(tooth_height / 2.0, tooth_width / 2.0);

    for i in 0..teeth {
        let angle = (i as f32) * 2.0 * PI / (teeth as f32);
        let dist = radius;
        let x = dist * angle.cos();
        let y = dist * angle.sin();

        shapes.push((
            Vect::new(x, y),
            angle, // Rotate to point outward
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
            ExternalForce::default(),
            EscapeWheel {
                teeth,
                radius,
                angle: 0.0,
                last_angle: 0.0,
                cumulative_angle: 0.0,
            },
        ))
        .id()
}

pub fn spawn_anchor(commands: &mut Commands, position: Vec2) -> Entity {
    let mut shapes = Vec::new();

    // Anchor geometry is specific to the escape wheel size.
    // Assuming escape wheel is roughly radius 5.0 nearby below.

    // Left pallet
    shapes.push((
        Vect::new(-1.5, -2.0),
        0.5, // Tilted
        Collider::cuboid(0.2, 0.2),
    ));

    // Right pallet
    shapes.push((
        Vect::new(1.5, -2.0),
        -0.5, // Tilted opposite
        Collider::cuboid(0.2, 0.2),
    ));

    // Arms connecting to pivot
    shapes.push((Vect::new(-2.5, -1.5), 0.5, Collider::cuboid(0.2, 2.0)));
    shapes.push((Vect::new(2.5, -1.5), -0.5, Collider::cuboid(0.2, 2.0)));

    // Pendulum Rod (upwards or downwards) - let's make it a pendulum swinging below
    shapes.push((Vect::new(0.0, -8.0), 0.0, Collider::cuboid(0.2, 8.0)));

    // Bob
    shapes.push((Vect::new(0.0, -16.0), 0.0, Collider::ball(2.0)));

    commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(position.extend(0.0))),
            RigidBody::Dynamic,
            Collider::compound(shapes),
            ColliderMassProperties::Density(2.0),
            Damping {
                linear_damping: 0.1,
                angular_damping: 0.1,
            }, // Low damping for pendulum
            LockedAxes::TRANSLATION_LOCKED,
            Anchor,
        ))
        .id()
}

fn detect_tick_system(
    mut events: EventWriter<TickEvent>,
    mut query: Query<(&Transform, &mut EscapeWheel)>,
) {
    for (transform, mut wheel) in &mut query {
        let angle = transform.rotation.to_euler(EulerRot::XYZ).2; // Z rotation

        // Handle wrapping -PI to PI
        let mut delta = angle - wheel.last_angle;

        if delta > PI {
            delta -= 2.0 * PI;
        } else if delta < -PI {
            delta += 2.0 * PI;
        }

        wheel.last_angle = angle;
        wheel.cumulative_angle += delta; // Signed sum

        // Check if we passed a tooth
        let tooth_angle = 2.0 * PI / (wheel.teeth as f32);

        if wheel.cumulative_angle.abs() >= tooth_angle {
            // Reset by one tooth worth (keeping the remainder)
            if wheel.cumulative_angle < 0.0 {
                wheel.cumulative_angle += tooth_angle;
            } else {
                wheel.cumulative_angle -= tooth_angle;
            }
            events.send(TickEvent);
        }
    }
}
