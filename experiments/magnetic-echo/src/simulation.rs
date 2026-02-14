use rand::Rng;

#[derive(Clone, Copy, Debug, PartialEq)]
pub struct Rgba {
    pub r: f32,
    pub g: f32,
    pub b: f32,
    pub a: f32,
}

impl Rgba {
    pub fn new(r: f32, g: f32, b: f32, a: f32) -> Self {
        Self { r, g, b, a }
    }

    pub fn black() -> Self {
        Self::new(0.0, 0.0, 0.0, 1.0)
    }

    pub fn white() -> Self {
        Self::new(1.0, 1.0, 1.0, 1.0)
    }

    pub fn mix(&self, other: &Self, t: f32) -> Self {
        Self {
            r: self.r + (other.r - self.r) * t,
            g: self.g + (other.g - self.g) * t,
            b: self.b + (other.b - self.b) * t,
            a: self.a + (other.a - self.a) * t,
        }
    }
}

#[derive(Clone, Debug)]
pub struct Sector {
    pub color: Rgba,
    pub magnetism: f32,
    pub original_color: Rgba,
}

impl Sector {
    pub fn new(color: Rgba) -> Self {
        Self {
            color,
            magnetism: 1.0,
            original_color: color,
        }
    }
}

pub struct Platter {
    pub sectors: Vec<Sector>,
    pub width: usize,
    pub height: usize,
}

impl Platter {
    pub fn new(width: usize, height: usize) -> Self {
        let sectors = vec![Sector::new(Rgba::black()); width * height];
        Self {
            sectors,
            width,
            height,
        }
    }

    pub fn get_index(&self, x: usize, y: usize) -> Option<usize> {
        if x < self.width && y < self.height {
            Some(y * self.width + x)
        } else {
            None
        }
    }

    pub fn read(&self, x: usize, y: usize) -> Option<Rgba> {
        self.get_index(x, y).map(|idx| self.sectors[idx].color)
    }

    pub fn write(&mut self, x: usize, y: usize, color: Rgba) {
        if let Some(idx) = self.get_index(x, y) {
            self.sectors[idx].color = color;
            self.sectors[idx].magnetism = 1.0;
            // Note: We do NOT update original_color, as that represents the "truth" which is lost
            // or perhaps original_color is just for debugging/comparison.
            // If we are simulating writing new data, maybe we should update original?
            // But if we are simulating "Head refreshing data", we are overwriting with what we think is correct.
        }
    }

    pub fn decay(&mut self, rate: f32) {
        let mut rng = rand::thread_rng();
        for sector in &mut self.sectors {
            // Random decay
            if rng.gen::<f32>() < 0.1 {
                sector.magnetism -= rate * rng.gen::<f32>();
            }
            if sector.magnetism < 0.0 {
                sector.magnetism = 0.0;
            }
        }
    }

    pub fn drift(&mut self, noise_amount: f32) {
        let mut rng = rand::thread_rng();
        for sector in &mut self.sectors {
            if sector.magnetism < 0.9 {
                // The lower the magnetism, the more it drifts
                let instability = 1.0 - sector.magnetism;
                if rng.gen::<f32>() < instability * 0.1 {
                    // Drift towards noise
                    let noise = Rgba::new(rng.gen(), rng.gen(), rng.gen(), 1.0);
                    sector.color = sector.color.mix(&noise, noise_amount * instability);
                }
            }
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_platter_creation() {
        let platter = Platter::new(10, 10);
        assert_eq!(platter.sectors.len(), 100);
        assert_eq!(platter.read(0, 0).unwrap(), Rgba::black());
    }

    #[test]
    fn test_write() {
        let mut platter = Platter::new(10, 10);
        let color = Rgba::new(1.0, 0.0, 0.0, 1.0);
        platter.write(5, 5, color);
        assert_eq!(platter.read(5, 5).unwrap(), color);
    }

    #[test]
    fn test_decay() {
        let mut platter = Platter::new(10, 10);
        platter.decay(0.5);
        // It's random, but at least one should decay or stay 1.0
        // Let's force decay by running it many times or checking logic
        let mut changed = false;
        for _ in 0..100 {
            platter.decay(0.5);
        }
        for sector in &platter.sectors {
            if sector.magnetism < 1.0 {
                changed = true;
                break;
            }
        }
        assert!(changed, "Decay should eventually reduce magnetism");
    }
}
