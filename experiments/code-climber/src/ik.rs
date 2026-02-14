use bevy::math::Vec2;
use std::f32::consts::PI;

pub fn solve_2_bone_ik(
    root: Vec2,
    target: Vec2,
    len1: f32,
    len2: f32,
    bend_dir: f32,
) -> Option<(f32, f32)> {
    let diff = target - root;
    let dist = diff.length();

    // Check reachability (with small epsilon)
    if dist > len1 + len2 + 1e-4 {
        return None;
    }

    // Handle degenerate case (target at root)
    if dist < 1e-4 {
        // Arbitrary fold
        return Some((0.0, PI));
    }

    // Law of Cosines for alpha (angle between root-target line and bone1)
    // len2^2 = len1^2 + dist^2 - 2*len1*dist*cos(alpha)
    let cos_alpha = (len1 * len1 + dist * dist - len2 * len2) / (2.0 * len1 * dist);
    let alpha = cos_alpha.clamp(-1.0, 1.0).acos();

    // Law of Cosines for beta (interior angle at elbow)
    // dist^2 = len1^2 + len2^2 - 2*len1*len2*cos(beta)
    let cos_beta = (len1 * len1 + len2 * len2 - dist * dist) / (2.0 * len1 * len2);
    let beta = cos_beta.clamp(-1.0, 1.0).acos();

    let base_angle = diff.y.atan2(diff.x);

    // Determine joint angles
    // bend_dir > 0 => elbow bends CCW relative to target line?
    // Let's assume bend_dir determines which solution we pick.
    let sign = bend_dir.signum();

    let theta1 = base_angle + sign * alpha;

    // Theta2 is the angle of bone2 relative to bone1.
    // If the arm is fully extended (beta = PI), theta2 is 0.
    // If fully folded (beta = 0), theta2 is PI (or -PI).
    // The formula `theta2 = sign * (beta - PI)` worked in the thought experiment.
    let theta2 = sign * (beta - PI);

    Some((theta1, theta2))
}

#[cfg(test)]
mod tests {
    use super::*;
    use std::f32::consts::PI;

    #[test]
    fn test_solve_2_bone_ik_reachable() {
        let root = Vec2::ZERO;
        let len1 = 1.0;
        let len2 = 1.0;

        // Target: (2.0, 0.0) -> Full extension along X
        let target = Vec2::new(2.0, 0.0);
        let result = solve_2_bone_ik(root, target, len1, len2, 1.0);
        assert!(result.is_some());
        let (t1, t2) = result.unwrap();
        assert!((t1 - 0.0).abs() < 1e-4, "Expected t1=0, got {}", t1);
        assert!((t2 - 0.0).abs() < 1e-4, "Expected t2=0, got {}", t2);

        // Target: (0.0, 2.0) -> Full extension along Y
        let target = Vec2::new(0.0, 2.0);
        let result = solve_2_bone_ik(root, target, len1, len2, 1.0);
        assert!(result.is_some());
        let (t1, t2) = result.unwrap();
        assert!((t1 - PI/2.0).abs() < 1e-4, "Expected t1=PI/2, got {}", t1);
        assert!((t2 - 0.0).abs() < 1e-4, "Expected t2=0, got {}", t2);

        // Target: (1.0, 1.0) -> 90 degree bend
        let target = Vec2::new(1.0, 1.0);
        let result = solve_2_bone_ik(root, target, len1, len2, 1.0); // positive bend
        assert!(result.is_some());
        let (t1, t2) = result.unwrap();

        // With bend_dir = 1.0, we expect elbow at (0, 1).
        // Theta1 (bone 1 angle) = PI/2.
        // Theta2 (relative bone 2 angle) = -PI/2.
        assert!((t1 - PI/2.0).abs() < 1e-4, "Expected t1=PI/2, got {}", t1);
        assert!((t2 - (-PI/2.0)).abs() < 1e-4, "Expected t2=-PI/2, got {}", t2);

        // Test with negative bend dir
        let result = solve_2_bone_ik(root, target, len1, len2, -1.0);
        assert!(result.is_some());
        let (t1, t2) = result.unwrap();
        // Elbow should be at (1, 0).
        // Bone 1: (0,0) to (1,0). Theta1 = 0.
        // Bone 2: (1,0) to (1,1). Direction is Y+. Angle PI/2.
        // Relative theta2 = PI/2 - 0 = PI/2.
        assert!((t1 - 0.0).abs() < 1e-4, "Expected t1=0, got {}", t1);
        assert!((t2 - PI/2.0).abs() < 1e-4, "Expected t2=PI/2, got {}", t2);
    }

    #[test]
    fn test_solve_2_bone_ik_unreachable() {
        let root = Vec2::ZERO;
        let len1 = 1.0;
        let len2 = 1.0;
        let target = Vec2::new(3.0, 0.0); // Too far
        let result = solve_2_bone_ik(root, target, len1, len2, 1.0);
        assert!(result.is_none());
    }
}
