mod graph;
mod ik;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use graph::CodeGraph;
use rand::Rng;
use std::f32::consts::PI;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Syntax Spider".into(),
                resolution: (1280.0, 720.0).into(),
                ..default()
            }),
            ..default()
        }))
        .add_plugins(ShapePlugin)
        .add_plugins(RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0))
        .add_plugins(RapierDebugRenderPlugin::default())
        .insert_resource(ClearColor(Color::BLACK))
        .add_systems(Startup, (setup_world, spawn_spider))
        .add_systems(
            Update,
            (camera_follow, move_spider, update_legs, handle_collisions),
        )
        .run();
}

#[derive(Component)]
struct GraphNode {
    path: std::path::PathBuf,
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct SpiderBody {
    speed: f32,
}

#[derive(Component)]
struct SpiderLeg {
    index: usize,
    offset_angle: f32,
    reach_dist: f32,
    segment_len: f32,

    // State
    current_foot_pos: Vec2,
    target_foot_pos: Vec2,
    step_progress: f32, // 0.0 to 1.0. If >= 1.0, grounded.
    step_start_pos: Vec2,

    // Timing
    step_duration: f32,
    cooldown: f32,
}

fn setup_world(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // Scan graph
    let root = std::env::current_dir().unwrap();
    let graph = CodeGraph::scan(root, 3);

    // Spawn edges
    for (parent_idx, child_idx) in &graph.edges {
        let start = graph.nodes[*parent_idx].position;
        let end = graph.nodes[*child_idx].position;

        let shape = shapes::Line(start, end);

        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0.0, 0.0, -1.0), // Z=-1 to be behind nodes
                    ..default()
                },
                ..default()
            },
            Stroke::new(Color::rgba(0.3, 0.3, 0.3, 0.5), 2.0),
        ));
    }

    // Spawn nodes
    for node in &graph.nodes {
        let shape = shapes::Circle {
            radius: node.radius,
            center: Vec2::ZERO,
        };

        let color = if node.is_dir {
            Color::rgba(0.2, 0.6, 1.0, 0.8)
        } else {
            Color::rgba(0.5, 0.5, 0.5, 0.5)
        };

        commands.spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shape),
                spatial: SpatialBundle {
                    transform: Transform::from_translation(node.position.extend(0.0)),
                    ..default()
                },
                ..default()
            },
            Fill::color(color),
            Stroke::new(Color::BLACK, 1.0),
            Collider::ball(node.radius),
            ActiveEvents::COLLISION_EVENTS, // Enable collision events
            GraphNode { path: node.path.clone() },
        ));
    }
}

fn spawn_spider(mut commands: Commands) {
    let body_radius = 15.0;

    // Body
    let body_entity = commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::Circle {
                    radius: body_radius,
                    center: Vec2::ZERO,
                }),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 10.0), // Z=10 to be on top
                    ..default()
                },
                ..default()
            },
            Fill::color(Color::CYAN),
            Stroke::new(Color::WHITE, 2.0),
            RigidBody::Dynamic,
            Collider::ball(body_radius),
            ActiveEvents::COLLISION_EVENTS, // Enable collision events
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            ExternalForce::default(),
            SpiderBody { speed: 1000.0 },
        ))
        .id();

    // Legs
    let num_legs = 8;
    for i in 0..num_legs {
        let angle_step = (2.0 * PI) / num_legs as f32;
        let angle = i as f32 * angle_step;

        let reach = 60.0;
        let foot_pos = Vec2::new(angle.cos() * reach, angle.sin() * reach);

        commands
            .spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Line(Vec2::ZERO, foot_pos)),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 9.0),
                        ..default()
                    },
                    ..default()
                },
                Stroke::new(Color::WHITE, 2.0),
                SpiderLeg {
                    index: i,
                    offset_angle: angle,
                    reach_dist: reach,
                    segment_len: 35.0,
                    current_foot_pos: foot_pos,
                    target_foot_pos: foot_pos,
                    step_progress: 1.0,
                    step_start_pos: foot_pos,
                    step_duration: 0.15,
                    cooldown: 0.0, // Randomize start cooldown?
                },
            ))
            .set_parent(body_entity);
    }
}

fn move_spider(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&SpiderBody, &mut ExternalForce)>,
) {
    for (spider, mut force) in query.iter_mut() {
        let mut move_dir = Vec2::ZERO;

        if keys.pressed(KeyCode::KeyW) {
            move_dir.y += 1.0;
        }
        if keys.pressed(KeyCode::KeyS) {
            move_dir.y -= 1.0;
        }
        if keys.pressed(KeyCode::KeyA) {
            move_dir.x -= 1.0;
        }
        if keys.pressed(KeyCode::KeyD) {
            move_dir.x += 1.0;
        }

        if move_dir != Vec2::ZERO {
            move_dir = move_dir.normalize();
            force.force = move_dir * spider.speed;
        } else {
            force.force = Vec2::ZERO;
        }
    }
}

fn update_legs(
    time: Res<Time>,
    mut leg_query: Query<(&mut SpiderLeg, &mut Path, &Parent)>,
    body_query: Query<(&GlobalTransform, &Velocity)>,
    node_query: Query<&Transform, With<GraphNode>>,
) {
    let mut rng = rand::thread_rng();

    for (mut leg, mut path, parent) in leg_query.iter_mut() {
        if let Ok((body_transform, body_vel)) = body_query.get(parent.get()) {
            let body_pos = body_transform.translation().truncate();
            let (_, rot, _) = body_transform.to_scale_rotation_translation();
            let body_angle = rot.to_euler(EulerRot::XYZ).2;

            let ideal_angle = body_angle + leg.offset_angle;
            let ideal_offset = Vec2::new(ideal_angle.cos(), ideal_angle.sin()) * leg.reach_dist;
            let ideal_pos_world = body_pos + ideal_offset;

            let dist = leg.current_foot_pos.distance(ideal_pos_world);

            // Randomize threshold per frame slightly to simulate organic noise?
            // Better: Constant threshold but random cooldown.
            let threshold = 30.0;

            if leg.step_progress >= 1.0 {
                // Grounded
                leg.cooldown -= time.delta_seconds();

                if dist > threshold && leg.cooldown <= 0.0 {
                    // Trigger step
                    leg.step_progress = 0.0;
                    leg.step_start_pos = leg.current_foot_pos;

                    let velocity = body_vel.linvel;
                    let lead = velocity * 0.2;
                    let ideal_target = ideal_pos_world + lead;

                    // Snap to nearest node
                    let mut best_target = ideal_target;
                    let mut min_dist = f32::MAX;
                    let search_radius = 40.0;

                    for node_transform in node_query.iter() {
                        let node_pos = node_transform.translation.truncate();
                        let d = node_pos.distance(ideal_target);
                        if d < search_radius && d < min_dist {
                            min_dist = d;
                            best_target = node_pos;
                        }
                    }

                    leg.target_foot_pos = best_target;

                    // Add cooldown + random noise
                    leg.cooldown = 0.1 + rng.gen_range(0.0..0.1);
                }
            } else {
                // Stepping
                leg.step_progress += time.delta_seconds() / leg.step_duration;
                if leg.step_progress >= 1.0 {
                    leg.step_progress = 1.0;
                    leg.current_foot_pos = leg.target_foot_pos;
                } else {
                    let t = leg.step_progress;
                    let smooth_t = t * t * (3.0 - 2.0 * t);
                    leg.current_foot_pos = leg.step_start_pos.lerp(leg.target_foot_pos, smooth_t);
                }
            }

            // IK Solving
            let rel_pos = leg.current_foot_pos - body_pos;
            let cos = (-body_angle).cos();
            let sin = (-body_angle).sin();
            let local_target = Vec2::new(
                rel_pos.x * cos - rel_pos.y * sin,
                rel_pos.x * sin + rel_pos.y * cos,
            );

            let elbow_dir = Vec2::new(-local_target.y, local_target.x).normalize_or_zero();

            let (elbow, end) = ik::solve_2_bone(
                Vec2::ZERO,
                local_target,
                leg.segment_len,
                leg.segment_len,
                elbow_dir,
            );

            let line = shapes::Polygon {
                points: vec![Vec2::ZERO, elbow, end],
                closed: false,
            };
            *path = GeometryBuilder::build_as(&line);
        }
    }
}

fn handle_collisions(
    mut collision_events: EventReader<CollisionEvent>,
    spider_q: Query<Entity, With<SpiderBody>>,
    node_q: Query<&GraphNode>,
) {
    let spider_entity = if let Ok(e) = spider_q.get_single() {
        e
    } else {
        return;
    };

    for event in collision_events.read() {
        if let CollisionEvent::Started(e1, e2, _) = event {
            let other = if *e1 == spider_entity {
                *e2
            } else if *e2 == spider_entity {
                *e1
            } else {
                continue;
            };

            if let Ok(node) = node_q.get(other) {
                // Log the file path
                println!("Spider touched: {:?}", node.path);
            }
        }
    }
}

fn camera_follow(
    time: Res<Time>,
    mut camera_q: Query<&mut Transform, With<MainCamera>>,
    spider_q: Query<&Transform, (With<SpiderBody>, Without<MainCamera>)>,
) {
    let mut cam_transform = camera_q.single_mut();

    // Follow spider if exists
    if let Ok(spider_transform) = spider_q.get_single() {
        let target = spider_transform.translation;
        let speed = 5.0;
        cam_transform.translation.x +=
            (target.x - cam_transform.translation.x) * speed * time.delta_seconds();
        cam_transform.translation.y +=
            (target.y - cam_transform.translation.y) * speed * time.delta_seconds();
    }
}
