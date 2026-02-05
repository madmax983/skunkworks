use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use crate::physics::Gear;
use std::f32::consts::PI;

#[derive(Component)]
pub struct VisualsAdded;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin)
           .add_systems(Update, add_gear_visuals);
    }
}

fn add_gear_visuals(
    mut commands: Commands,
    query: Query<(Entity, &Gear), Without<VisualsAdded>>,
) {
    for (entity, gear) in query.iter() {
        commands.entity(entity).insert(VisualsAdded);

        let color = match gear.name.as_str() {
            "Sun" => Color::srgba(1.0, 0.84, 0.0, 1.0),
            "Moon" => Color::srgba(0.75, 0.75, 0.75, 1.0),
            "Zodiac" => Color::srgba(0.5, 0.0, 0.5, 1.0),
            _ => Color::srgba(0.5, 0.5, 0.5, 1.0),
        };

        // 1. Draw Rim (Main Circle)
        let pitch_circumference = 2.0 * PI * gear.radius;
        let pitch = pitch_circumference / (gear.teeth as f32);
        let tooth_depth = pitch * 0.6;
        let rim_radius = gear.radius - (tooth_depth * 0.6);

        let shape = shapes::Circle {
            radius: rim_radius,
            center: Vec2::ZERO,
        };

        commands.entity(entity).with_children(|parent| {
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    ..default()
                },
                Fill::color(color),
                Stroke::new(Color::BLACK, 1.0),
            ));

            // 2. Draw Teeth
            let tooth_width = pitch * 0.4;

            for i in 0..gear.teeth {
                let angle = (i as f32) * 2.0 * PI / (gear.teeth as f32);
                let dist = gear.radius;

                let x = dist * angle.cos();
                let y = dist * angle.sin();

                let tooth_shape = shapes::Rectangle {
                    extents: Vec2::new(tooth_depth, tooth_width), // width/height
                    origin: RectangleOrigin::Center,
                };

                parent.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&tooth_shape),
                        spatial: SpatialBundle::from_transform(
                            Transform::from_translation(Vec3::new(x, y, 0.1))
                            .with_rotation(Quat::from_rotation_z(angle))
                        ),
                        ..default()
                    },
                    Fill::color(color),
                    Stroke::new(Color::BLACK, 1.0),
                ));
            }

            // 3. Draw Marker (Line to show rotation)
            let marker = shapes::Line(Vec2::ZERO, Vec2::new(gear.radius, 0.0));
             parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&marker),
                    spatial: SpatialBundle::from_transform(Transform::from_xyz(0.0, 0.0, 0.2)),
                    ..default()
                },
                Stroke::new(Color::BLACK, 2.0),
            ));
        });
    }
}
