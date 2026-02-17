use crate::mayan::MayanDate;
use market_sim::{Grid, Particle};
use rand::rngs::ThreadRng;
use rand::{Rng, thread_rng};

pub struct BabylonianMarket {
    pub grid: Grid,
    pub current_date: MayanDate,
    pub rng: ThreadRng,
}

impl BabylonianMarket {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            grid: Grid::new(width, height),
            // Start at 13.0.0.0.0 (End of the previous cycle)
            current_date: MayanDate::new(13, 0, 0, 0, 0),
            rng: thread_rng(),
        }
    }

    pub fn update(&mut self) {
        // Time flows: 1 day per tick
        let days = self.current_date.total_days();
        self.current_date = MayanDate::from_days(days + 1);

        // Update physics
        self.grid.update();

        // Astrological Trading
        self.astrological_trading();
    }

    fn astrological_trading(&mut self) {
        let kin = self.current_date.kin;
        let uinal = self.current_date.uinal;
        let _tun = self.current_date.tun;

        // "Lucky 13" Strategy: Bids (Buyers) enter on multiples of 13 kins
        if kin % 13 == 0 {
            self.inject_bids(3);
        }

        // "Cyclical 20" Strategy: Asks (Sellers) enter on multiples of 20 kins (new Uinal)
        if kin == 0 { // New Uinal (since kin goes 0-19)
             self.inject_asks(3);
        }

        // "Tun Volatility": Start of a new Tun (360 days) causes massive volume
        if uinal == 0 && kin == 0 {
            self.inject_bids(10);
            self.inject_asks(10);
        }

        // Random noise to keep the market alive
        if self.rng.gen_bool(0.1) {
            self.inject_bids(1);
        }
        if self.rng.gen_bool(0.1) {
            self.inject_asks(1);
        }
    }

    fn inject_bids(&mut self, count: usize) {
        for _ in 0..count {
            let x = self.rng.gen_range(0..self.grid.width);
            // Spawn bids at the bottom (Low Price)
            // But maybe slightly higher to simulate aggressive buying
            let y = self.grid.height - 1 - self.rng.gen_range(0..5);

            // Only overwrite Empty or old Trades (which are not blocking but should be checked)
            // Grid::set overwrites. But let's check Empty to avoid overwriting another Bid
            if matches!(self.grid.get(x, y), Particle::Empty) {
                 self.grid.set(x, y, Particle::Bid(self.rng.gen()));
            }
        }
    }

    fn inject_asks(&mut self, count: usize) {
        for _ in 0..count {
            let x = self.rng.gen_range(0..self.grid.width);
            // Spawn asks at the top (High Price)
            let y = self.rng.gen_range(0..5);

            if matches!(self.grid.get(x, y), Particle::Empty) {
                self.grid.set(x, y, Particle::Ask(self.rng.gen()));
            }
        }
    }
}
