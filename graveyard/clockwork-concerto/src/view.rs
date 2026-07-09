use crate::{
    cpu::{CpuState, Program},
    mechanism::{Anchor, EscapeWheel},
};
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use std::f32::consts::PI;

pub struct ViewPlugin;

impl Plugin for ViewPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                spawn_gear_visuals,
                spawn_anchor_visuals,
                draw_cylinder_system,
            ),
        );
    }
}

fn spawn_gear_visuals(
    mut commands: Commands,
    query: Query<(Entity, &EscapeWheel), Added<EscapeWheel>>,
) {
    for (entity, wheel) in &query {
        let radius = wheel.radius;
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
                Stroke::new(Color::BLACK, 0.1),
            ));

            let tooth_width = 0.5; // Visual
            let tooth_height = 1.0;

            for i in 0..wheel.teeth {
                let angle = (i as f32) * 2.0 * PI / (wheel.teeth as f32);
                let dist = radius;

                let tooth_shape = shapes::Rectangle {
                    extents: Vec2::new(tooth_height, tooth_width),
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

fn draw_cylinder_system(mut gizmos: Gizmos, cpu_query: Query<&CpuState>, program: Res<Program>) {
    if let Ok(state) = cpu_query.get_single() {
        let prog_len = program.0.len();
        if prog_len == 0 {
            return;
        }

        // Draw Cylinder at (-15, 0)
        let pos = Vec2::new(-15.0, 0.0);
        let radius = 8.0;

        gizmos.circle_2d(pos, radius, Color::WHITE);

        let angle_step = 2.0 * PI / (prog_len as f32);

        for (i, instr) in program.0.iter().enumerate() {
            let rel_idx = state.pc as i32 - i as i32;
            let angle = rel_idx as f32 * angle_step;

            let pin_pos = pos + Vec2::new(angle.cos() * radius, angle.sin() * radius);

            let color = match instr {
                crate::cpu::Instruction::Note(_) => Color::srgb(0.0, 1.0, 0.0), // Green for Note
                crate::cpu::Instruction::Jmp(_) => Color::srgb(1.0, 0.0, 0.0),  // Red for Jump
                _ => Color::srgb(0.5, 0.5, 0.5),                                // Gray
            };

            gizmos.circle_2d(pin_pos, 0.5, color);
            gizmos.line_2d(pos, pin_pos, color.with_alpha(0.3));
        }

        // Draw "Play Head" (Comb)
        gizmos.line_2d(
            pos + Vec2::new(radius, -2.0),
            pos + Vec2::new(radius + 2.0, 0.0),
            Color::srgb(1.0, 0.84, 0.0),
        );
    }
}
