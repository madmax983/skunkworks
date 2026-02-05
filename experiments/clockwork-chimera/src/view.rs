use crate::mechanism::{Anchor, EscapeWheel};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::f32::consts::PI;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin)
            .add_systems(Update, (spawn_gear_visuals, spawn_anchor_visuals));
    }
}

fn spawn_gear_visuals(
    mut commands: Commands,
    query: Query<(Entity, &EscapeWheel), Added<EscapeWheel>>,
) {
    for (entity, wheel) in &query {
        let radius = 3.0;

        let color = Color::srgba(0.8, 0.6, 0.2, 1.0); // Brass

        // Rim
        let rim = shapes::Circle {
            radius: radius - 0.5,
            ..default()
        };

        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&rim),
                    ..default()
                },
                Fill::color(color),
                Stroke::new(Color::BLACK, 0.05),
            ));

            // Teeth
            let _pitch = (2.0 * PI * radius) / (wheel.teeth as f32);
            let tooth_width = 0.4; // Matching mechanism (cuboid 0.2 width)
            let tooth_height = 1.0; // Matching mechanism (cuboid 0.5 height * 2)

            for i in 0..wheel.teeth {
                let angle = (i as f32) * 2.0 * PI / (wheel.teeth as f32);
                let dist = radius;

                let tooth_shape = shapes::Rectangle {
                    extents: Vec2::new(tooth_height, tooth_width), // x=height(radial), y=width
                    origin: RectangleOrigin::Center,
                };

                parent.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&tooth_shape),
                        spatial: SpatialBundle::from_transform(
                            Transform::from_translation(Vec3::new(
                                dist * angle.cos(),
                                dist * angle.sin(),
                                0.1,
                            ))
                            .with_rotation(Quat::from_rotation_z(angle)),
                        ),
                        ..default()
                    },
                    Fill::color(color),
                    Stroke::new(Color::BLACK, 0.05),
                ));
            }
        });
    }
}

fn spawn_anchor_visuals(mut commands: Commands, query: Query<Entity, Added<Anchor>>) {
    for entity in &query {
        let color = Color::srgba(0.7, 0.7, 0.8, 1.0); // Steel

        commands.entity(entity).with_children(|parent| {
            // Left Pallet
            draw_rect(
                parent,
                Vec2::new(-1.5, -2.0),
                0.5,
                Vec2::new(0.4, 0.4),
                color,
            );
            // Right Pallet
            draw_rect(
                parent,
                Vec2::new(1.5, -2.0),
                -0.5,
                Vec2::new(0.4, 0.4),
                color,
            );

            // Arms
            draw_rect(
                parent,
                Vec2::new(-2.5, -1.5),
                0.5,
                Vec2::new(0.4, 4.0),
                color,
            );
            draw_rect(
                parent,
                Vec2::new(2.5, -1.5),
                -0.5,
                Vec2::new(0.4, 4.0),
                color,
            );

            // Rod
            draw_rect(
                parent,
                Vec2::new(0.0, -8.0),
                0.0,
                Vec2::new(0.4, 16.0),
                color,
            );

            // Bob
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Circle {
                        radius: 2.0,
                        ..default()
                    }),
                    spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
                        0.0, -16.0, 0.1,
                    ))),
                    ..default()
                },
                Fill::color(color),
                Stroke::new(Color::BLACK, 0.05),
            ));
        });
    }
}

fn draw_rect(parent: &mut ChildBuilder, pos: Vec2, angle: f32, size: Vec2, color: Color) {
    let rect = shapes::Rectangle {
        extents: size,
        origin: RectangleOrigin::Center,
    };
    parent.spawn((
        ShapeBundle {
            path: GeometryBuilder::build_as(&rect),
            spatial: SpatialBundle::from_transform(
                Transform::from_translation(pos.extend(0.1))
                    .with_rotation(Quat::from_rotation_z(angle)),
            ),
            ..default()
        },
        Fill::color(color),
        Stroke::new(Color::BLACK, 0.05),
    ));
}
