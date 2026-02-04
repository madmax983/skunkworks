use bevy::prelude::*;

/// Solves 2-bone Inverse Kinematics.
/// Returns (elbow_pos, end_effector_pos).
///
/// `root`: The base position of the limb.
/// `target`: The desired position of the end effector.
/// `len1`: Length of the first segment (root -> elbow).
/// `len2`: Length of the second segment (elbow -> end).
/// `elbow_dir`: A hint vector to determine which way the joint bends.
pub fn solve_2_bone(root: Vec2, target: Vec2, len1: f32, len2: f32, elbow_dir: Vec2) -> (Vec2, Vec2) {
    let to_target = target - root;
    let dist = to_target.length();

    // Unreachable: Too far
    if dist >= len1 + len2 {
        let dir = if dist > 0.0 { to_target / dist } else { Vec2::X };
        let elbow = root + dir * len1;
        let end = root + dir * (len1 + len2);
        return (elbow, end);
    }

    // Unreachable: Too close (can't reach even when folded back)
    // In this case, we just point towards target and fold as much as possible?
    // Or just treat as normal case (circle intersection still works mathematically often)
    // But if dist < |len1 - len2|, circles don't intersect.
    if dist < (len1 - len2).abs() {
        // Just extend towards target
        let dir = if dist > 0.0 { to_target / dist } else { Vec2::X };
        let elbow = root + dir * len1;
        // End is clamped?
        // Let's just return fully extended or folded.
        // If len1 > len2, end is inside sphere of len1.
        let end = elbow + dir * len2; // This extends away.
        return (elbow, end);
    }

    // Law of Cosines / Circle Intersection
    // x is distance from root to projection of elbow on AC line
    // x = (r1^2 - r2^2 + d^2) / (2d)
    // where r1=len1, r2=len2
    let x = (len1.powi(2) - len2.powi(2) + dist.powi(2)) / (2.0 * dist);

    // y is height of elbow from AC line
    // y^2 = r1^2 - x^2
    let y_sq = len1.powi(2) - x.powi(2);
    let y = if y_sq > 0.0 { y_sq.sqrt() } else { 0.0 };

    // Basis vectors
    let forward = to_target / dist;
    let right = Vec2::new(-forward.y, forward.x); // Perpendicular (90 deg CCW)

    // Two possible elbow positions: P + y*right OR P - y*right
    let p = root + forward * x;
    let elbow1 = p + right * y;
    let elbow2 = p - right * y;

    // Choose based on elbow_dir
    // We compare which one is closer to (root + elbow_dir) or similar heuristic.
    // Or dot product of (elbow - root) with elbow_dir?
    // Let's use dot product of the perp offset with elbow_dir.
    // Actually, just check which side of AC the elbow_dir points to.

    // Vector form root to elbow candidates
    // let r_e1 = elbow1 - root;
    // let r_e2 = elbow2 - root;

    // If elbow_dir aligns with r_e1 more than r_e2?
    // Simpler: Check cross product (2D determinant) of forward and elbow_dir.
    // If positive, we want the "left" solution (which is 'right' vector here? wait).
    // `right` vector is (-y, x) which is CCW (Left in standard coords).
    // So if elbow_dir is "Left" of forward, choose elbow1.

    let cross = forward.perp_dot(elbow_dir); // x1*y2 - y1*x2

    let elbow = if cross > 0.0 {
        elbow1 // 'right' vector is CCW/Left
    } else {
        elbow2
    };

    (elbow, target)
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_solve_reachable() {
        let root = Vec2::new(0.0, 0.0);
        let target = Vec2::new(2.0, 0.0);
        let len1 = 1.5;
        let len2 = 1.5;
        // With lengths 1.5 + 1.5 = 3.0, reaching 2.0 is easy.

        // Elbow hint up
        let elbow_dir = Vec2::new(0.0, 1.0);

        let (elbow, end) = solve_2_bone(root, target, len1, len2, elbow_dir);

        // Verify end is at target
        assert!((end - target).length() < 0.001, "End effector should reach target. Got {:?} expected {:?}", end, target);

        // Verify lengths
        let l1 = (elbow - root).length();
        let l2 = (end - elbow).length();
        assert!((l1 - len1).abs() < 0.001, "Segment 1 length incorrect: {} != {}", l1, len1);
        assert!((l2 - len2).abs() < 0.001, "Segment 2 length incorrect: {} != {}", l2, len2);

        // Verify elbow is "up" (positive Y)
        assert!(elbow.y > 0.0, "Elbow should bend up. Got {:?}", elbow);
    }
}
