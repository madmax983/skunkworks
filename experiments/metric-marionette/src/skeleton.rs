use locus::Vec2;

#[derive(Clone, Debug)]
pub struct Joint {
    pub name: String,
    pub length: f64,
    pub angle: f64, // Relative to parent in radians
    pub children: Vec<Joint>,
}

#[derive(Clone, Debug)]
pub struct Skeleton {
    pub root: Joint,
}

impl Skeleton {
    pub fn new() -> Self {
        Skeleton {
            root: Joint {
                name: "root".to_string(),
                length: 0.0,
                angle: 0.0,
                children: vec![],
            },
        }
    }

    /// Returns a list of (start, end) points for drawing the bones.
    pub fn solve_fk(&self) -> Vec<(Vec2, Vec2)> {
        let mut bones = Vec::new();
        // Start at (0,0) with 0 rotation
        self.solve_recursive(&self.root, Vec2::new(0.0, 0.0), 0.0, &mut bones);
        bones
    }

    fn solve_recursive(
        &self,
        joint: &Joint,
        start_pos: Vec2,
        parent_angle: f64,
        bones: &mut Vec<(Vec2, Vec2)>,
    ) {
        let global_angle = parent_angle + joint.angle;

        let offset = Vec2::new(global_angle.cos(), global_angle.sin()) * joint.length;
        let end_pos = start_pos + offset;

        // Only add bone if it has length (visual purposes)
        if joint.length > 0.001 {
            bones.push((start_pos, end_pos));
        }

        for child in &joint.children {
            self.solve_recursive(child, end_pos, global_angle, bones);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fk_chain() {
        // Create a 2-segment arm
        // Root (0,0) -> Arm (Length 10, Angle 0) -> Forearm (Length 10, Angle 0)
        // Expected Tip: (20, 0) assuming 0 degrees is right/X+.

        let forearm = Joint {
            name: "forearm".to_string(),
            length: 10.0,
            angle: 0.0,
            children: vec![],
        };

        let arm = Joint {
            name: "arm".to_string(),
            length: 10.0,
            angle: 0.0,
            children: vec![forearm],
        };

        let root = Joint {
            name: "root".to_string(),
            length: 0.0, // Root is just a pivot
            angle: 0.0,
            children: vec![arm],
        };

        let skeleton = Skeleton { root };
        let bones = skeleton.solve_fk();

        assert!(!bones.is_empty(), "Bones should not be empty");

        // Root has length 0, so it doesn't produce a bone line in my implementation if I check length > 0.
        // Bone 0: Arm
        let (p1_start, p1_end) = bones[0];
        // Floating point comparison needs tolerance, but for exact integers 0, 10, it might be fine.
        // Let's use epsilon if needed.
        let epsilon = 0.0001;
        assert!((p1_start.x - 0.0).abs() < epsilon);
        assert!((p1_start.y - 0.0).abs() < epsilon);
        assert!((p1_end.x - 10.0).abs() < epsilon);
        assert!((p1_end.y - 0.0).abs() < epsilon);

        // Bone 1: Forearm
        let (p2_start, p2_end) = bones[1];
        assert!((p2_start.x - 10.0).abs() < epsilon);
        assert!((p2_start.y - 0.0).abs() < epsilon);
        assert!((p2_end.x - 20.0).abs() < epsilon);
        assert!((p2_end.y - 0.0).abs() < epsilon);
    }
}
