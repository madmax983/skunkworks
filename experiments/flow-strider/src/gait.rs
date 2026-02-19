use bevy::prelude::*;
use crate::graph::Graph;
use crate::strider::{Strider, Limb, LimbState};
use rand::prelude::*;

pub struct GaitPlugin;

impl Plugin for GaitPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, (move_body, update_limbs));
    }
}

fn move_body(
    mut striders: Query<(&mut Transform, &mut Strider)>,
    graph: Res<Graph>,
    time: Res<Time>,
) {
    let speed = 60.0; // pixels per second

    for (mut transform, mut strider) in striders.iter_mut() {
        if strider.target_node >= graph.nodes.len() { continue; }

        let target_pos = graph.nodes[strider.target_node];
        let current_pos = transform.translation.truncate();
        let diff = target_pos - current_pos;
        let dist = diff.length();

        if dist < 5.0 {
            // Arrived, pick new random neighbor
            // Find edges connected to current node
            let mut neighbors = Vec::new();
            for &(start, end) in &graph.edges {
                if start == strider.target_node {
                    neighbors.push(end);
                } else if end == strider.target_node {
                    neighbors.push(start);
                }
            }

            if !neighbors.is_empty() {
                let mut rng = thread_rng();
                strider.target_node = *neighbors.choose(&mut rng).unwrap();
            }
        } else {
            // Move towards target
            let move_vec = diff.normalize() * speed * time.delta_seconds();
            transform.translation += move_vec.extend(0.0);
        }
    }
}

fn update_limbs(
    mut limbs: Query<(&Parent, &mut Limb)>,
    transforms: Query<&GlobalTransform>,
    time: Res<Time>,
) {
    for (parent, mut limb) in limbs.iter_mut() {
        if let Ok(body_transform) = transforms.get(parent.get()) {
            let body_pos = body_transform.translation().truncate();

            // "Ideal" is where the foot naturally rests relative to body
            let offset = match limb.index {
                0 => Vec2::new(-30.0, 30.0),
                1 => Vec2::new(30.0, 30.0),
                2 => Vec2::new(-30.0, -30.0),
                3 => Vec2::new(30.0, -30.0),
                _ => Vec2::ZERO,
            };
            let ideal_pos = body_pos + offset;

            match limb.state {
                LimbState::Stance => {
                    let dist = limb.foot_pos.distance(ideal_pos);
                    if dist > 50.0 {
                        // Trigger step if far
                        limb.state = LimbState::Lift(0.0);
                        limb.start_foot_pos = limb.foot_pos;
                        // Step ahead of ideal?
                        // Simple prediction: just step to ideal for now.
                        limb.target_foot_pos = ideal_pos;
                        limb.lift_height = 20.0;
                    }
                },
                LimbState::Lift(mut t) => {
                    t += time.delta_seconds() * 4.0; // Step speed
                    if t >= 1.0 {
                        limb.foot_pos = limb.target_foot_pos;
                        limb.state = LimbState::Stance;
                    } else {
                        limb.state = LimbState::Lift(t);
                        // Interpolate ground position
                        let ground_pos = limb.start_foot_pos.lerp(limb.target_foot_pos, t);
                        limb.foot_pos = ground_pos;
                        // Height logic would go here if 3D or visual offset
                    }
                }
            }
        }
    }
}
