use bevy::prelude::*;

#[derive(Component)]
pub struct IkChain {
    pub target: Entity,
    pub joints: Vec<Entity>,
    pub end_effector: Entity,
}

pub fn solve_ik(
    chains: Query<&IkChain>,
    mut transforms: Query<&mut Transform>,
    global_transforms: Query<&GlobalTransform>,
) {
    for chain in chains.iter() {
        let target_pos = if let Ok(target_tf) = global_transforms.get(chain.target) {
            target_tf.translation().truncate()
        } else {
            continue;
        };

        let effector_pos = if let Ok(eff_tf) = global_transforms.get(chain.end_effector) {
            eff_tf.translation().truncate()
        } else {
            continue;
        };

        // Stop if close enough
        if effector_pos.distance_squared(target_pos) < 1.0 {
            continue;
        }

        // Iterate backwards through joints
        for &joint_entity in chain.joints.iter().rev() {
            if let Ok(joint_global) = global_transforms.get(joint_entity) {
                let joint_pos = joint_global.translation().truncate();

                let to_effector = effector_pos - joint_pos;
                let to_target = target_pos - joint_pos;

                if to_effector.length_squared() < 0.001 || to_target.length_squared() < 0.001 {
                    continue;
                }

                // 2D Rotation logic
                let current_angle = to_effector.y.atan2(to_effector.x);
                let target_angle = to_target.y.atan2(to_target.x);
                let mut angle_diff = target_angle - current_angle;

                // Normalize angle to -PI..PI
                while angle_diff > std::f32::consts::PI {
                    angle_diff -= 2.0 * std::f32::consts::PI;
                }
                while angle_diff < -std::f32::consts::PI {
                    angle_diff += 2.0 * std::f32::consts::PI;
                }

                // Dampening for organic feel
                let speed = 0.1;
                let angle_diff = angle_diff.clamp(-speed, speed);

                if let Ok(mut joint_local) = transforms.get_mut(joint_entity) {
                    joint_local.rotate_z(angle_diff);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ik_system_logic() {
        let mut app = App::new();
        app.add_systems(Update, solve_ik);

        // Setup entities
        let target = app
            .world_mut()
            .spawn((
                Transform::from_xyz(10.0, 0.0, 0.0),
                GlobalTransform::from_xyz(10.0, 0.0, 0.0),
            ))
            .id();

        let joint = app
            .world_mut()
            .spawn((
                Transform::from_xyz(0.0, 0.0, 0.0),
                GlobalTransform::from_xyz(0.0, 0.0, 0.0),
            ))
            .id();

        let effector = app
            .world_mut()
            .spawn((
                Transform::from_xyz(5.0, 0.0, 0.0), // Starts at x=5
                GlobalTransform::from_xyz(5.0, 0.0, 0.0),
            ))
            .id();

        app.world_mut().spawn(IkChain {
            target,
            joints: vec![joint],
            end_effector: effector,
        });

        // In a real run, hierarchy propagation updates GlobalTransform.
        // Here we manually set them for the test scenario.
        // Joint is at 0,0. Effector at 5,0. Target at 10,0.
        // Already aligned. No rotation should happen.

        app.update();

        let joint_tf = app.world().get::<Transform>(joint).unwrap();
        // Rotation should be identity (0)
        assert!(joint_tf.rotation.to_axis_angle().1.abs() < 0.001);

        // Now move target to 0, 5 (90 degrees)
        // Joint at 0,0. Effector at 5,0. Target at 0,5.
        // Angle to effector: 0. Angle to target: PI/2. Diff: PI/2.
        // It should rotate positive.

        let mut target_tf = app.world_mut().get_mut::<GlobalTransform>(target).unwrap();
        *target_tf = GlobalTransform::from_xyz(0.0, 5.0, 0.0);

        app.update();

        let joint_tf = app.world().get::<Transform>(joint).unwrap();
        let (_axis, angle) = joint_tf.rotation.to_axis_angle();
        // Since we clamp speed to 0.1, the angle should be close to 0.1
        assert!((angle - 0.1).abs() < 0.001);
    }
}
