use rand::Rng;

#[derive(Clone, Debug)]
pub struct Sector {
    pub magnetization: f32, // 0.0 to 1.0 (1.0 = perfect signal)
    pub coercivity: f32,    // 0.0 to 1.0 (Resistance to flip)
    pub address: usize,
}

impl Sector {
    pub fn new(address: usize) -> Self {
        Self {
            magnetization: 1.0,
            coercivity: 1.0,
            address,
        }
    }

    pub fn decay(&mut self, dt: f32) -> bool {
        // Returns true if a "flip" (mutation) should occur

        // Natural decay of signal
        // Decay rate depends on coercivity (lower coercivity = faster decay)
        let decay_rate = 0.005 * (2.0 - self.coercivity);
        self.magnetization -= decay_rate * dt;
        self.magnetization = self.magnetization.max(0.0);

        // Probability of bit flip (mutation) increases as magnetization drops
        if self.magnetization < 0.6 {
            let danger = 0.6 - self.magnetization;
            // Scale probability by dt
            let mut rng = rand::thread_rng();
            // 1% per second at max danger? Let's make it more aggressive for gameplay.
            let flip_prob = danger * 0.1 * dt;
            return rng.gen::<f32>() < flip_prob;
        }
        false
    }

    pub fn scrub(&mut self) {
        // Refresh signal, but damage medium
        self.magnetization = 1.0;
        self.coercivity -= 0.02;
        self.coercivity = self.coercivity.max(0.1);
    }

    pub fn write_stress(&mut self) {
        // Writing is stressful but restores signal
        self.magnetization = 1.0;
        self.coercivity -= 0.01;
        self.coercivity = self.coercivity.max(0.1);
    }
}

pub struct Platter {
    pub sectors: Vec<Sector>,
    pub width: usize,
    pub height: usize,
}

impl Platter {
    pub fn new(width: usize, height: usize) -> Self {
        let count = width * height;
        let mut sectors = Vec::with_capacity(count);

        for i in 0..count {
            sectors.push(Sector::new(i));
        }

        Self {
            sectors,
            width,
            height,
        }
    }

    /// Update physics for all sectors.
    /// Returns a list of (x, y) coordinates where mutation events triggered.
    pub fn update(&mut self, dt: f32) -> Vec<(usize, usize)> {
        let mut mutations = Vec::new();
        for y in 0..self.height {
            for x in 0..self.width {
                if let Some(sector) = self.get_sector_mut(x, y) {
                    if sector.decay(dt) {
                        mutations.push((x, y));
                    }
                }
            }
        }
        mutations
    }

    pub fn get_sector(&self, x: usize, y: usize) -> Option<&Sector> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let idx = y * self.width + x;
        self.sectors.get(idx)
    }

    pub fn get_sector_mut(&mut self, x: usize, y: usize) -> Option<&mut Sector> {
        if x >= self.width || y >= self.height {
            return None;
        }
        let idx = y * self.width + x;
        self.sectors.get_mut(idx)
    }
}
