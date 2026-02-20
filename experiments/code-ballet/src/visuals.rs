use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
struct Visualized;

pub struct VisualsPlugin;

impl Plugin for VisualsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(ShapePlugin)
           .add_systems(Update, add_visuals);
    }
}

fn add_visuals(
    mut commands: Commands,
    query: Query<(Entity, &Collider), Without<Visualized>>,
) {
    for (entity, collider) in query.iter() {
        // Only visualize things that are "visible" conceptually?
        // Actually everything with a collider is part of the world.

        let mut color = Color::rgba(0.0, 1.0, 1.0, 0.3);
        let mut stroke_color = Color::WHITE;

        // Try to distinguish based on other components?
        // Cannot query other components easily here without specific queries.
        // But for now, generic visualization is fine.

        if let Some(cuboid) = collider.as_cuboid() {
            let he = cuboid.half_extents();
            let shape = shapes::Rectangle {
                extents: Vec2::new(he.x * 2.0, he.y * 2.0),
                origin: RectangleOrigin::Center,
            };
             let shape_entity = commands.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    ..default()
                },
                Stroke::new(stroke_color, 1.0),
                Fill::color(color),
            )).id();
            commands.entity(entity).push_children(&[shape_entity]).insert(Visualized);
        } else if let Some(ball) = collider.as_ball() {
            let radius = ball.radius();
             let shape = shapes::Circle {
                radius,
                ..default()
            };
             let shape_entity = commands.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    ..default()
                },
                Stroke::new(stroke_color, 1.0),
                Fill::color(color),
            )).id();
            commands.entity(entity).push_children(&[shape_entity]).insert(Visualized);
        }
    }
}
