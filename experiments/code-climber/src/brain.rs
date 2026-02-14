use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use crate::ik;
use crate::climber::{Climber, Limb, Side, BoneType};

#[derive(Component)]
pub struct Hold;

pub fn climb_control(
    mut limbs_q: Query<(&Limb, &mut ImpulseJoint)>,
    climber_q: Query<&Transform, With<Climber>>,
    time: Res<Time>,
) {
    let t = time.elapsed_seconds();
    // Target moves up and circles
    let target_pos = Vec2::new(-30.0 + (t * 2.0).cos() * 20.0, -50.0 + t * 10.0 + (t * 2.0).sin() * 20.0);

    if let Ok(torso_tf) = climber_q.get_single() {
         let torso_pos = torso_tf.translation.truncate();
         // Approx shoulder position (Left) relative to Torso (0,0) is (-10, 15)
         // We should ideally rotate this offset by torso rotation, but let's assume stability for now.
         let shoulder_pos = torso_pos + Vec2::new(-10.0, 15.0);

         let len1 = 25.0;
         let len2 = 25.0;

         // Solve IK for Left Arm
         if let Some((theta1, theta2)) = ik::solve_2_bone_ik(shoulder_pos, target_pos, len1, len2, 1.0) {
             for (limb, mut joint) in limbs_q.iter_mut() {
                 if limb.side == Side::Left {
                     if let Some(revolute) = joint.data.as_revolute_mut() {
                         if limb.bone_type == BoneType::Upper {
                             revolute.set_motor_position(theta1, 50000.0, 1000.0);
                         } else if limb.bone_type == BoneType::Lower {
                             revolute.set_motor_position(theta2, 50000.0, 1000.0);
                         }
                     }
                 }

                 // Mirror for Right Arm (just for fun, inverse target x)
                 if limb.side == Side::Right {
                      let right_target = Vec2::new(-target_pos.x, target_pos.y);
                      let right_shoulder = torso_pos + Vec2::new(10.0, 15.0);
                      // Bend dir -1 for right arm
                      if let Some((r_theta1, r_theta2)) = ik::solve_2_bone_ik(right_shoulder, right_target, len1, len2, -1.0) {
                          if let Some(revolute) = joint.data.as_revolute_mut() {
                             if limb.bone_type == BoneType::Upper {
                                 revolute.set_motor_position(r_theta1, 50000.0, 1000.0);
                             } else if limb.bone_type == BoneType::Lower {
                                 revolute.set_motor_position(r_theta2, 50000.0, 1000.0);
                             }
                         }
                      }
                 }
             }
         }
    }
}
