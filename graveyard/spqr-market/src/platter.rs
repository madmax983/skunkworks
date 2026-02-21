#[derive(Debug, Clone)]
pub struct Platter {
    pub magnetism: Vec<f32>,
    pub width: usize,
    pub height: usize,
}

impl Platter {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            magnetism: vec![0.0; width * height],
            width,
            height,
        }
    }

    pub fn magnetize(&mut self, x: usize, y: usize, amount: f32) {
        if x < self.width && y < self.height {
            let idx = y * self.width + x;
            self.magnetism[idx] = (self.magnetism[idx] + amount).min(1.0);
        }
    }

    pub fn get_magnetism(&self, x: usize, y: usize) -> f32 {
        if x < self.width && y < self.height {
            self.magnetism[y * self.width + x]
        } else {
            0.0
        }
    }

    pub fn decay(&mut self, rate: f32) {
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
        assert_eq!(platter.get_magnetism(5, 5), 0.5);

        platter.magnetize(5, 5, 0.6);
        assert_eq!(platter.get_magnetism(5, 5), 1.0); // Clamped

        platter.decay(0.5);
        assert_eq!(platter.get_magnetism(5, 5), 0.5);
    }
}
