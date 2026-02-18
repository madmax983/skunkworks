use bevy::prelude::*;
use crate::dancer::{Dancer, DancePose, Limb, LimbKind, PoseTransition, TargetPosition};

pub fn pose_interpolator(
    time: Res<Time>,
    mut dancers: Query<(&DancePose, &TargetPosition, &PoseTransition, &Children, &mut Transform), With<Dancer>>,
    mut limbs: Query<(&Limb, &mut Transform), Without<Dancer>>,
) {
    let dt = time.delta_seconds();

    for (target_pose, target_pos, transition, children, mut root_transform) in dancers.iter_mut() {
        // Interpolate Position
        let diff = target_pos.0 - root_transform.translation;
        if diff.length_squared() > 0.1 {
            root_transform.translation += diff * (transition.speed * dt).min(1.0);
        } else {
            root_transform.translation = target_pos.0;
        }

        // Interpolate Limbs
        for &child in children.iter() {
            if let Ok((limb, mut limb_transform)) = limbs.get_mut(child) {
                let target_angle = match limb.kind {
                    LimbKind::Head => target_pose.head_tilt,
                    LimbKind::Torso => target_pose.torso_bend,
                    LimbKind::LeftArm => target_pose.left_arm_angle,
                    LimbKind::RightArm => target_pose.right_arm_angle,
                    LimbKind::LeftLeg => target_pose.left_leg_angle,
                    LimbKind::RightLeg => target_pose.right_leg_angle,
                };

                // Current angle from Z-rotation
                let (_, _, current_angle) = limb_transform.rotation.to_euler(EulerRot::XYZ);

                // Interpolate
                let diff = target_angle - current_angle;
                let new_angle = current_angle + diff * (transition.speed * dt).min(1.0);

                // Apply
                limb_transform.rotation = Quat::from_rotation_z(new_angle);
            }
        }
    }
}
