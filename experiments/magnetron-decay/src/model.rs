use rand::Rng;

#[derive(Clone, Debug)]
pub struct Sector {
    pub data: [u8; 64],
    pub original_data: [u8; 64],
    pub magnetization: f32, // 0.0 to 1.0 (1.0 = perfect signal)
    pub coercivity: f32,    // 0.0 to 1.0 (Resistance to flip)
    pub address: usize,
}

impl Sector {
    pub fn new(address: usize, data: [u8; 64]) -> Self {
        Self {
            data,
            original_data: data,
            magnetization: 1.0,
            coercivity: 1.0,
            address,
        }
    }

    pub fn decay(&mut self, dt: f32, rng: &mut impl Rng) {
        // Natural decay of signal
        // Decay rate depends on coercivity (lower coercivity = faster decay)
        // dt is simulated time.

        let decay_rate = 0.005 * (2.0 - self.coercivity);
        self.magnetization -= decay_rate * dt;
        self.magnetization = self.magnetization.max(0.0);

        // Probability of bit flip increases as magnetization drops
        // If magnetization is < 0.6, danger zone.
        if self.magnetization < 0.6 {
            // The probability needs to be per-frame-ish but significant enough to see
            let danger = 0.6 - self.magnetization;
            // Higher danger = higher probability
            let flip_prob = danger * 0.01 * dt;

            for i in 0..64 {
                // Check if we should flip a bit in this byte
                if rng.gen::<f32>() < flip_prob {
                    // FLIP A RANDOM BIT
                    let bit = rng.gen_range(0..8);
                    self.data[i] ^= 1 << bit;
                }
            }
        }
    }

    pub fn scrub(&mut self) {
        // Refresh signal, but damage media
        // Reading amplifies the signal back to max, but the physical medium degrades
        self.magnetization = 1.0;
        self.coercivity -= 0.02; // Permanent damage
        self.coercivity = self.coercivity.max(0.1);

        // Note: Scrubbing does NOT correct data! It just strengthens the signal of whatever bits are there.
        // To correct data, you would need to Write.
    }

    pub fn write(&mut self, data: [u8; 64]) {
        self.data = data;
        self.magnetization = 1.0;
        // Writing is stressful
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
        let mut rng = rand::thread_rng();

        // Initialize with some "Lorem Ipsum" and code-like data to make decay visible
        let corpus = "
The sky above the port was the color of television, tuned to a dead channel.
It's not like I'm using,' Case heard someone say, as he shouldered his way through the crowd around the door of the Chat. 'It's like my body's developed this massive drug deficiency.' It was a Sprawl voice and a Sprawl joke. The Chat was a bar for professional expatriates; you could drink there for a week and never hear two words in Japanese.
Ratz was tending bar. His prosthetic arm jerked monotonously as he filled a tray of glasses with draft Kirin. He saw Case and smiled, his teeth a webwork of East European steel and brown decay. Case found a place at the bar, between the unlikely tan on one of Lonny Zone's whores and the crisp naval uniform of a tall African whose cheekbones were ridged with precise rows of tribal scars. 'Wage was in here early, with two joeboys,' Ratz said, shoving a draft across the bar with his good hand. 'Is some business with you, Case?'
Case shrugged. The girl to his right giggled and nudged him.
The bartender's smile widened. His ugliness was the stuff of legend. In an age of affordable beauty, there was something heraldic about his lack of it. The antique arm whined as he reached for another mug.
        ".as_bytes();

        for i in 0..count {
            let mut data = [0u8; 64];
            // Cycle through corpus
            for j in 0..64 {
                let corpus_idx = (i * 64 + j) % corpus.len();
                data[j] = corpus[corpus_idx];
            }

            // Add some random noise to 10% of sectors to simulate "Empty" or "Binary" space
            if rng.gen_bool(0.1) {
                rng.fill(&mut data);
            }

            sectors.push(Sector::new(i, data));
        }

        Self {
            sectors,
            width,
            height,
        }
    }

    pub fn update(&mut self, dt: f32) {
        let mut rng = rand::thread_rng();
        for sector in &mut self.sectors {
            sector.decay(dt, &mut rng);
        }
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_magnetization_decay() {
        let mut rng = rand::thread_rng();
        let mut sector = Sector::new(0, [0u8; 64]);
        sector.magnetization = 1.0;

        // Simulate a lot of time
        sector.decay(10.0, &mut rng);

        assert!(sector.magnetization < 1.0);
    }

    #[test]
    fn test_bit_flip() {
        let mut rng = rand::thread_rng();
        let mut sector = Sector::new(0, [0u8; 64]);

        // Force critical condition
        sector.magnetization = 0.0;

        // Simulate time
        // High time step to ensure flips
        sector.decay(100.0, &mut rng);

        // Check if data changed
        assert_ne!(
            sector.data, [0u8; 64],
            "Bits should have flipped given zero magnetization and high dt"
        );
    }
}
