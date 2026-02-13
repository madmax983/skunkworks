use crate::{mechanism, Anchor, EscapeWheel, TickEvent};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::f32::consts::PI;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin)
            .add_systems(Startup, spawn_tick_indicator)
            .add_systems(Update, (spawn_gear_visuals, spawn_anchor_visuals, animate_tick_indicator));
    }
}

#[derive(Component)]
struct TickIndicator {
    timer: Timer,
}

fn spawn_tick_indicator(mut commands: Commands) {
    commands.spawn((
        TickIndicator {
            timer: Timer::from_seconds(0.1, TimerMode::Once),
        },
        ShapeBundle {
            path: GeometryBuilder::build_as(&shapes::Circle {
                radius: 2.0,
                ..default()
            }),
            spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
                -15.0, 15.0, 0.0,
            ))),
            ..default()
        },
        Fill::color(Color::srgb(0.5, 0.5, 0.5)),
        Stroke::new(Color::BLACK, 0.1),
    ));
}

fn animate_tick_indicator(
    mut events: EventReader<TickEvent>,
    mut query: Query<(&mut TickIndicator, &mut Fill)>,
    time: Res<Time>,
) {
    let mut ticked = false;
    for _ in events.read() {
        ticked = true;
    }

    for (mut indicator, mut fill) in &mut query {
        if ticked {
            indicator.timer.reset();
            *fill = Fill::color(Color::srgb(0.0, 1.0, 0.0));
        }

        indicator.timer.tick(time.delta());
        if indicator.timer.finished() {
            *fill = Fill::color(Color::srgb(0.5, 0.5, 0.5));
        }
    }
}

fn spawn_gear_visuals(
    mut commands: Commands,
    query: Query<(Entity, &EscapeWheel), Added<EscapeWheel>>,
) {
    for (entity, wheel) in &query {
        let radius = wheel.radius; // Use the actual radius from component

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
            let circumference = 2.0 * PI * radius;
            let pitch = circumference / (wheel.teeth as f32);
            let tooth_width = pitch * 0.8;
            let tooth_height = 1.2;

            let tooth_points = mechanism::get_tooth_points(tooth_height, tooth_width);
            let tooth_poly = shapes::Polygon {
                points: tooth_points,
                closed: true,
            };

            for i in 0..wheel.teeth {
                let angle = (i as f32) * 2.0 * PI / (wheel.teeth as f32);
                let dist = radius - 0.2; // Embed slightly

                parent.spawn((
                    ShapeBundle {
                        path: GeometryBuilder::build_as(&tooth_poly),
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

fn spawn_anchor_visuals(mut commands: Commands, query: Query<(Entity, &Anchor), Added<Anchor>>) {
    for (entity, anchor) in &query {
        let color = Color::srgba(0.7, 0.7, 0.8, 1.0); // Steel

        let wheel_radius = anchor.wheel_radius;
        let span_teeth = anchor.span_teeth;
        let pitch_angle = 2.0 * PI / (anchor.wheel_teeth as f32);

        let half_span_angle = (span_teeth * pitch_angle) / 2.0;
        let safe_angle = half_span_angle.clamp(0.1, 1.5);
        let arm_length = wheel_radius * safe_angle.tan();
        let arm_angle = PI / 2.0 - safe_angle;

        let px = arm_length * arm_angle.sin();
        let py = -arm_length * arm_angle.cos();

        commands.entity(entity).with_children(|parent| {
            let pallet_size = Vec2::new(0.4, 0.8);

            // Left Pallet
            draw_rect(
                parent,
                Vec2::new(-px, py),
                0.5,
                pallet_size,
                color,
            );
            // Right Pallet
            draw_rect(
                parent,
                Vec2::new(px, py),
                -0.5,
                pallet_size,
                color,
            );

            // Arms (visual only, can use rects)
            let arm_size = Vec2::new(0.2, arm_length);
            // Left Arm
            draw_rect(
                parent,
                Vec2::new(-px/2.0, py/2.0),
                arm_angle,
                arm_size,
                color,
            );
            // Right Arm
            draw_rect(
                parent,
                Vec2::new(px/2.0, py/2.0),
                -arm_angle,
                arm_size,
                color,
            );

            // Rod
            draw_rect(
                parent,
                Vec2::new(0.0, -6.0),
                0.0,
                Vec2::new(0.4, 12.0),
                color,
            );

            // Bob
            parent.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Circle {
                        radius: 1.5,
                        ..default()
                    }),
                    spatial: SpatialBundle::from_transform(Transform::from_translation(Vec3::new(
                        0.0, -12.0, 0.1,
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
