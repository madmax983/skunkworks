use locus::Vec2;

#[derive(Debug, Clone)]
pub struct Arm {
    pub joints: Vec<Vec2>,
    pub lengths: Vec<f64>,
    pub tolerance: f64,
}

impl Arm {
    pub fn new(base: Vec2, lengths: Vec<f64>) -> Self {
        let mut joints = vec![base];
        let mut current_pos = base;

        // Extend upwards initially (assuming standard cartesian for physics)
        // Note: In TUI, Y grows downwards, so we might want to extend downwards or upwards depending on where base is.
        // Let's stick to extending in +Y direction for now, layout will determine visual.
        for length in &lengths {
            current_pos.y += length;
            joints.push(current_pos);
        }

        Self {
            joints,
            lengths,
            tolerance: 0.1,
        }
    }

    /// Solves Inverse Kinematics to reach the target using FABRIK
    pub fn solve(&mut self, target: Vec2) {
        let total_length: f64 = self.lengths.iter().sum();
        let base = self.joints[0];
        let distance_to_target = base.distance(target);

        // Check if target is unreachable
        if distance_to_target > total_length {
            // Target is unreachable, stretch fully
            for i in 0..self.lengths.len() {
                let r = target.distance(self.joints[i]);
                let lambda = self.lengths[i] / r;
                let next_x = (1.0 - lambda) * self.joints[i].x + lambda * target.x;
                let next_y = (1.0 - lambda) * self.joints[i].y + lambda * target.y;
                self.joints[i + 1] = Vec2::new(next_x, next_y);
            }
        } else {
            // Target is reachable
            let mut diff = self.joints.last().unwrap().distance(target);
            let mut iterations = 0;
            let max_iterations = 10; // FABRIK converges fast

            while diff > self.tolerance && iterations < max_iterations {
                // Backward Reaching
                let last_idx = self.joints.len() - 1;
                self.joints[last_idx] = target;
                for i in (0..last_idx).rev() {
                    let r = self.joints[i + 1].distance(self.joints[i]);
                    let lambda = self.lengths[i] / r;
                    let next_x = (1.0 - lambda) * self.joints[i + 1].x + lambda * self.joints[i].x;
                    let next_y = (1.0 - lambda) * self.joints[i + 1].y + lambda * self.joints[i].y;
                    self.joints[i] = Vec2::new(next_x, next_y);
                }

                // Forward Reaching
                self.joints[0] = base;
                for i in 0..self.lengths.len() {
                    let r = self.joints[i + 1].distance(self.joints[i]);
                    let lambda = self.lengths[i] / r;
                    let next_x = (1.0 - lambda) * self.joints[i].x + lambda * self.joints[i + 1].x;
                    let next_y = (1.0 - lambda) * self.joints[i].y + lambda * self.joints[i + 1].y;
                    self.joints[i + 1] = Vec2::new(next_x, next_y);
                }

                diff = self.joints.last().unwrap().distance(target);
                iterations += 1;
            }
        }
    }

    pub fn end_effector(&self) -> Vec2 {
        *self.joints.last().unwrap()
    }
}
