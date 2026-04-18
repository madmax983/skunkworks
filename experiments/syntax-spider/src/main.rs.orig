mod graph;
mod ik;

use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;
use graph::CodeGraph;
use ik::FabrikSolver;
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
        // .add_plugins(RapierDebugRenderPlugin::default()) // Disabled for cleaner look
        .insert_resource(ClearColor(Color::rgb(0.05, 0.05, 0.08)))
        .add_systems(Startup, (setup_world, spawn_spider))
        .add_systems(
            Update,
            (
                camera_follow,
                move_spider,
                update_legs,
                handle_collisions,
                pulse_body,
            ),
        )
        .run();
}

#[derive(Component)]
struct GraphNode {
    path: std::path::PathBuf,
    original_scale: f32,
}

#[derive(Component)]
struct MainCamera;

#[derive(Component)]
struct SpiderBody {
    speed: f32,
    base_scale: f32,
}

#[derive(Component)]
struct SpiderLeg {
    index: usize,
    offset_angle: f32,
    reach_dist: f32,

    // Solver
    solver: FabrikSolver,

    // State
    current_foot_pos: Vec2, // World space
    target_foot_pos: Vec2,  // World space

    foothold_entity: Option<Entity>,

    // Animation
    step_progress: f32, // 0.0 to 1.0. If >= 1.0, grounded.
    step_start_pos: Vec2,
    step_duration: f32,
    cooldown: f32,
}

fn setup_world(mut commands: Commands) {
    commands.spawn((Camera2dBundle::default(), MainCamera));

    // Scan graph
    let root = std::env::current_dir().unwrap();
    let graph = CodeGraph::scan(root, 4); // Increased depth scan

    // Spawn nodes
    for node in graph.nodes {
        let shape = shapes::Circle {
            radius: node.radius,
            center: Vec2::ZERO,
        };

        let color = if node.is_dir {
            Color::rgb(0.2, 0.6, 1.0)
        } else {
            Color::rgb(0.5, 0.5, 0.5)
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
            ActiveEvents::COLLISION_EVENTS,
            GraphNode {
                path: node.path,
                original_scale: node.radius,
            },
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
            Fill::color(Color::rgb(1.0, 0.3, 0.3)),
            Stroke::new(Color::BLACK, 2.0),
            RigidBody::Dynamic,
            Collider::ball(body_radius),
            ActiveEvents::COLLISION_EVENTS,
            Damping {
                linear_damping: 2.0,
                angular_damping: 1.0,
            },
            ExternalForce::default(),
            SpiderBody {
                speed: 1500.0,
                base_scale: 1.0,
            },
        ))
        .id();

    // Legs
    let num_legs = 8;
    for i in 0..num_legs {
        let angle_step = (2.0 * PI) / num_legs as f32;
        let angle = i as f32 * angle_step;

        let reach = 70.0;
        let num_segments = 3;
        let segment_len = reach / 2.0; // Total length > reach to allow slack

        let foot_pos = Vec2::new(angle.cos() * reach, angle.sin() * reach);

        commands
            .spawn((
                ShapeBundle {
                    // Initial path placeholder
                    path: GeometryBuilder::build_as(&shapes::Line(Vec2::ZERO, foot_pos)),
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 9.0),
                        ..default()
                    },
                    ..default()
                },
                Stroke::new(Color::rgb(0.8, 0.8, 0.8), 2.0),
                SpiderLeg {
                    index: i,
                    offset_angle: angle,
                    reach_dist: reach,
                    solver: FabrikSolver::new(Vec2::ZERO, num_segments, segment_len),
                    current_foot_pos: foot_pos,
                    target_foot_pos: foot_pos,
                    foothold_entity: None,
                    step_progress: 1.0,
                    step_start_pos: foot_pos,
                    step_duration: 0.2,
                    cooldown: i as f32 * 0.1, // Stagger initial steps
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
    body_query: Query<(Entity, &GlobalTransform, &Velocity)>,
    rapier_context: Res<RapierContext>,
) {
    let mut rng = rand::thread_rng();

    for (mut leg, mut path, parent) in leg_query.iter_mut() {
        if let Ok((body_entity, body_transform, body_vel)) = body_query.get(parent.get()) {
            let body_pos = body_transform.translation().truncate();
            let (_, rot, _) = body_transform.to_scale_rotation_translation();
            let body_angle = rot.to_euler(EulerRot::XYZ).2;

            // Ideal position relative to body
            let ideal_angle = body_angle + leg.offset_angle;
            let ideal_offset = Vec2::new(ideal_angle.cos(), ideal_angle.sin()) * leg.reach_dist;
            let ideal_pos_world = body_pos + ideal_offset;

            // Check if we need to step
            let dist_to_ideal = leg.current_foot_pos.distance(ideal_pos_world);
            let threshold = 40.0;

            // Logic:
            // If grounded, check if we need to move foot.
            // If stepping, animate.

            if leg.step_progress >= 1.0 {
                // Grounded
                leg.cooldown -= time.delta_seconds();

                if dist_to_ideal > threshold && leg.cooldown <= 0.0 {
                    // Find a foothold near the ideal position
                    // We cast a shape or query intersection near ideal_pos_world
                    let search_radius = 40.0;

                    let mut best_foothold = None;

                    // Query nearby colliders
                    rapier_context.intersections_with_shape(
                        ideal_pos_world,
                        0.0,
                        &Collider::ball(search_radius),
                        QueryFilter::new().exclude_collider(body_entity),
                        |entity| {
                            // Check distance
                            // We don't know exact position of collider center easily without querying Transform component of that entity
                            // But we can just assume the hit is good enough?
                            // Actually, we want to snap to the center of the node if possible.
                            best_foothold = Some(entity);
                            true // Continue? No need to iterate all if we just want one? Let's iterate all to find closest?
                                 // Rapier's callback is simple.
                                 // To find closest, we need access to transforms of these entities.
                                 // For now, just take the first one or valid one.
                        },
                    );

                    // Wait, we need the position of that foothold to snap to it.
                    // We can't get it from Rapier callback easily without a separate Query.
                    // So we will optimistically target the `ideal_pos_world` adjusted by velocity,
                    // but if we had the `GraphNode` query we could snap.
                    // Let's refine target based on velocity first.

                    let velocity = body_vel.linvel;
                    let lead = velocity * 0.3; // Lead the target
                    let search_target = ideal_pos_world + lead;

                    // START STEP
                    leg.step_progress = 0.0;
                    leg.step_start_pos = leg.current_foot_pos;
                    leg.target_foot_pos = search_target; // Default to air step
                    leg.foothold_entity = None;

                    // Try to find a real node to lock onto
                    // We can use a simplified approach: just raycast from body to ideal? No.
                    // We will check if `target_foot_pos` is close to a node in next frames?
                    // Or iterate all nodes? (Too slow).

                    // Let's just step to "Air" for now, and if we land on something (via collision?)
                    // Or better: Use the `rapier_context` properly.
                    // `project_point` finds the closest point on a collider.
                    if let Some((entity, point)) = rapier_context.project_point(
                        search_target,
                        true, // solid
                        QueryFilter::new().exclude_collider(body_entity),
                    ) {
                        if point.is_inside {
                            leg.target_foot_pos = search_target; // Already inside?
                        } else {
                            // If it is close enough
                            if point.point.distance(search_target) < search_radius {
                                leg.target_foot_pos = point.point;
                                leg.foothold_entity = Some(entity);
                            }
                        }
                    }

                    leg.cooldown = 0.1 + rng.gen_range(0.0..0.1);
                }
            } else {
                // Stepping animation
                leg.step_progress += time.delta_seconds() / leg.step_duration;
                if leg.step_progress >= 1.0 {
                    leg.step_progress = 1.0;
                    leg.current_foot_pos = leg.target_foot_pos;
                } else {
                    let t = leg.step_progress;
                    // Ease-in-out
                    let smooth_t = t * t * (3.0 - 2.0 * t);
                    leg.current_foot_pos = leg.step_start_pos.lerp(leg.target_foot_pos, smooth_t);

                    // Lift leg (Visual only, solved by IK)
                    // We can't easily lift in 2D without Z. But we can pretend.
                }
            }

            // If grounded on a moving entity (not implemented yet, nodes are static), update pos.

            // SOLVE IK
            // Convert current_foot_pos (World) to Local (relative to Body)
            // Body transform: T * R
            // Local = R^-1 * (World - T)

            let rel_pos = leg.current_foot_pos - body_pos;
            let cos = (-body_angle).cos();
            let sin = (-body_angle).sin();
            let local_target = Vec2::new(
                rel_pos.x * cos - rel_pos.y * sin,
                rel_pos.x * sin + rel_pos.y * cos,
            );

            leg.solver.solve(local_target);

            // Update Path
            // Joints are in local space.
            let points: Vec<Vec2> = leg.solver.joints.clone();

            // Make it a nice curve? Or just lines.
            // Using a thick stroke with Lyon.
            let line = shapes::Polygon {
                points,
                closed: false,
            };
            *path = GeometryBuilder::build_as(&line);
        }
    }
}

fn pulse_body(time: Res<Time>, mut query: Query<(&mut Transform, &SpiderBody)>) {
    for (mut transform, body) in query.iter_mut() {
        let scale_pulse = 1.0 + (time.elapsed_seconds() * 2.0).sin() * 0.05;
        transform.scale = Vec3::splat(body.base_scale * scale_pulse);
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

            if let Ok(_node) = node_q.get(other) {
                // Log the file path
                // println!("Spider touched: {:?}", node.path);
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

    if let Ok(spider_transform) = spider_q.get_single() {
        let target = spider_transform.translation;
        let speed = 3.0;
        cam_transform.translation.x +=
            (target.x - cam_transform.translation.x) * speed * time.delta_seconds();
        cam_transform.translation.y +=
            (target.y - cam_transform.translation.y) * speed * time.delta_seconds();

        // Zoom out a bit
        cam_transform.scale = Vec3::splat(2.0);
    }
}
