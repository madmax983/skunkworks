use glam::Vec2;

#[derive(Clone, Debug)]
pub struct IKChain {
    pub joints: Vec<Vec2>,
    pub lengths: Vec<f32>,
    pub tolerance: f32,
    pub max_iterations: usize,
}

impl IKChain {
    pub fn new(start: Vec2, num_segments: usize, segment_length: f32) -> Self {
        let mut joints = Vec::with_capacity(num_segments + 1);
        let mut lengths = Vec::with_capacity(num_segments);
        for i in 0..=num_segments {
            // Initialize pointing right
            joints.push(start + Vec2::new(i as f32 * segment_length, 0.0));
            if i < num_segments {
                lengths.push(segment_length);
            }
        }
        Self {
            joints,
            lengths,
            tolerance: 0.01,
            max_iterations: 10,
        }
    }

    pub fn solve_fabrik(&mut self, root: Vec2, target: Vec2) {
        // Distance check
        let total_len: f32 = self.lengths.iter().sum();
        let dist = root.distance(target);

        if dist >= total_len {
            // Target is out of reach - stretch completely
            let dir = (target - root).normalize_or_zero();
            self.joints[0] = root;
            for i in 0..self.lengths.len() {
                self.joints[i + 1] = self.joints[i] + dir * self.lengths[i];
            }
        } else {
            // Target is reachable
            let n = self.joints.len();

            // We usually start with the current configuration, but ensure root is at root
            self.joints[0] = root;

            let mut diff = self.joints.last().unwrap().distance(target);
            let mut iter = 0;

            while diff > self.tolerance && iter < self.max_iterations {
                // Forward reaching (Target -> Root)
                // Set end effector to target
                self.joints[n - 1] = target;

                for i in (0..n - 1).rev() {
                    let dist_curr_next = self.joints[i + 1].distance(self.joints[i]);
                    // Avoid division by zero if points overlap exactly
                    if dist_curr_next > 1e-5 {
                        let lambda = self.lengths[i] / dist_curr_next;
                        // Find position of i based on i+1
                        self.joints[i] =
                            (1.0 - lambda) * self.joints[i + 1] + lambda * self.joints[i];
                    }
                }

                // Backward reaching (Root -> Target)
                // Set root to original root position
                self.joints[0] = root;

                for i in 0..n - 1 {
                    let dist_curr_next = self.joints[i + 1].distance(self.joints[i]);
                    if dist_curr_next > 1e-5 {
                        let lambda = self.lengths[i] / dist_curr_next;
                        // Find position of i+1 based on i
                        self.joints[i + 1] =
                            (1.0 - lambda) * self.joints[i] + lambda * self.joints[i + 1];
                    }
                }

                diff = self.joints.last().unwrap().distance(target);
                iter += 1;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_ik_reachable() {
        let mut chain = IKChain::new(Vec2::ZERO, 2, 10.0);
        // Total length = 20. Target at (10, 10). Distance = 14.14 < 20. Reachable.
        let target = Vec2::new(10.0, 10.0);
        chain.solve_fabrik(Vec2::ZERO, target);

        let end_effector = *chain.joints.last().unwrap();
        assert!(
            end_effector.distance(target) < 0.1,
            "End effector {:.?} should be close to target {:.?}",
            end_effector,
            target
        );

        // Check lengths preserved
        for i in 0..chain.lengths.len() {
            let dist = chain.joints[i].distance(chain.joints[i + 1]);
            assert!(
                (dist - 10.0).abs() < 0.1,
                "Segment {} length {} changed",
                i,
                dist
            );
        }
    }

    #[test]
    fn test_ik_unreachable() {
        let mut chain = IKChain::new(Vec2::ZERO, 2, 10.0);
        // Target at (30, 0). Reach is 20. Should stretch.
        let target = Vec2::new(30.0, 0.0);
        chain.solve_fabrik(Vec2::ZERO, target);

        let end_effector = *chain.joints.last().unwrap();
        // Should be at (20, 0)
        assert!((end_effector.x - 20.0).abs() < 0.1);
        assert!((end_effector.y - 0.0).abs() < 0.1);
    }
}
