use crate::hologram::Hologram;
use crate::organism::{Organism, OpCode};
use rand::Rng;

pub struct Simulation {
    pub width: usize,
    pub height: usize,
    pub hologram: Hologram,
    pub organisms: Vec<Organism>,
    pub channel_cache: Vec<Vec<f64>>,
    pub tick: usize,
}

impl Simulation {
    pub fn new(width: usize, height: usize) -> Self {
        let mut sim = Self {
            width,
            height,
            hologram: Hologram::new(width, height),
            organisms: Vec::new(),
            channel_cache: Vec::new(),
            tick: 0,
        };

        // Seed Hologram with Genetic Information
        // Channel 0: Consume (Food)
        // Channel 1: Photosynthesize (Sun)
        // Channel 2: Divide (Reproduction)
        // Channel 3: Move (Migration)
        // Channel 4: None (Noise)

        let instructions = [
            ("CCCCCCCC", 0), // Consume everywhere
            ("PPPPPPPP", 1), // Photo everywhere
            ("DDDDDDDD", 2), // Divide everywhere
            ("MMMMMMMM", 3), // Move everywhere
        ];

        use std::f64::consts::PI;

        for (text, channel_idx) in instructions {
             let (bin_x, bin_y) = Hologram::get_channel_shift(channel_idx);

             // Convert bin shift to radians per pixel
             let rec_x = (bin_x as f64) * 2.0 * PI / (width as f64);
             let rec_y = (bin_y as f64) * 2.0 * PI / (height as f64);

             let layer = Hologram::from_text(text, width, height, rec_x, rec_y);
             sim.hologram.add_layer(&layer);
        }

        // Spawn Organisms
        for _ in 0..100 {
            let mut rng = rand::thread_rng();
            sim.organisms.push(Organism::new(
                rng.gen_range(0..width),
                rng.gen_range(0..height),
            ));
        }

        // Initialize cache
        for _ in 0..5 {
            sim.channel_cache.push(vec![0.0; width * height]);
        }

        sim
    }

    pub fn step(&mut self) {
        self.tick += 1;

        // Reconstruct all channels
        // In a real physics simulation, the hologram might change (if organisms write to it).
        // Here, the environment is static code, so we only need to reconstruct once.
        // But let's assume dynamic environment later.
        // For now, reconstruct every 100 ticks to save CPU?
        // Or just once at startup?
        // Let's reconstruct every tick to simulate "reading".

        let total_channels = 5;
        for i in 0..total_channels {
            self.channel_cache[i] = self.hologram.reconstruct_channel(i, total_channels);
        }

        let mut new_organisms = Vec::new();

        for org in &mut self.organisms {
            if org.halted { continue; }

            let image = &self.channel_cache[org.channel_idx % total_channels];
            let op = org.scan(image, self.width, self.height);

            if let Some(child) = org.step(self.width, self.height, op) {
                new_organisms.push(child);
            }
        }

        self.organisms.retain(|o| !o.halted);
        self.organisms.extend(new_organisms);

        // Repopulate if low
        if self.organisms.len() < 10 {
            let mut rng = rand::thread_rng();
            self.organisms.push(Organism::new(
                rng.gen_range(0..self.width),
                rng.gen_range(0..self.height),
            ));
        }
    }
}
