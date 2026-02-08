use crate::monitor::Monitor;
use rand::Rng;
use std::f64::consts::PI;
use tui_shared::math::Vec2;

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
    pub time: f64,
}

impl Skeleton {
    pub fn new_humanoid() -> Self {
        // Build a humanoid skeleton
        // Lengths are relative units

        let head = Joint {
            name: "head".into(),
            length: 4.0,
            angle: 0.0,
            children: vec![],
        }; // Relative to neck
        let neck = Joint {
            name: "neck".into(),
            length: 2.0,
            angle: 0.0,
            children: vec![head],
        }; // Relative to spine

        // Arms (Starting from shoulder)
        // Left Arm (Points Left: PI). Parent Spine is Up (PI/2). Relative: PI/2.
        let l_hand = Joint {
            name: "l_hand".into(),
            length: 3.0,
            angle: 0.0,
            children: vec![],
        };
        let l_forearm = Joint {
            name: "l_forearm".into(),
            length: 6.0,
            angle: 0.0,
            children: vec![l_hand],
        };
        let l_arm = Joint {
            name: "l_arm".into(),
            length: 6.0,
            angle: PI / 2.0,
            children: vec![l_forearm],
        };

        // Right Arm (Points Right: 0). Parent Spine is Up (PI/2). Relative: -PI/2.
        let r_hand = Joint {
            name: "r_hand".into(),
            length: 3.0,
            angle: 0.0,
            children: vec![],
        };
        let r_forearm = Joint {
            name: "r_forearm".into(),
            length: 6.0,
            angle: 0.0,
            children: vec![r_hand],
        };
        let r_arm = Joint {
            name: "r_arm".into(),
            length: 6.0,
            angle: -PI / 2.0,
            children: vec![r_forearm],
        };

        let spine = Joint {
            name: "spine".into(),
            length: 8.0,
            angle: PI / 2.0, // Up relative to root (0)
            children: vec![neck, l_arm, r_arm],
        };

        // Legs
        // Left Leg (Down-Left). Parent Root is 0. Down is -PI/2. Slight spread.
        let l_foot = Joint {
            name: "l_foot".into(),
            length: 2.0,
            angle: PI / 2.0,
            children: vec![],
        };
        let l_shin = Joint {
            name: "l_shin".into(),
            length: 8.0,
            angle: 0.0,
            children: vec![l_foot],
        };
        let l_thigh = Joint {
            name: "l_thigh".into(),
            length: 8.0,
            angle: -PI / 2.0 - 0.3,
            children: vec![l_shin],
        };

        // Right Leg (Down-Right)
        let r_foot = Joint {
            name: "r_foot".into(),
            length: 2.0,
            angle: PI / 2.0,
            children: vec![],
        };
        let r_shin = Joint {
            name: "r_shin".into(),
            length: 8.0,
            angle: 0.0,
            children: vec![r_foot],
        };
        let r_thigh = Joint {
            name: "r_thigh".into(),
            length: 8.0,
            angle: -PI / 2.0 + 0.3,
            children: vec![r_shin],
        };

        // Root (Pelvis)
        let root = Joint {
            name: "root".into(),
            length: 0.0,
            angle: 0.0,
            children: vec![spine, l_thigh, r_thigh],
        };

        Skeleton { root, time: 0.0 }
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

    pub fn animate(&mut self, monitor: &Monitor, dt: f64) {
        self.time += dt;
        let time = self.time; // Copy for closure/recursive call

        // Use a recursive helper
        Self::animate_recursive(&mut self.root, monitor, time);
    }

    fn animate_recursive(joint: &mut Joint, monitor: &Monitor, time: f64) {
        let mut rng = rand::thread_rng();

        match joint.name.as_str() {
            "spine" => {
                // Breathing: Fast if CPU high
                let breath_speed = 2.0 + monitor.cpu_usage * 10.0;
                let breath = (time * breath_speed).sin() * 0.05;

                // Slouch: Heavy RAM
                let slouch = monitor.ram_usage * 0.5;

                joint.angle = (PI / 2.0) + breath - slouch;
            }
            "l_arm" => {
                // Jitter
                let jitter = (rng.r#gen::<f64>() - 0.5) * monitor.cpu_usage * 0.3;
                // Droop (RAM)
                let droop = monitor.ram_usage * 1.5;

                joint.angle = (PI / 2.0) + droop + jitter;
            }
            "r_arm" => {
                let jitter = (rng.r#gen::<f64>() - 0.5) * monitor.cpu_usage * 0.3;
                let droop = monitor.ram_usage * 1.5;

                joint.angle = (-PI / 2.0) - droop + jitter;
            }
            "head" => {
                // Look around randomly if network? Or just random noise.
                let noise = (rng.r#gen::<f64>() - 0.5) * 0.2;
                joint.angle = noise;
            }
            _ => {}
        }

        for child in &mut joint.children {
            Self::animate_recursive(child, monitor, time);
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

        let skeleton = Skeleton { root, time: 0.0 };
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
