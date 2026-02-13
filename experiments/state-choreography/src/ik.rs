use bevy::prelude::*;
use crate::components::*;

pub struct IKPlugin;

impl Plugin for IKPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, solve_ik);
    }
}

fn solve_ik(
    chains: Query<&IKChain>,
    mut joints_query: Query<(&mut Joint, &mut Transform)>, // We need both component and transform
    global_transforms: Query<&GlobalTransform>,
) {
    for chain in chains.iter() {
        let target_pos = if let Ok(t) = global_transforms.get(chain.target) {
            t.translation().truncate()
        } else {
            continue;
        };

        // We need the effector's global position.
        // We'll update this as we rotate joints.
        let mut current_effector_pos = if let Ok(t) = global_transforms.get(chain.effector) {
            t.translation().truncate()
        } else {
            continue;
        };

        for _ in 0..chain.iterations {
            for &joint_entity in chain.joints.iter().rev() {
                // Get Joint Global Position
                let joint_global_pos = if let Ok(t) = global_transforms.get(joint_entity) {
                    t.translation().truncate()
                } else {
                    continue;
                };

                let to_effector = current_effector_pos - joint_global_pos;
                let to_target = target_pos - joint_global_pos;

                // Avoid singularities
                if to_effector.length_squared() < 0.0001 || to_target.length_squared() < 0.0001 {
                    continue;
                }

                let current_angle_to_effector = to_effector.y.atan2(to_effector.x);
                let target_angle_to_target = to_target.y.atan2(to_target.x);
                let mut diff = target_angle_to_target - current_angle_to_effector;

                // Normalize diff to -PI..PI
                while diff > std::f32::consts::PI { diff -= 2.0 * std::f32::consts::PI; }
                while diff < -std::f32::consts::PI { diff += 2.0 * std::f32::consts::PI; }

                // Apply rotation with limits
                if let Ok((mut joint, mut transform)) = joints_query.get_mut(joint_entity) {
                    let new_angle = joint.current_angle + diff;
                    let clamped_angle = new_angle.clamp(joint.min_angle, joint.max_angle);

                    let actual_change = clamped_angle - joint.current_angle;

                    // Update Joint state
                    joint.current_angle = clamped_angle;

                    // Update Transform
                    // transform.rotate_z(actual_change); // This rotates incrementally.
                    // Alternatively, set rotation explicitly from angle if Z-axis is the only rotation.
                    // But simpler to just rotate by the delta.
                    transform.rotate_z(actual_change);

                    // Update current_effector_pos for next iteration (upstream joint)
                    // We rotate the vector `to_effector` by `actual_change`
                    let cos_theta = actual_change.cos();
                    let sin_theta = actual_change.sin();
                    let rx = to_effector.x * cos_theta - to_effector.y * sin_theta;
                    let ry = to_effector.x * sin_theta + to_effector.y * cos_theta;

                    current_effector_pos = joint_global_pos + Vec2::new(rx, ry);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use bevy::prelude::*;
    use bevy::transform::TransformPlugin;
    use bevy::hierarchy::HierarchyPlugin;

    #[test]
    fn test_ik_solver_rotates_joint() {
        let mut app = App::new();
        app.add_plugins(MinimalPlugins);
        app.add_plugins(TransformPlugin);
        app.add_plugins(HierarchyPlugin);
        app.add_systems(Update, solve_ik);

        // Setup Scene
        let target = app.world_mut().spawn(TransformBundle::from_transform(Transform::from_translation(Vec3::new(0.0, 5.0, 0.0)))).id();

        // Chain: Root -> Joint -> Effector
        let root = app.world_mut().spawn(TransformBundle::from_transform(Transform::from_translation(Vec3::ZERO))).id();

        let joint = app.world_mut().spawn((
            TransformBundle::from_transform(Transform::from_translation(Vec3::ZERO)),
            Joint::default(),
        )).id();
        app.world_mut().entity_mut(root).add_child(joint);

        let effector = app.world_mut().spawn(TransformBundle::from_transform(Transform::from_translation(Vec3::new(5.0, 0.0, 0.0)))).id();
        app.world_mut().entity_mut(joint).add_child(effector);

        app.world_mut().spawn(IKChain {
            joints: vec![joint],
            effector,
            target,
            iterations: 1,
        });

        // First update to propagate initial transforms
        app.update();

        // Second update to run IK
        app.update();

        // Third update to propagate new transforms from IK (Wait, IK modifies Local Transform, need propagation to see Global effect)
        app.update();

        let effector_tf = app.world().get::<GlobalTransform>(effector).unwrap();
        let target_tf = app.world().get::<GlobalTransform>(target).unwrap();

        let dist_before = Vec3::new(5.0, 0.0, 0.0).distance(target_tf.translation());
        let dist_after = effector_tf.translation().distance(target_tf.translation());

        // Assert that we moved closer
        // Original dist = 7.07 (from (5,0) to (0,5))
        // If we rotate 90 deg, we are at (0,5), dist = 0.
        println!("Dist Before: {}, Dist After: {}", dist_before, dist_after);
        assert!(dist_after < dist_before, "Effector did not move closer. Dist before: {}, Dist after: {}", dist_before, dist_after);
    }
}
