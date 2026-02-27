#[derive(Debug, Clone)]
pub struct Platter {
    pub magnetism: Vec<f64>,
    pub width: usize,
    pub height: usize,
}

impl Platter {
    pub fn new(width: usize, height: usize) -> Self {
        let size = width.checked_mul(height).expect("Platter size overflow");
        Self {
            magnetism: vec![0.0; size],
            width,
            height,
        }
    }

    pub fn magnetize(&mut self, x: usize, y: usize, amount: f64) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.magnetism[idx] = (self.magnetism[idx] + amount).min(1.0);
        }
    }

    pub fn accumulate(&mut self, x: usize, y: usize, amount: f64) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.magnetism[idx] += amount;
        }
    }

    pub fn get_magnetism(&self, x: usize, y: usize) -> f64 {
        if x < self.width && y < self.height {
            self.magnetism[y * self.width + x]
        } else {
            0.0
        }
    }

    pub fn decay(&mut self, rate: f64) {
        for m in &mut self.magnetism {
            *m *= rate;
            if *m < 0.001 {
                *m = 0.0;
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnetize_decay() {
        let mut platter = Platter::new(10, 10);
        platter.magnetize(5, 5, 0.5);
        assert!((platter.get_magnetism(5, 5) - 0.5).abs() < 1e-6);

        platter.magnetize(5, 5, 0.6);
        assert!((platter.get_magnetism(5, 5) - 1.0).abs() < 1e-6); // Clamped

        platter.decay(0.5);
        assert!((platter.get_magnetism(5, 5) - 0.5).abs() < 1e-6);
    }

    #[test]
    fn test_accumulate() {
        let mut platter = Platter::new(10, 10);
        platter.accumulate(5, 5, 0.5);
        assert!((platter.get_magnetism(5, 5) - 0.5).abs() < 1e-6);

        platter.accumulate(5, 5, 0.6);
        assert!((platter.get_magnetism(5, 5) - 1.1).abs() < 1e-6); // Not clamped
    }
}
