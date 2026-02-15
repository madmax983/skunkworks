use bevy::prelude::*;

#[derive(Debug, Clone)]
pub struct FabrikSolver {
    pub joints: Vec<Vec2>,
    pub lengths: Vec<f32>,
    pub tolerance: f32,
    pub max_iterations: usize,
}

impl FabrikSolver {
    pub fn new(start: Vec2, num_segments: usize, segment_len: f32) -> Self {
        let mut joints = Vec::with_capacity(num_segments + 1);
        let mut lengths = Vec::with_capacity(num_segments);

        let mut current = start;
        joints.push(current);

        for _ in 0..num_segments {
            current.x += segment_len;
            joints.push(current);
            lengths.push(segment_len);
        }

        Self {
            joints,
            lengths,
            tolerance: 0.1,
            max_iterations: 10,
        }
    }

    pub fn solve(&mut self, target: Vec2) {
        if self.joints.is_empty() {
            return;
        }

        let root = self.joints[0]; // Anchor
        let total_len: f32 = self.lengths.iter().sum();
        let dist = root.distance(target);

        // Unreachable
        if dist >= total_len {
            // Stretch towards target
            let dir = (target - root).normalize_or_zero();
            let dir = if dir == Vec2::ZERO { Vec2::X } else { dir };
            for i in 0..self.lengths.len() {
                self.joints[i+1] = self.joints[i] + dir * self.lengths[i];
            }
        } else {
            // Reachable - Iteration
            let last_idx = self.joints.len() - 1;

            // Check current error
            let mut diff = self.joints[last_idx].distance(target);
            let mut iter = 0;

            while diff > self.tolerance && iter < self.max_iterations {
                // BACKWARD: Set end to target
                self.joints[last_idx] = target;

                for i in (0..last_idx).rev() {
                    let _r = self.joints[i+1] - self.joints[i];
                    let len = self.lengths[i];
                    // We want joint i to be at distance 'len' from joint i+1 along vector r
                    // New P_i = P_{i+1} + (P_i - P_{i+1}) / dist * len
                    let vector_from_next = self.joints[i] - self.joints[i+1];
                    let dir = vector_from_next.normalize_or_zero();
                    let dir = if dir == Vec2::ZERO { Vec2::X } else { dir };

                    self.joints[i] = self.joints[i+1] + dir * len;
                }

                // FORWARD: Set root to original root
                self.joints[0] = root;
                for i in 0..self.lengths.len() {
                    // New P_{i+1} = P_i + (P_{i+1} - P_i).normalized * len
                    let vector_from_prev = self.joints[i+1] - self.joints[i];
                    let dir = vector_from_prev.normalize_or_zero();
                    let dir = if dir == Vec2::ZERO { Vec2::X } else { dir };
                    let len = self.lengths[i];

                    self.joints[i+1] = self.joints[i] + dir * len;
                }

                diff = self.joints[last_idx].distance(target);
                iter += 1;
            }
        }
    }
}


#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_fabrik_reach() {
        let mut solver = FabrikSolver::new(Vec2::ZERO, 2, 50.0);
        let target = Vec2::new(60.0, 40.0); // Reachable (dist approx 72 < 100)

        solver.solve(target);

        let end = *solver.joints.last().unwrap();
        assert!(end.distance(target) < 1.0, "Solver failed to reach target. Got {:?}, expected {:?}", end, target);

        // Verify lengths
        for i in 0..solver.lengths.len() {
            let d = solver.joints[i].distance(solver.joints[i+1]);
            assert!((d - solver.lengths[i]).abs() < 0.1, "Segment length violated at {}: {}", i, d);
        }
    }

    #[test]
    fn test_fabrik_stretch() {
        let mut solver = FabrikSolver::new(Vec2::ZERO, 2, 50.0);
        let target = Vec2::new(200.0, 0.0); // Unreachable

        solver.solve(target);

        let end = *solver.joints.last().unwrap();
        // Should be at (100, 0)
        assert!((end - Vec2::new(100.0, 0.0)).length() < 0.1, "Solver should stretch fully. Got {:?}", end);
    }
}
