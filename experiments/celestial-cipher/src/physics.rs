use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use std::f32::consts::PI;

#[derive(Component)]
pub struct Gear {
    pub teeth: usize,
    pub radius: f32,
    pub name: String,
}

#[derive(Component)]
pub struct ReadHead;

pub struct PhysicsPlugin;

impl Plugin for PhysicsPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_mechanism)
           .add_systems(Update, drive_system);
    }
}

pub fn spawn_gear(
    commands: &mut Commands,
    position: Vec2,
    teeth: usize,
    radius: f32,
    name: &str,
) -> Entity {
    let mut shapes = Vec::new();

    // Calculate module from radius and teeth
    // Module = 2 * Radius / Teeth
    // Tooth Height usually ~ 2.25 * Module
    // But for this simulation, we simplify.
    // Let's assume 'radius' is the Pitch Radius.

    let pitch_circumference = 2.0 * PI * radius;
    let pitch = pitch_circumference / (teeth as f32);

    let tooth_depth = pitch * 0.6;
    let tooth_width = pitch * 0.4;

    // The main disk should be below the tooth root.
    // Root radius = Pitch Radius - Dedendum (approx 1.25 * module? or just half tooth depth)
    let rim_radius = radius - (tooth_depth * 0.6);

    shapes.push((Vect::ZERO, 0.0, Collider::ball(rim_radius)));

    let tooth_shape = Collider::cuboid(tooth_depth / 2.0, tooth_width / 2.0);

    for i in 0..teeth {
        let angle = (i as f32) * 2.0 * PI / (teeth as f32);
        // We place the center of the tooth cuboid at radius distance?
        // No, if we want pitch circle at 'radius', the tooth should span across it.
        // Cuboid center at `radius`.
        let dist = radius;
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
            ColliderMassProperties::Density(5.0), // Heavy gears
            Damping {
                linear_damping: 0.0,
                angular_damping: 0.1, // Slight drag
            },
            Velocity::default(),
            LockedAxes::TRANSLATION_LOCKED, // Pinned pivot
            Gear {
                teeth,
                radius,
                name: name.to_string(),
            },
        ))
        .id()
}

pub fn setup_mechanism(mut commands: Commands) {
    // Sun Gear: 13 teeth (Input)
    let sun_teeth = 13;
    let sun_radius = 30.0;
    let sun_pos = Vec2::new(0.0, 0.0);

    // Module M = 2*R/N = 60/13 = 4.615
    let module = (2.0 * sun_radius) / (sun_teeth as f32);

    let _sun = spawn_gear(&mut commands, sun_pos, sun_teeth, sun_radius, "Sun");

    // Moon Gear: 17 teeth
    let moon_teeth = 17;
    let moon_radius = (module * moon_teeth as f32) / 2.0;

    // Separation: sum of radii + clearance
    let dist_sm = sun_radius + moon_radius + 2.0;
    let moon_pos = sun_pos + Vec2::new(dist_sm, 0.0);

    let _moon = spawn_gear(&mut commands, moon_pos, moon_teeth, moon_radius, "Moon");

    // Zodiac Gear: 23 teeth
    let zodiac_teeth = 23;
    let zodiac_radius = (module * zodiac_teeth as f32) / 2.0;

    // Separation: Moon -> Zodiac
    // Place it below Moon
    let dist_mz = moon_radius + zodiac_radius + 2.0;
    let zodiac_pos = moon_pos + Vec2::new(0.0, -dist_mz);

    let _zodiac = spawn_gear(&mut commands, zodiac_pos, zodiac_teeth, zodiac_radius, "Zodiac");

    // Read Head Sensor (Conceptual)
    commands.spawn((
        SpatialBundle::from_transform(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))),
        ReadHead,
    ));
}

// System to drive the Sun gear
fn drive_system(mut query: Query<(&mut Velocity, &Gear)>) {
    for (mut vel, gear) in query.iter_mut() {
        if gear.name == "Sun" {
            // Constant Angular Velocity Motor
            // We force it to spin at 1 rad/s approx
            // But to be "Physically Accurate", we should apply Torque.
            // If we set angvel directly, it's a kinematic override (infinite torque).
            // Let's use a P-controller or just set it for simplicity.
            vel.angvel = 2.0;
        }
    }
}
