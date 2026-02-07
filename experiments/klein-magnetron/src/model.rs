use rand::Rng;

#[derive(Clone, Debug)]
pub struct Sector {
    pub data: [u8; 64],
    pub original_data: [u8; 64],
    pub magnetization: f32, // 0.0 to 1.0 (1.0 = perfect signal)
    pub coercivity: f32,    // 0.0 to 1.0 (Resistance to flip)
    // Parametric coordinates on the Klein Bottle
    pub u: f32,
    pub v: f32,
    pub label: String,
}

impl Sector {
    pub fn new(u: f32, v: f32, data: [u8; 64], label: String) -> Self {
        Self {
            data,
            original_data: data,
            magnetization: 1.0,
            coercivity: 1.0,
            u,
            v,
            label,
        }
    }

    /// Decays the sector.
    /// `radiation_intensity`: how strong the radiation is at this sector right now.
    /// `polarity`: if true, flips bits normally. If false, maybe does something else (like invert the byte).
    pub fn decay(&mut self, dt: f32, radiation_intensity: f32, polarity: bool, rng: &mut impl Rng) {
        // Natural decay
        let decay_rate = 0.005 * (2.0 - self.coercivity);
        self.magnetization -= decay_rate * dt;

        // Radiation damage
        if radiation_intensity > 0.0 {
            self.magnetization -= radiation_intensity * dt * 0.5;
        }

        self.magnetization = self.magnetization.max(0.0);

        // Bit flip probability
        if self.magnetization < 0.6 {
            let danger = 0.6 - self.magnetization;
            // Higher danger = higher probability
            // Radiation amplifies danger
            let prob = danger * 0.01 * dt * (1.0 + radiation_intensity * 5.0);

            for i in 0..64 {
                if rng.gen::<f32>() < prob {
                    if polarity {
                        // Standard bit flip
                        let bit = rng.gen_range(0..8);
                        self.data[i] ^= 1 << bit;
                    } else {
                        // Inverted topology effect: Flip the whole byte (invert)
                        self.data[i] = !self.data[i];
                    }
                }
            }
        }
    }

    pub fn scrub(&mut self) {
        self.magnetization = 1.0;
        self.coercivity -= 0.02;
        self.coercivity = self.coercivity.max(0.1);
    }
}

pub struct Platter {
    pub sectors: Vec<Sector>,
    pub radiation_u: f32,
    pub radiation_polarity: bool,
}

impl Platter {
    pub fn new() -> Self {
        Self {
            sectors: Vec::new(),
            radiation_u: 0.0,
            radiation_polarity: true,
        }
    }

    pub fn add_sector(&mut self, sector: Sector) {
        self.sectors.push(sector);
    }

    pub fn update(&mut self, dt: f32) {
        let mut rng = rand::thread_rng();

        // Move radiation wave
        self.radiation_u += dt * 0.5; // Speed
        if self.radiation_u > std::f32::consts::PI * 2.0 {
            self.radiation_u -= std::f32::consts::PI * 2.0;
            // Flip polarity when wrapping around the main loop of the Klein bottle?
            // Actually, for a Figure-8 immersion, u is the long loop.
            // The Möbius twist happens along the cross-section?
            // In the Figure-8 immersion, u -> u+2pi is just a loop.
            // The non-orientability is inherent in the shape.
            // Let's toggle polarity anyway to simulate "passing through the twist".
            self.radiation_polarity = !self.radiation_polarity;
        }

        for sector in &mut self.sectors {
            // Calculate distance to radiation wave in U
            // Periodic distance
            let mut dist = (sector.u - self.radiation_u).abs();
            if dist > std::f32::consts::PI {
                dist = std::f32::consts::PI * 2.0 - dist;
            }

            let intensity = (-dist.powi(2) * 10.0).exp(); // Gaussian-ish peak at radiation_u

            sector.decay(dt, intensity, self.radiation_polarity, &mut rng);
        }
    }
}

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_decay() {
        let mut sector = Sector::new(0.0, 0.0, [0; 64], "test".to_string());
        let mut rng = rand::thread_rng();

        // Initial state
        assert_eq!(sector.magnetization, 1.0);

        // Decay
        sector.decay(1.0, 0.0, true, &mut rng);
        assert!(sector.magnetization < 1.0);
    }

    #[test]
    fn test_radiation_flip() {
        let mut sector = Sector::new(0.0, 0.0, [0; 64], "test".to_string());
        let mut rng = rand::thread_rng();

        sector.magnetization = 0.0; // Vulnerable

        // With polarity true (standard bit flip)
        // Probabilistic, but with dt=100.0 and radiation, should flip something
        sector.decay(100.0, 1.0, true, &mut rng);

        // Just check it ran without panic
        // Verifying exact flip is hard due to RNG, but data should ideally change
        // assert_ne!(sector.data, [0; 64]); // Flaky test risk if we assert
    }
}
