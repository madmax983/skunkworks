use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::geometry::{CollisionGroups, Group};
use bevy_rapier2d::prelude::*;
use clockwork_cipher::gear;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Clockwork Cipher".to_string(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(50.0))
        .add_systems(Startup, setup_scene)
        .add_systems(Update, (rotate_drive, update_readout))
        .run();
}

#[derive(Component)]
struct MainDrive;

#[derive(Component)]
struct KeyGear {
    index: usize,
    teeth: usize,
}

#[derive(Component)]
struct Feeler {
    index: usize,
}

#[derive(Component)]
struct CipherReadout;

fn setup_scene(mut commands: Commands) {
    commands.spawn(Camera2dBundle::default());

    let module = 15.0;
    let pressure_angle = 20.0;

    // Driver Gear (Center)
    let driver_teeth = 12;
    let driver_radius = (driver_teeth as f32 * module) / 2.0;

    let driver_id = spawn_visual_gear(
        &mut commands,
        Vec2::ZERO,
        driver_teeth,
        module,
        pressure_angle,
        Color::srgb(0.5, 0.5, 0.5),
        "Driver",
    );
    commands
        .entity(driver_id)
        .insert(MainDrive)
        .insert(ExternalForce::default())
        .insert(ColliderMassProperties::Mass(10.0))
        .insert(CollisionGroups::new(Group::GROUP_1, Group::GROUP_1));

    // Key Gears
    let key_gears: [(usize, f32); 4] = [(13, 0.0), (17, 90.0), (19, 180.0), (23, 270.0)];

    let gear_group = CollisionGroups::new(Group::GROUP_1, Group::GROUP_1);
    let cam_group = CollisionGroups::new(Group::GROUP_2, Group::GROUP_3);
    let feeler_group = CollisionGroups::new(Group::GROUP_3, Group::GROUP_2);

    for (i, (teeth, angle_deg)) in key_gears.iter().enumerate() {
        let angle_rad = angle_deg.to_radians();
        let gear_radius = (*teeth as f32 * module) / 2.0;
        let dist = driver_radius + gear_radius;

        let pos = Vec2::new(dist * angle_rad.cos(), dist * angle_rad.sin());

        let gap_rotation = PI - (PI / *teeth as f32);
        let total_rotation = Quat::from_rotation_z(angle_rad + gap_rotation);

        let id = spawn_visual_gear(
            &mut commands,
            pos,
            *teeth,
            module,
            pressure_angle,
            Color::srgb(1.0, 0.84, 0.0),
            &format!("KeyGear_{}", i),
        );

        commands
            .entity(id)
            .insert(Transform::from_translation(pos.extend(0.0)).with_rotation(total_rotation))
            .insert(KeyGear {
                index: i,
                teeth: *teeth,
            })
            .insert(ColliderMassProperties::Mass(5.0))
            .insert(gear_group);

        // Spawn Cam (Child)
        let cam_radius = 20.0;
        let cam_offset = 15.0;

        commands.entity(id).with_children(|parent| {
            parent.spawn((
                Collider::ball(cam_radius),
                Transform::from_translation(Vec3::new(cam_offset, 0.0, 0.1)),
                cam_group,
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Circle {
                        radius: cam_radius,
                        center: Vec2::ZERO,
                    }),
                    spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
                        cam_offset, 0.0, 0.1,
                    ))),
                    ..default()
                },
                Fill::color(Color::srgb(0.8, 0.2, 0.2)),
            ));
        });

        // Spawn Feeler
        spawn_feeler(&mut commands, pos, i, feeler_group);
    }

    // Readout Text
    commands.spawn((
        TextBundle::from_section(
            "Cipher: INITIALIZING",
            TextStyle {
                font_size: 30.0,
                color: Color::srgb(1.0, 1.0, 1.0),
                ..default()
            },
        )
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(20.0),
            left: Val::Px(20.0),
            ..default()
        }),
        CipherReadout,
    ));
}

fn spawn_feeler(
    commands: &mut Commands,
    gear_pos: Vec2,
    index: usize,
    collision_groups: CollisionGroups,
) {
    let pivot_pos = gear_pos + Vec2::new(0.0, 60.0);
    let arm_length = 80.0;
    let arm_width = 5.0;
    let tip_radius = 5.0;

    let feeler_id = commands
        .spawn((
            SpatialBundle::from_transform(Transform::from_translation(pivot_pos.extend(0.1))),
            RigidBody::Dynamic,
            Damping {
                angular_damping: 1.0,
                ..default()
            },
            Feeler { index },
            Name::new(format!("Feeler_{}", index)),
        ))
        .with_children(|parent| {
            // Visual Arm
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Rectangle {
                        extents: Vec2::new(arm_width, arm_length),
                        origin: RectangleOrigin::CustomCenter(Vec2::new(0.0, -arm_length / 2.0)),
                    }),
                    ..default()
                },
                Fill::color(Color::srgb(0.3, 0.3, 1.0)),
            ));

            // Collider at tip (Child)
            parent.spawn((
                Collider::ball(tip_radius),
                Transform::from_translation(Vec3::new(0.0, -arm_length, 0.0)),
                collision_groups,
            ));
        })
        .id();

    let pivot_body = commands
        .spawn((
            TransformBundle::from_transform(Transform::from_translation(pivot_pos.extend(0.0))),
            RigidBody::Fixed,
        ))
        .id();

    commands.entity(feeler_id).insert(ImpulseJoint::new(
        pivot_body,
        RevoluteJointBuilder::new()
            .local_anchor1(Vec2::ZERO)
            .local_anchor2(Vec2::ZERO),
    ));
}

fn spawn_visual_gear(
    commands: &mut Commands,
    position: Vec2,
    teeth: usize,
    module: f32,
    pressure_angle: f32,
    color: Color,
    name: &str,
) -> Entity {
    let shape_path = gear::generate_gear_path(teeth, module, pressure_angle);
    let collider = gear::generate_gear_collider(teeth, module, pressure_angle);

    commands
        .spawn((
            ShapeBundle {
                path: shape_path,
                spatial: SpatialBundle::from_transform(Transform::from_translation(
                    position.extend(0.0),
                )),
                ..default()
            },
            Fill::color(color),
            Stroke::new(Color::BLACK, 2.0),
            RigidBody::Dynamic,
            collider,
            ColliderMassProperties::Density(1.0),
            LockedAxes::TRANSLATION_LOCKED,
            Damping {
                linear_damping: 0.0,
                angular_damping: 0.5,
            },
            Name::new(name.to_string()),
        ))
        .id()
}

fn rotate_drive(mut query: Query<&mut ExternalForce, With<MainDrive>>, _time: Res<Time>) {
    for mut force in &mut query {
        force.torque = -5000000.0;
    }
}

fn update_readout(
    feeler_query: Query<(&Feeler, &Transform)>,
    mut text_query: Query<&mut Text, With<CipherReadout>>,
) {
    let mut values = [0u8; 4];
    for (feeler, transform) in feeler_query.iter() {
        let angle = transform.rotation.to_euler(EulerRot::XYZ).2;
        // Map angle to byte.
        values[feeler.index % 4] = (angle.abs() * 200.0) as u8;
    }

    if let Ok(mut text) = text_query.get_single_mut() {
        text.sections[0].value = format!(
            "Cipher: {:02X} {:02X} {:02X} {:02X}",
            values[0], values[1], values[2], values[3]
        );
    }
}
