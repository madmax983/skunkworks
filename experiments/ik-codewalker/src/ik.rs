use bevy::prelude::*;
use crate::components::*;

pub struct IKPlugin;

impl Plugin for IKPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, solve_ik);
    }
}

#[derive(Component)]
pub struct IKChain {
    pub joints: Vec<Entity>, // Ordered from Root to Last Joint
    pub effector: Entity,
    pub target: Entity,
    pub iterations: usize,
}

fn solve_ik(
    mut chains: Query<&IKChain>,
    mut transforms: Query<&mut Transform>,
    global_transforms: Query<&GlobalTransform>,
) {
    for chain in chains.iter() {
        // Get Target Position
        let target_pos = if let Ok(t) = global_transforms.get(chain.target) {
            t.translation().truncate()
        } else {
            continue;
        };

        // Get initial effector position
        let mut current_effector_pos = if let Ok(t) = global_transforms.get(chain.effector) {
            t.translation().truncate()
        } else {
            continue;
        };

        for _ in 0..chain.iterations {
            // Iterate backwards from the last joint to the root
            for &joint_entity in chain.joints.iter().rev() {
                let joint_global_transform = if let Ok(t) = global_transforms.get(joint_entity) {
                    t
                } else {
                    continue;
                };

                let joint_global_pos = joint_global_transform.translation().truncate();

                let to_effector = current_effector_pos - joint_global_pos;
                let to_target = target_pos - joint_global_pos;

                // Avoid division by zero
                if to_effector.length_squared() < 0.001 || to_target.length_squared() < 0.001 {
                    continue;
                }

                let current_angle = to_effector.y.atan2(to_effector.x);
                let target_angle = to_target.y.atan2(to_target.x);
                let mut diff = target_angle - current_angle;

                // Normalize angle
                if diff > std::f32::consts::PI {
                    diff -= 2.0 * std::f32::consts::PI;
                } else if diff < -std::f32::consts::PI {
                    diff += 2.0 * std::f32::consts::PI;
                }

                // Apply rotation to local transform
                // Note: This assumes the global rotation change maps 1:1 to local rotation change (2D constraint)
                if let Ok(mut joint_tf) = transforms.get_mut(joint_entity) {
                    joint_tf.rotate_z(diff);
                }

                // Update proxy effector position for the next iteration (upstream joint)
                let cos_theta = diff.cos();
                let sin_theta = diff.sin();
                let rx = to_effector.x * cos_theta - to_effector.y * sin_theta;
                let ry = to_effector.x * sin_theta + to_effector.y * cos_theta;

                current_effector_pos = joint_global_pos + Vec2::new(rx, ry);
            }
        }
    }
}
