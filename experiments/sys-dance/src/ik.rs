use bevy::prelude::*;
use crate::skeleton::IKChain;

pub fn solve_two_bone(
    root_pos: Vec2,
    target_pos: Vec2,
    len1: f32,
    len2: f32,
    bend_dir: f32,
) -> (f32, f32) {
    let to_target = target_pos - root_pos;
    let dist_sq = to_target.length_squared();
    let dist = dist_sq.sqrt();

    // Clamp reach
    let max_len = len1 + len2 - 0.001;
    let dist = dist.clamp(0.001, max_len);

    // Law of Cosines for Elbow Angle (interior angle C)
    // c^2 = a^2 + b^2 - 2ab cos(C) -> cos(C) = (a^2 + b^2 - c^2) / 2ab
    let cos_c = (len1 * len1 + len2 * len2 - dist * dist) / (2.0 * len1 * len2);
    let angle_c = cos_c.clamp(-1.0, 1.0).acos();

    // Law of Cosines for Shoulder Angle (relative to target vector)
    // b^2 = a^2 + c^2 - 2ac cos(A) -> cos(A) = (a^2 + c^2 - b^2) / 2ac
    // Here a=len2, b=dist (wait, opposite to B), c=len1.
    // Standard: a=len1, b=len2, c=dist (side opposite elbow).
    // Angle at shoulder (A) is opposite side a (len2).
    // cos(A) = (b^2 + c^2 - a^2) / 2bc = (len1^2 + dist^2 - len2^2) / (2 * len1 * dist)
    let cos_a = (len1 * len1 + dist * dist - len2 * len2) / (2.0 * len1 * dist);
    let angle_a = cos_a.clamp(-1.0, 1.0).acos();

    let target_angle = to_target.y.atan2(to_target.x);

    let angle1 = target_angle + angle_a * bend_dir;
    let angle2 = (std::f32::consts::PI - angle_c) * bend_dir; // Deviation from straight

    (angle1, angle2)
}

pub fn ik_system(
    chains: Query<(Entity, &IKChain, &GlobalTransform, &Parent)>,
    mut transforms: Query<&mut Transform>,
    globals: Query<&GlobalTransform>,
) {
    // Collect updates first to avoid borrow checker issues if chains overlap (unlikely but possible)
    let mut updates = Vec::new();

    for (entity, chain, chain_global, parent) in chains.iter() {
        let root_pos = chain_global.translation().truncate();
        let target_pos = chain.target;

        // Calculate angles
        let (global_angle1, local_angle2) = solve_two_bone(
            root_pos,
            target_pos,
            chain.len1,
            chain.len2,
            chain.bend_dir
        );

        // We need the parent's global rotation to convert global_angle1 to local
        if let Ok(parent_global) = globals.get(parent.get()) {
             let parent_rotation = parent_global.to_scale_rotation_translation().1.to_euler(EulerRot::XYZ).2;

             // Apply offset because visual is -Y aligned (Down)
             // We want Angle1 to correspond to direction.
             // Rotation R applied to (0,-1) = (cos(a), sin(a))
             // R - PI/2 = a => R = a + PI/2
             let local_angle1 = global_angle1 - parent_rotation + std::f32::consts::FRAC_PI_2;

             updates.push((chain.bone1, local_angle1));
             updates.push((chain.bone2, local_angle2));
        }
    }

    for (bone, angle) in updates {
        if let Ok(mut transform) = transforms.get_mut(bone) {
            transform.rotation = Quat::from_rotation_z(angle);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_straight_reach() {
        // Target directly below root at full extension
        let (a1, a2) = solve_two_bone(Vec2::ZERO, Vec2::new(0.0, -20.0), 10.0, 10.0, 1.0);
        // Angle1 should be -PI/2 (pointing down)
        // Angle2 should be 0 (straight)
        assert!((a1 - -PI/2.0).abs() < 0.05, "Angle1 was {}", a1);
        assert!(a2.abs() < 0.05, "Angle2 was {}", a2);
    }
}
