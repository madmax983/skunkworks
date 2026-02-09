#[derive(Clone)]
pub struct EuclideanGenerator {
    pub steps: usize,
    pub pulses: usize,
    pub rotation: usize,
    pub pattern: Vec<bool>,
}

impl EuclideanGenerator {
    pub fn new(steps: usize, pulses: usize) -> Self {
        let mut gen = Self {
            steps,
            pulses,
            rotation: 0,
            pattern: Vec::new(),
        };
        gen.compute();
        gen
    }

    pub fn set_params(&mut self, steps: usize, pulses: usize) {
        if self.steps != steps || self.pulses != pulses {
            self.steps = steps;
            self.pulses = pulses;
            self.compute();
        }
    }

    pub fn set_rotation(&mut self, rotation: usize) {
        self.rotation = rotation;
    }

    pub fn get_beat_at(&self, step: usize) -> bool {
        if self.pattern.is_empty() {
            return false;
        }
        let idx = (step + self.rotation) % self.pattern.len();
        self.pattern[idx]
    }

    fn compute(&mut self) {
        if self.steps == 0 {
            self.pattern = vec![];
            return;
        }
        if self.pulses == 0 {
            self.pattern = vec![false; self.steps];
            return;
        }
        if self.pulses >= self.steps {
            self.pattern = vec![true; self.steps];
            return;
        }

        // Using Bresenham line algorithm to distribute pulses evenly
        // Initializing accumulator = steps ensures we start with a beat (Downbeat-first)
        self.pattern = Vec::with_capacity(self.steps);
        let mut accumulator = self.steps;

        for _ in 0..self.steps {
            if accumulator >= self.steps {
                accumulator -= self.steps;
                self.pattern.push(true);
            } else {
                self.pattern.push(false);
            }
            accumulator += self.pulses;
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_euclidean_5_13() {
        let gen = EuclideanGenerator::new(13, 5);
        let pattern: String = gen
            .pattern
            .iter()
            .map(|&b| if b { 'X' } else { '.' })
            .collect();
        println!("Pattern (5, 13): {}", pattern);
        assert_eq!(gen.pattern.iter().filter(|&&b| b).count(), 5);
        assert_eq!(gen.pattern.len(), 13);
        assert!(gen.pattern[0]);
    }

    #[test]
    fn test_euclidean_4_16() {
        let gen = EuclideanGenerator::new(16, 4);
        let pattern: String = gen
            .pattern
            .iter()
            .map(|&b| if b { 'X' } else { '.' })
            .collect();
        println!("Pattern (4, 16): {}", pattern);
        assert_eq!(gen.pattern.iter().filter(|&&b| b).count(), 4);
        assert!(gen.pattern[0]);
        assert!(gen.pattern[4]);
    }
}
