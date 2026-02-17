use bevy::prelude::*;

#[derive(Clone)]
pub struct TwoBoneSolver {
    pub root_pos: Vec2,
    pub l1: f32,
    pub l2: f32,
    pub flip: bool, // Flip the knee?
}

impl TwoBoneSolver {
    pub fn new(l1: f32, l2: f32) -> Self {
        Self {
            root_pos: Vec2::ZERO,
            l1,
            l2,
            flip: false,
        }
    }

    /// Solves for the joint position given a target position (relative to root).
    /// Returns (joint_pos, end_effector_pos).
    /// If the target is out of reach, it points towards it.
    pub fn solve(&self, target_rel: Vec2) -> (Vec2, Vec2) {
        let dist = target_rel.length();

        // Clamp reach
        let max_reach = self.l1 + self.l2;
        let valid_target = if dist > max_reach {
            target_rel.normalize_or_zero() * max_reach
        } else {
            target_rel
        };

        let mut dist = valid_target.length();
        let mut final_target = valid_target;

        let min_reach = (self.l1 - self.l2).abs();
        if dist < min_reach {
             // If dist is too small, clamp to min_reach
             if dist < 0.0001 {
                 final_target = Vec2::new(min_reach, 0.0);
             } else {
                 final_target = valid_target.normalize() * min_reach;
             }
             dist = min_reach;
        }

        if dist < 0.001 {
             // Should not happen if min_reach > 0.001, but handled above
            return (Vec2::new(self.l1, 0.0), Vec2::ZERO);
        }

        // Law of Cosines
        // c^2 = a^2 + b^2 - 2ab cos(C)
        // We want angle at root (alpha) and angle at joint (beta)

        // Angle of the target vector
        let base_angle = final_target.y.atan2(final_target.x);

        // Angle offset for the first bone
        // cos(angle) = (b^2 + c^2 - a^2) / (2bc)
        // here a = l2, b = l1, c = dist
        let cos_angle_l1 = (self.l1 * self.l1 + dist * dist - self.l2 * self.l2) / (2.0 * self.l1 * dist);

        // Clamp for safety (floating point errors)
        let cos_angle_l1 = cos_angle_l1.clamp(-1.0, 1.0);
        let angle_l1 = cos_angle_l1.acos();

        let root_angle = if self.flip {
            base_angle + angle_l1
        } else {
            base_angle - angle_l1
        };

        // Joint position
        let joint_pos = Vec2::new(root_angle.cos(), root_angle.sin()) * self.l1;

        (joint_pos, final_target)
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ik_reach() {
        let solver = TwoBoneSolver::new(10.0, 10.0);
        let target = Vec2::new(10.0, 0.0);
        let (joint, end) = solver.solve(target);

        // At dist 10, with L1=10, L2=10
        // It forms an isosceles triangle.
        // Height = sqrt(10^2 - 5^2) = sqrt(75) ~= 8.66
        // Joint x should be 5.0.

        assert!((end.x - 10.0).abs() < 0.001);
        assert!((end.y - 0.0).abs() < 0.001);
        assert!((joint.x - 5.0).abs() < 0.01);
    }

    #[test]
    fn test_ik_max_reach() {
        let solver = TwoBoneSolver::new(10.0, 10.0);
        let target = Vec2::new(30.0, 0.0); // Out of reach (max 20)
        let (_joint, end) = solver.solve(target);

        assert!((end.length() - 20.0).abs() < 0.001);
        assert!((end.y).abs() < 0.001);
    }
}
