use crate::ik::TwoBoneSolver;
use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;
use bevy_rapier2d::prelude::*;

#[derive(Component)]
pub struct StriderBody {
    pub speed: f32,
    pub turn_speed: f32,
    pub target_heading: f32, // Smooth turning
}

#[derive(Component)]
pub struct StriderLeg {
    pub index: usize,
    pub offset_angle: f32, // Angle relative to body forward (0 is right, PI/2 is front?)
    pub root_offset_dist: f32, // Distance from body center to leg root

    // IK
    pub solver: TwoBoneSolver,

    // State
    pub state: LegState,
    pub current_foot_pos: Vec2, // World space
    pub target_foot_pos: Vec2,  // World space target for step
    pub start_step_pos: Vec2,   // Where step started
    pub step_progress: f32,     // 0.0 to 1.0
}

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum LegState {
    Grounded,
    Stepping,
}

pub struct StriderPlugin;

impl Plugin for StriderPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_strider)
            .add_systems(Update, (strider_movement, leg_logic));
    }
}

fn spawn_strider(mut commands: Commands) {
    let body_radius = 20.0;

    // Body
    let body_entity = commands
        .spawn((
            ShapeBundle {
                path: GeometryBuilder::build_as(&shapes::RegularPolygon {
                    sides: 6,
                    feature: shapes::RegularPolygonFeature::Radius(body_radius),
                    ..default()
                }),
                spatial: SpatialBundle {
                    transform: Transform::from_xyz(0.0, 0.0, 10.0),
                    ..default()
                },
                ..default()
            },
            Fill::color(Color::rgb(0.8, 0.2, 0.2)),
            Stroke::new(Color::BLACK, 2.0),
            RigidBody::Dynamic,
            Collider::ball(body_radius),
            Damping {
                linear_damping: 5.0,
                angular_damping: 5.0,
            },
            Velocity::default(),
            ExternalForce::default(),
            StriderBody {
                speed: 4000.0,
                turn_speed: 10000.0, // Torque
                target_heading: 0.0,
            },
        ))
        .id();

    // Legs
    let num_legs = 6;
    for i in 0..num_legs {
        // Hexapod layout:
        // 0: Right Front (PI/6)
        // 1: Right Middle (-PI/2) ? No.
        // Let's do standard hexapod:
        // Front Right: 30 deg
        // Mid Right: -90 deg
        // Back Right: -150 deg
        // Back Left: 150 deg
        // Mid Left: 90 deg
        // Front Left: 30 deg? No.

        // Let's just distribute evenly for simplicity first: 0, 60, 120, 180, 240, 300
        let angle = (i as f32) * std::f32::consts::PI / 3.0;

        // Initial foot pos
        let root_offset = 20.0;
        let reach = 80.0;
        let foot_pos = Vec2::new(angle.cos(), angle.sin()) * (root_offset + reach * 0.5);

        commands
            .spawn((
                ShapeBundle {
                    path: GeometryBuilder::build_as(&shapes::Line(Vec2::ZERO, Vec2::X * 10.0)), // Placeholder
                    spatial: SpatialBundle {
                        transform: Transform::from_xyz(0.0, 0.0, 9.0),
                        ..default()
                    },
                    ..default()
                },
                Stroke::new(Color::rgb(0.7, 0.7, 0.7), 4.0),
                StriderLeg {
                    index: i,
                    offset_angle: angle,
                    root_offset_dist: root_offset,
                    solver: TwoBoneSolver::new(reach * 0.6, reach * 0.6),
                    state: LegState::Grounded,
                    current_foot_pos: foot_pos, // Will be corrected
                    target_foot_pos: foot_pos,
                    start_step_pos: foot_pos,
                    step_progress: 0.0,
                },
            ))
            .set_parent(body_entity);
    }
}

fn strider_movement(
    keys: Res<ButtonInput<KeyCode>>,
    mut query: Query<(&mut ExternalForce, &Transform, &StriderBody)>,
) {
    for (mut force, _transform, body) in query.iter_mut() {
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
            force.force = move_dir * body.speed;
        } else {
            force.force = Vec2::ZERO;
        }
    }
}

fn leg_logic(
    time: Res<Time>,
    mut leg_query: Query<(&mut StriderLeg, &mut Path, &Parent)>,
    body_query: Query<(&GlobalTransform, &Velocity)>,
    rapier_context: Res<RapierContext>,
) {
    // Count how many legs are stepping to limit concurrency
    let mut stepping_count = leg_query
        .iter()
        .filter(|(l, _, _)| l.state == LegState::Stepping)
        .count();
    let max_stepping = 3; // Tripod gait allows 3

    for (mut leg, mut path, parent) in leg_query.iter_mut() {
        if let Ok((body_transform, body_vel)) = body_query.get(parent.get()) {
            let body_pos = body_transform.translation().truncate();
            let (_, rot, _) = body_transform.to_scale_rotation_translation();
            let body_angle = rot.to_euler(EulerRot::XYZ).2;

            // Calculate ideal foot position
            // Body Angle + Leg Offset
            let mounting_angle = body_angle + leg.offset_angle;
            let mounting_pos = body_pos
                + Vec2::new(mounting_angle.cos(), mounting_angle.sin()) * leg.root_offset_dist;

            // Ideal foot pos is some distance out from mounting point, potentially lead by velocity
            let ideal_dist = 60.0;
            let ideal_pos_world =
                mounting_pos + Vec2::new(mounting_angle.cos(), mounting_angle.sin()) * ideal_dist;

            // Add velocity prediction
            let velocity_lead = body_vel.linvel * 0.3;
            let target_search_center = ideal_pos_world + velocity_lead;

            let dist_current = leg.current_foot_pos.distance(ideal_pos_world);

            match leg.state {
                LegState::Grounded => {
                    // Check if we need to step
                    let threshold = 40.0;
                    if dist_current > threshold {
                        // Attempt to step if budget allows
                        // Or if we are REALLY far (panic step)
                        let panic = dist_current > threshold * 1.5;

                        if panic || stepping_count < max_stepping {
                            // Find a foothold
                            // Raycast or check intersection?
                            // We want to land ON a node.
                            // Let's verify if there is a node near `target_search_center`

                            // Use Rapier to project point
                            let filter = QueryFilter::new().exclude_sensors(); // Nodes are solid balls

                            // We search in a radius
                            let mut best_pos = target_search_center;

                            // Simplified: Just use Rapier Project Point on the closest non-body collider?
                            // But body is collider too. We need to exclude body.
                            // We can't easily exclude specific entity without knowing its ID here unless passed down.
                            // But legs are children of body, so `parent` is body.
                            let body_entity = parent.get();
                            let filter = filter.exclude_collider(body_entity);

                            if let Some((_entity, projection)) =
                                rapier_context.project_point(target_search_center, true, filter)
                            {
                                // If the point is inside or close, snap to it?
                                // Actually, `project_point` gives closest point on collider surface.
                                // If inside, point is same.
                                best_pos = projection.point;
                            }

                            // Start stepping
                            leg.state = LegState::Stepping;
                            leg.start_step_pos = leg.current_foot_pos;
                            leg.target_foot_pos = best_pos;
                            leg.step_progress = 0.0;
                            stepping_count += 1;
                        }
                    }
                }
                LegState::Stepping => {
                    leg.step_progress += time.delta_seconds() * 5.0; // Speed of step
                    if leg.step_progress >= 1.0 {
                        leg.step_progress = 1.0;
                        leg.current_foot_pos = leg.target_foot_pos;
                        leg.state = LegState::Grounded;
                    } else {
                        // Lerp with arc
                        let t = leg.step_progress;
                        let flat_pos = leg.start_step_pos.lerp(leg.target_foot_pos, t);
                        // No Z in 2D physics, but visual could use Z?
                        // IK is 2D.
                        leg.current_foot_pos = flat_pos;
                    }
                }
            }

            // --- IK SOLVE ---
            // Convert current_foot_pos (World) to Local (relative to mounting point?)
            // TwoBoneSolver root is (0,0).
            // So we need vector from Mounting Point to Foot in Local Space?
            // Wait, solver assumes root is at (0,0).
            // We draw the leg in Local Space of the Leg Entity.
            // The Leg Entity is parented to Body.
            // But Leg Entity Transform?
            // The Leg Entity is usually at (0,0,0) relative to body?
            // In `spawn_strider`, leg entity is spawned at `(0,0,9.0)`.
            // So leg entity origin IS body origin (offset in Z).
            // So local space of leg entity is body space.

            // Mounting point relative to body:
            let mount_local =
                Vec2::new(leg.offset_angle.cos(), leg.offset_angle.sin()) * leg.root_offset_dist;

            // Foot position relative to body:
            // We need to inverse transform current_foot_pos from World to Body Local.
            let rel_pos_world = leg.current_foot_pos - body_pos;
            let cos = (-body_angle).cos();
            let sin = (-body_angle).sin();
            let rel_pos_body = Vec2::new(
                rel_pos_world.x * cos - rel_pos_world.y * sin,
                rel_pos_world.x * sin + rel_pos_world.y * cos,
            );

            // Now, Solver assumes root is at (0,0). But our leg root is at `mount_local`.
            // So target for solver is `rel_pos_body - mount_local`.
            let target_for_solver = rel_pos_body - mount_local;

            let (joint, end) = leg.solver.solve(target_for_solver);

            // Now we have joint and end relative to mount_local.
            // We need to draw the path in Leg Entity Space (which is Body Space).
            // Points: Mount -> Joint -> End

            let p1 = mount_local;
            let p2 = mount_local + joint;
            let p3 = mount_local + end;

            // Draw
            let line = shapes::Polygon {
                points: vec![p1, p2, p3],
                closed: false,
            };
            *path = GeometryBuilder::build_as(&line);
        }
    }
}
