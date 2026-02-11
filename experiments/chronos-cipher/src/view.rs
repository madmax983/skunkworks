use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::mechanism::{Gear, EscapementAnchor, EscapeWheel};
use std::f32::consts::PI;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin)
           .add_systems(Update, (spawn_gear_visuals, spawn_anchor_visuals, rotate_visuals));
    }
}

// Marker for visual entities so we can update them if needed (though children inherit transform usually)
#[derive(Component)]
struct Visual;

fn spawn_gear_visuals(
    mut commands: Commands,
    query: Query<(Entity, &Gear, Option<&EscapeWheel>), Added<Gear>>,
) {
    for (entity, gear, is_escape) in &query {
        let color = if is_escape.is_some() {
            Color::srgba(0.8, 0.7, 0.2, 1.0) // Brass
        } else {
            // Random-ish brass/bronze colors based on teeth
            let hue = 0.1 + (gear.teeth as f32 % 5.0) * 0.02;
            Color::hsla(hue * 360.0, 0.6, 0.5, 1.0)
        };

        let radius = gear.radius;

        commands.entity(entity).with_children(|parent| {
            // Main Disk
            let circle = shapes::Circle {
                radius: radius * 0.85,
                center: Vec2::ZERO,
            };

            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&circle),
                    spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(0.0, 0.0, -0.1))),
                    ..default()
                },
                Fill::color(color),
                Stroke::new(Color::BLACK, 2.0),
                Visual,
            ));

            // Spokes (for style)
            let spoke_count = 5;
            for i in 0..spoke_count {
                 let angle = (i as f32) * 2.0 * PI / (spoke_count as f32);
                 let spoke = shapes::Rectangle {
                    extents: Vec2::new(radius * 0.6, radius * 0.1),
                    origin: RectangleOrigin::Center,
                 };

                 parent.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&spoke),
                        spatial: SpatialBundle::from_transform(
                            Transform::from_translation(Vec3::new(
                                radius * 0.4 * angle.cos(),
                                radius * 0.4 * angle.sin(),
                                0.1 // On top
                            )).with_rotation(Quat::from_rotation_z(angle))
                        ),
                        ..default()
                    },
                    Fill::color(Color::srgba(0.0, 0.0, 0.0, 0.3)), // Darker/Hollow look
                    Visual,
                 ));
            }

            // Teeth
            let tooth_count = gear.teeth;
            let module = gear.module;
            let pitch = PI * module;
            let tooth_width = pitch * 0.45;
            let tooth_height = module * 0.8;

            for i in 0..tooth_count {
                let angle = (i as f32) * 2.0 * PI / (tooth_count as f32);
                let dist = radius;

                let tooth_shape = shapes::Rectangle {
                    extents: Vec2::new(tooth_height, tooth_width), // x is radial here because we rotate
                    origin: RectangleOrigin::Center,
                };

                let mut t = Transform::from_translation(Vec3::new(
                    dist * angle.cos(),
                    dist * angle.sin(),
                    0.0,
                ));
                t.rotate_z(angle);

                // If escape wheel, skew teeth
                if is_escape.is_some() {
                    t.rotate_z(-0.4);
                }

                parent.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&tooth_shape),
                        spatial: SpatialBundle::from_transform(t),
                        ..default()
                    },
                    Fill::color(color),
                    Visual,
                ));
            }
        });
    }
}

fn spawn_anchor_visuals(
    mut commands: Commands,
    query: Query<Entity, Added<EscapementAnchor>>,
) {
    for entity in &query {
        let color = Color::srgba(0.7, 0.7, 0.8, 1.0); // Steel

        commands.entity(entity).with_children(|parent| {
            // Simplified visual representation matching the physics shapes roughly

            // Pendulum Rod
            let rod = shapes::Rectangle {
                extents: Vec2::new(0.5, 15.0),
                origin: RectangleOrigin::Center,
            };
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&rod),
                    spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(0.0, 15.0, 0.0))),
                    ..default()
                },
                Fill::color(color),
                Stroke::new(Color::BLACK, 1.0),
                Visual,
            ));

            // Bob
            let bob = shapes::Circle {
                radius: 3.0,
                center: Vec2::ZERO,
            };
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&bob),
                    spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(0.0, 30.0, 0.1))),
                    ..default()
                },
                Fill::color(Color::srgba(0.9, 0.8, 0.2, 1.0)), // Gold Bob
                Stroke::new(Color::BLACK, 1.0),
                Visual,
            ));

            // Anchor Arms
            // Just a cross bar for now
            let bar = shapes::Rectangle {
                extents: Vec2::new(10.0, 1.0),
                origin: RectangleOrigin::Center,
            };
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&bar),
                    spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(0.0, 0.0, 0.0))),
                    ..default()
                },
                Fill::color(color),
                Visual,
            ));
        });
    }
}

fn rotate_visuals(
    mut _gizmos: Gizmos,
    _query: Query<&Transform, With<Gear>>,
) {
    // We don't need to manually rotate visuals if they are children of the physics entity.
    // The physics engine updates the parent Transform, and children inherit it.
}
