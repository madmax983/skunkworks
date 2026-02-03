#[derive(Debug, Clone)]
pub struct Arm {
    pub joints: Vec<(f64, f64)>, // x, y coordinates
    pub lengths: Vec<f64>,
    pub tolerance: f64,
}

impl Arm {
    pub fn new(base: (f64, f64), lengths: Vec<f64>) -> Self {
        let mut joints = vec![base];
        let x = base.0;
        let mut y = base.1;
        // Extend upwards initially (assuming standard cartesian for physics, -Y for screen later)
        for length in &lengths {
            y += length;
            joints.push((x, y));
        }

        Self {
            joints,
            lengths,
            tolerance: 0.1,
        }
    }

    /// Solves Inverse Kinematics to reach the target using FABRIK
    pub fn solve(&mut self, target: (f64, f64)) {
        let total_length: f64 = self.lengths.iter().sum();
        let base = self.joints[0];
        let distance_to_target = dist(base, target);

        // Check if target is unreachable
        if distance_to_target > total_length {
            // Target is unreachable, stretch fully
            for i in 0..self.lengths.len() {
                let r = dist(target, self.joints[i]);
                let lambda = self.lengths[i] / r;
                self.joints[i+1] = (
                    (1.0 - lambda) * self.joints[i].0 + lambda * target.0,
                    (1.0 - lambda) * self.joints[i].1 + lambda * target.1,
                );
            }
        } else {
            // Target is reachable
            let mut diff = dist(self.joints.last().cloned().unwrap(), target);
            let mut iterations = 0;
            let max_iterations = 10; // FABRIK converges fast

            while diff > self.tolerance && iterations < max_iterations {
                // Backward Reaching
                let last_idx = self.joints.len() - 1;
                self.joints[last_idx] = target;
                for i in (0..last_idx).rev() {
                    let r = dist(self.joints[i+1], self.joints[i]);
                    let lambda = self.lengths[i] / r;
                    self.joints[i] = (
                        (1.0 - lambda) * self.joints[i+1].0 + lambda * self.joints[i].0,
                        (1.0 - lambda) * self.joints[i+1].1 + lambda * self.joints[i].1,
                    );
                }

                // Forward Reaching
                self.joints[0] = base;
                for i in 0..self.lengths.len() {
                    let r = dist(self.joints[i+1], self.joints[i]);
                    let lambda = self.lengths[i] / r;
                    self.joints[i+1] = (
                        (1.0 - lambda) * self.joints[i].0 + lambda * self.joints[i+1].0,
                        (1.0 - lambda) * self.joints[i].1 + lambda * self.joints[i+1].1,
                    );
                }

                diff = dist(self.joints.last().cloned().unwrap(), target);
                iterations += 1;
            }
        }
    }

    pub fn end_effector(&self) -> (f64, f64) {
        *self.joints.last().unwrap()
    }
}

fn dist(p1: (f64, f64), p2: (f64, f64)) -> f64 {
    ((p1.0 - p2.0).powi(2) + (p1.1 - p2.1).powi(2)).sqrt()
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_reach_target() {
        let lengths = vec![10.0, 10.0, 10.0];
        let mut arm = Arm::new((0.0, 0.0), lengths);

        // Target is reachable (30 units away max)
        let target = (15.0, 15.0);
        arm.solve(target);

        let end = arm.end_effector();
        let dist = ((end.0 - target.0).powi(2) + (end.1 - target.1).powi(2)).sqrt();

        // We use a slightly looser check in case iteration limit hit, but FABRIK is robust
        assert!(dist < arm.tolerance, "Arm did not reach target. Dist: {}, Tolerance: {}", dist, arm.tolerance);
    }

    #[test]
    fn test_unreachable_target() {
        let lengths = vec![10.0, 10.0];
        let mut arm = Arm::new((0.0, 0.0), lengths);

        // Target is far away
        let target = (50.0, 50.0);
        arm.solve(target);

        let end = arm.end_effector();
        let d = dist((0.0, 0.0), end);
        // It should be stretched to max length (20.0)
        assert!((d - 20.0).abs() < 0.1, "Arm did not stretch fully. Length: {}", d);
    }
}
