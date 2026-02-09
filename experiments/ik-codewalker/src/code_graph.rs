use crate::components::*;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use rand::Rng;

pub struct CodeGraphPlugin;

impl Plugin for CodeGraphPlugin {
    fn build(&self, app: &mut App) {
        // ShapePlugin is already added in main
        app.add_systems(Startup, setup_code_graph);
    }
}

#[derive(Component)]
pub struct CodeGraph {
    pub root: Entity,
    pub nodes: Vec<Entity>,
}

fn setup_code_graph(mut commands: Commands) {
    let mut rng = rand::thread_rng();

    // Generate a simple tree structure
    let root_pos = Vec2::new(0.0, 300.0);

    let root = spawn_node(&mut commands, root_pos, NodeType::Root, "main".to_string());

    let mut nodes = vec![root];
    // Queue: (Entity, Position, Depth)
    let mut queue = vec![(root, root_pos, 0)];

    // BFS generation
    let mut head = 0;
    while head < queue.len() {
        let (parent, parent_pos, depth) = queue[head];
        head += 1;

        if depth >= 4 {
            continue;
        }

        let num_children = rng.gen_range(1..=3);

        // Spread children angle based on parent position to avoid crossing lines
        let base_angle = -std::f32::consts::PI / 2.0;
        let spread = std::f32::consts::PI / 2.0;

        for i in 0..num_children {
            // Random angle within a cone
            let angle_offset = rng.gen_range(-spread / 2.0..spread / 2.0);
            let angle = base_angle + angle_offset;

            let length = rng.gen_range(80.0..150.0);

            let offset = Vec2::new(angle.cos(), angle.sin()) * length;
            let pos = parent_pos + offset;

            let node_type = if depth == 3 {
                if rng.gen_bool(0.5) {
                    NodeType::Variable
                } else {
                    NodeType::ControlFlow
                }
            } else {
                NodeType::Function
            };

            let name = format!("node_{}_{}", depth, i);

            let child = spawn_node(&mut commands, pos, node_type, name);
            nodes.push(child);
            queue.push((child, pos, depth + 1));

            // Draw connection line
            let shape = shapes::Line(parent_pos, pos);
            commands.spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shape),
                    ..default()
                },
                Stroke::new(Color::GRAY, 2.0),
            ));
        }
    }

    commands.spawn(CodeGraph { root, nodes });
}

fn spawn_node(commands: &mut Commands, pos: Vec2, node_type: NodeType, name: String) -> Entity {
    let color = match node_type {
        NodeType::Root => Color::RED,
        NodeType::Function => Color::BLUE,
        NodeType::Variable => Color::GREEN,
        NodeType::ControlFlow => Color::YELLOW,
    };

    let shape = shapes::Circle {
        radius: 10.0,
        center: Vec2::ZERO,
    };

    commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                spatial: SpatialBundle::from_transform(Transform::from_translation(
                    pos.extend(0.0),
                )),
                ..default()
            },
            Fill::color(color),
            CodeNode { name, node_type },
        ))
        .id()
}
