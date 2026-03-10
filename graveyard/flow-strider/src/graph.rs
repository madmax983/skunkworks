use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::prelude::*;

#[derive(Resource, Default)]
pub struct Graph {
    pub nodes: Vec<Vec2>,
    pub edges: Vec<(usize, usize)>,
}

pub struct GraphPlugin;

impl Plugin for GraphPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Graph>()
            .add_systems(Startup, setup_graph);
    }
}

fn setup_graph(mut commands: Commands, mut graph: ResMut<Graph>) {
    // Generate a simple grid graph
    let rows = 5;
    let cols = 8;
    let spacing = 80.0;
    let jitter = 20.0;
    let mut rng = thread_rng();

    for y in 0..rows {
        for x in 0..cols {
            let cx = x as f32 * spacing - (cols as f32 * spacing) / 2.0;
            let cy = y as f32 * spacing - (rows as f32 * spacing) / 2.0;
            let pos = Vec2::new(cx, cy)
                + Vec2::new(
                    rng.gen_range(-jitter..jitter),
                    rng.gen_range(-jitter..jitter),
                );
            graph.nodes.push(pos);
        }
    }

    // Connect neighbors
    for y in 0..rows {
        for x in 0..cols {
            let idx = y * cols + x;
            if x < cols - 1 {
                graph.edges.push((idx, idx + 1));
            }
            if y < rows - 1 {
                graph.edges.push((idx, idx + cols));
            }
        }
    }

    // Render Nodes
    for &pos in &graph.nodes {
        let shape = shapes::Circle {
            radius: 5.0,
            center: Vec2::ZERO,
        };
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                spatial: SpatialBundle::from_transform(Transform::from_translation(
                    pos.extend(0.0),
                )),
                ..default()
            },
            Fill::color(Color::rgb(0.2, 0.2, 0.2)),
            Stroke::new(Color::rgb(0.5, 0.5, 0.5), 1.0),
        ));
    }

    // Render Edges
    for &(start, end) in &graph.edges {
        let start_pos = graph.nodes[start];
        let end_pos = graph.nodes[end];
        let shape = shapes::Line(start_pos, end_pos);
        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                ..default()
            },
            Stroke::new(Color::rgb(0.1, 0.1, 0.1), 2.0),
        ));
    }
}
