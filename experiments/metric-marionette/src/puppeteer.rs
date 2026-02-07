use crate::monitor::Monitor;
use crate::skeleton::{Joint, Skeleton};
use rand::Rng;
use std::f64::consts::PI;

pub struct Puppeteer {
    time: f64,
}

impl Puppeteer {
    pub fn new() -> Self {
        Self { time: 0.0 }
    }

    pub fn build_skeleton() -> Skeleton {
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

        Skeleton { root }
    }

    pub fn update(&mut self, skeleton: &mut Skeleton, monitor: &Monitor, dt: f64) {
        self.time += dt;
        self.animate_recursive(&mut skeleton.root, monitor);
    }

    fn animate_recursive(&self, joint: &mut Joint, monitor: &Monitor) {
        let mut rng = rand::thread_rng();

        match joint.name.as_str() {
            "spine" => {
                // Breathing: Fast if CPU high
                let breath_speed = 2.0 + monitor.cpu_usage * 10.0;
                let breath = (self.time * breath_speed).sin() * 0.05;

                // Slouch: Heavy RAM
                let slouch = monitor.ram_usage * 0.5;

                joint.angle = (PI / 2.0) + breath - slouch;
            }
            "l_arm" => {
                // Jitter
                let jitter = (rng.r#gen::<f64>() - 0.5) * monitor.cpu_usage * 0.3;
                // Droop (RAM)
                let droop = monitor.ram_usage * 1.5;
                // Base relative: PI/2
                // Droop: Increases angle (towards PI) or decreases?
                // Spine is Up. Arm is Left. Down is further Left/Down?
                // Global goal: Down (-PI/2).
                // Global Spine: PI/2.
                // Target Rel: -PI.
                // Current Rel: PI/2.
                // Interpolate towards -PI?

                // Let's just rotate downward. Left is CCW from Up. Down is more CCW? No, Left (PI). Down (-PI/2 or 3PI/2).
                // So we want to increase angle from PI/2 to PI (Left-Down 45 deg) to 3PI/2 (Down).
                // So + droop.

                joint.angle = (PI / 2.0) + droop + jitter;
            }
            "r_arm" => {
                let jitter = (rng.r#gen::<f64>() - 0.5) * monitor.cpu_usage * 0.3;
                let droop = monitor.ram_usage * 1.5;
                // Base relative: -PI/2 (Right)
                // Spine Up (PI/2).
                // Right (0). Rel: -PI/2.
                // Down (-PI/2 or 3PI/2).
                // Global goal: -PI/2.
                // Global Spine: PI/2.
                // Target Rel: -PI.
                // Current Rel: -PI/2.
                // To go Down, we need to subtract angle?
                // Up -> Right is -90. Right -> Down is -90.
                // So - droop.

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
            self.animate_recursive(child, monitor);
        }
    }
}
