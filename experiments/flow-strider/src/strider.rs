use crate::graph::Graph;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

#[derive(Component)]
pub struct Strider {
    pub target_node: usize,
}

#[derive(Component)]
pub struct Limb {
    pub index: usize,
    pub foot_pos: Vec2,
    pub start_foot_pos: Vec2, // For interpolation
    pub target_foot_pos: Vec2,
    pub knee_pos: Vec2, // Calculated control point (local space)
    pub state: LimbState,
    pub lift_height: f32,
}

#[derive(PartialEq, Clone, Copy)]
pub enum LimbState {
    Stance,
    Lift(f32), // 0.0 to 1.0 progress
}

pub struct StriderPlugin;

impl Plugin for StriderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_strider)
            .add_systems(Update, update_limbs_render);
    }
}

fn spawn_strider(mut commands: Commands, graph: Res<Graph>) {
    if graph.nodes.is_empty() {
        return;
    }

    let start_node = 0;
    let start_pos = graph.nodes[start_node];

    // Spawn Body
    let body = commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: 10.0,
                    center: Vec2::ZERO,
                }),
                spatial: SpatialBundle::from_transform(Transform::from_translation(
                    start_pos.extend(1.0),
                )),
                ..default()
            },
            Fill::color(Color::rgb(0.8, 0.2, 0.2)),
            Strider {
                target_node: start_node,
            },
        ))
        .id();

    // Spawn 4 Limbs as children
    for i in 0..4 {
        let offset = match i {
            0 => Vec2::new(-30.0, 30.0),  // FL
            1 => Vec2::new(30.0, 30.0),   // FR
            2 => Vec2::new(-30.0, -30.0), // BL
            3 => Vec2::new(30.0, -30.0),  // BR
            _ => Vec2::ZERO,
        };
        let foot_pos = start_pos + offset;

        let limb = commands
            .spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Line(Vec2::ZERO, Vec2::ZERO)), // Placeholder
                    spatial: SpatialBundle::default(), // Relative to body
                    ..default()
                },
                Stroke::new(Color::rgb(0.8, 0.5, 0.5), 2.0),
                Limb {
                    index: i,
                    foot_pos,
                    start_foot_pos: foot_pos,
                    target_foot_pos: foot_pos,
                    knee_pos: Vec2::ZERO,
                    state: LimbState::Stance,
                    lift_height: 0.0,
                },
            ))
            .id();

        commands.entity(body).add_child(limb);
    }
}

fn update_limbs_render(
    mut limbs: Query<(&Parent, &mut Limb, &mut Path)>,
    transforms: Query<&GlobalTransform>,
) {
    for (parent, mut limb, mut path) in limbs.iter_mut() {
        if let Ok(body_transform) = transforms.get(parent.get()) {
            let body_pos = body_transform.translation().truncate();

            // Calculate local foot position
            let local_foot = limb.foot_pos - body_pos;

            // Calculate knee (Control Point)
            // Quadratic Bezier: P0 (0,0), P1 (Knee), P2 (Foot)
            // Knee is pushed outwards perpendicular to the leg vector
            let mid = local_foot / 2.0;
            let perp = Vec2::new(-local_foot.y, local_foot.x).normalize_or_zero();
            // Flip perp based on side?
            // If local_foot.x is positive (Right side), we want knee to go right.
            // If perp points right, good.
            let knee_offset = 30.0;
            let knee = mid + perp * knee_offset * (if limb.index % 2 == 0 { -1.0 } else { 1.0 });

            limb.knee_pos = knee;

            // Build path
            let mut path_builder = PathBuilder::new();
            path_builder.move_to(Vec2::ZERO);
            path_builder.quadratic_bezier_to(knee, local_foot);
            *path = path_builder.build();
        }
    }
}
