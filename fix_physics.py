import re

with open("experiments/fabric-limb/src/physics.rs", "r") as f:
    content = f.read()

# Replace solve logic
new_solve = """    pub fn solve(&mut self, target: (f64, f64)) {
        if self.joints.is_empty() {
            return;
        }

        let total_length: f64 = self.lengths.iter().sum();
        let base = self.joints[0];
        let distance_to_target = dist(base, target);

        // Check if target is unreachable
        if distance_to_target > total_length {
            // Target is unreachable, stretch fully
            let len = std::cmp::min(self.lengths.len(), self.joints.len().saturating_sub(1));
            for i in 0..len {
                let r = dist(target, self.joints[i]);
                if r > f64::EPSILON {
                    let lambda = self.lengths[i] / r;
                    self.joints[i + 1] = (
                        (1.0 - lambda) * self.joints[i].0 + lambda * target.0,
                        (1.0 - lambda) * self.joints[i].1 + lambda * target.1,
                    );
                }
            }
        } else {
            // Target is reachable
            let mut diff = dist(self.joints.last().cloned().unwrap_or(base), target);
            let mut iterations = 0;
            let max_iterations = 10; // FABRIK converges fast

            while diff > self.tolerance && iterations < max_iterations {
                // Backward Reaching
                let last_idx = self.joints.len() - 1;
                self.joints[last_idx] = target;
                let len = std::cmp::min(self.lengths.len(), last_idx);
                for i in (0..len).rev() {
                    let r = dist(self.joints[i + 1], self.joints[i]);
                    if r > f64::EPSILON {
                        let lambda = self.lengths[i] / r;
                        self.joints[i] = (
                            (1.0 - lambda) * self.joints[i + 1].0 + lambda * self.joints[i].0,
                            (1.0 - lambda) * self.joints[i + 1].1 + lambda * self.joints[i].1,
                        );
                    }
                }

                // Forward Reaching
                self.joints[0] = base;
                let len = std::cmp::min(self.lengths.len(), self.joints.len().saturating_sub(1));
                for i in 0..len {
                    let r = dist(self.joints[i + 1], self.joints[i]);
                    if r > f64::EPSILON {
                        let lambda = self.lengths[i] / r;
                        self.joints[i + 1] = (
                            (1.0 - lambda) * self.joints[i].0 + lambda * self.joints[i + 1].0,
                            (1.0 - lambda) * self.joints[i].1 + lambda * self.joints[i + 1].1,
                        );
                    }
                }

                diff = dist(self.joints.last().cloned().unwrap_or(base), target);
                iterations += 1;
            }
        }
    }"""

content = re.sub(r'    pub fn solve\(&mut self, target: \(f64, f64\)\) \{.*?(?=    pub fn end_effector)', new_solve + '\n\n', content, flags=re.DOTALL)

with open("experiments/fabric-limb/src/physics.rs", "w") as f:
    f.write(content)
