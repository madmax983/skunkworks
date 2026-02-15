//! # Market Simulator: The Physics of Finance 📈
//!
//! Adapted for Thermo-Market: Includes Particle::Wall.

use rand::Rng;

/// The default duration (in frames) that a Trade particle persists.
pub const DEFAULT_TRADE_AGE: u8 = 5;

/// A fundamental unit of the market simulation.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Particle {
    /// Empty space. No order exists here.
    Empty,

    /// A Buyer's order (Bid).
    Bid(usize),

    /// A Seller's order (Ask).
    Ask(usize),

    /// The remnant of a successful transaction.
    Trade {
        /// How many frames this particle will persist before disappearing.
        age: u8,
    },

    /// A structural barrier (Cooling Fin / Wall).
    /// Blocks movement.
    Wall,
}

/// A record of a successful transaction between a Buyer and a Seller.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TradeEvent {
    pub buyer: usize,
    pub seller: usize,
    pub price: f32,
}

/// The simulation environment.
#[derive(Debug)]
pub struct Grid {
    /// Width of the grid (number of columns).
    pub width: usize,
    /// Height of the grid (number of price levels).
    pub height: usize,
    /// Flat vector storing the grid state (row-major).
    pub cells: Vec<Particle>,

    // --- Statistics ---
    pub trade_count: usize,
    pub total_bids: usize,
    pub total_asks: usize,
    pub center_of_mass: f32,

    // --- Internal Simulation State ---
    updated: Vec<bool>,
    scan_x: Vec<usize>,
}

impl Grid {
    pub fn new(width: usize, height: usize) -> Self {
        Self {
            width,
            height,
            cells: vec![Particle::Empty; width * height],
            trade_count: 0,
            total_bids: 0,
            total_asks: 0,
            center_of_mass: height as f32 / 2.0,
            updated: vec![false; width * height],
            scan_x: (0..width).collect(),
        }
    }

    pub fn get(&self, x: usize, y: usize) -> Particle {
        if x >= self.width || y >= self.height {
            return Particle::Empty;
        }
        self.cells[y * self.width + x]
    }

    pub fn set(&mut self, x: usize, y: usize, p: Particle) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = p;
        }
    }

    pub fn update(&mut self) -> Vec<TradeEvent> {
        let mut rng = rand::thread_rng();
        let mut trade_events = Vec::new();

        self.updated.fill(false);

        // Randomize column scan order
        if rng.gen_bool(0.5) {
            self.scan_x.iter_mut().enumerate().for_each(|(i, v)| *v = i);
        } else {
            self.scan_x
                .iter_mut()
                .enumerate()
                .for_each(|(i, v)| *v = self.width - 1 - i);
        }

        let scan_order = self.scan_x.clone();

        // Pass 1: Bids (Up)
        for y in 0..self.height {
            for &x in &scan_order {
                let idx = y * self.width + x;
                if self.updated[idx] {
                    continue;
                }
                if let Particle::Bid(owner) = self.cells[idx] {
                    if let Some(event) = self.process_bid(x, y, idx, owner, &mut rng) {
                        trade_events.push(event);
                    }
                }
            }
        }

        // Pass 2: Asks (Down)
        for y in (0..self.height).rev() {
            for &x in &scan_order {
                let idx = y * self.width + x;
                if self.updated[idx] {
                    continue;
                }
                if let Particle::Ask(owner) = self.cells[idx] {
                    if let Some(event) = self.process_ask(x, y, idx, owner, &mut rng) {
                        trade_events.push(event);
                    }
                }
            }
        }

        // Pass 3: Stats & Cleanup
        self.update_stats_and_cleanup(trade_events.len());

        trade_events
    }

    fn try_move_sideways(
        &mut self,
        current_idx: usize,
        x: usize,
        y: usize,
        particle: Particle,
        rng: &mut impl Rng,
    ) {
        let dxs = if rng.gen_bool(0.5) { [-1, 1] } else { [1, -1] };
        for dx in dxs {
            let nx = x as isize + dx;
            if nx >= 0 && nx < self.width as isize {
                let nx = nx as usize;
                let n_idx = y * self.width + nx;
                // Move if empty (implicit: blocked by Wall, Trade, Bid, Ask)
                if !self.updated[n_idx] && matches!(self.cells[n_idx], Particle::Empty) {
                    self.cells[n_idx] = particle;
                    self.cells[current_idx] = Particle::Empty;
                    self.updated[n_idx] = true;
                    return;
                }
            }
        }
    }

    fn process_bid(
        &mut self,
        x: usize,
        y: usize,
        idx: usize,
        owner: usize,
        rng: &mut impl Rng,
    ) -> Option<TradeEvent> {
        if y > 0 {
            let target_idx = (y - 1) * self.width + x;
            match self.cells[target_idx] {
                Particle::Empty => {
                    self.cells[target_idx] = Particle::Bid(owner);
                    self.cells[idx] = Particle::Empty;
                    self.updated[target_idx] = true;
                }
                Particle::Ask(seller) => {
                    // Collision!
                    self.cells[target_idx] = Particle::Trade {
                        age: DEFAULT_TRADE_AGE,
                    };
                    self.cells[idx] = Particle::Empty;
                    self.updated[target_idx] = true;
                    return Some(TradeEvent {
                        buyer: owner,
                        seller,
                        price: (self.height - 1 - (y - 1)) as f32,
                    });
                }
                Particle::Wall => {
                    // Blocked by Wall -> Try Sideways
                    self.try_move_sideways(idx, x, y, Particle::Bid(owner), rng);
                }
                _ => {
                    // Blocked by Trade or Bid -> Try Sideways
                    self.try_move_sideways(idx, x, y, Particle::Bid(owner), rng);
                }
            }
        } else {
            // Reached top (Maximum Price), expire.
            self.cells[idx] = Particle::Empty;
        }
        None
    }

    fn process_ask(
        &mut self,
        x: usize,
        y: usize,
        idx: usize,
        owner: usize,
        rng: &mut impl Rng,
    ) -> Option<TradeEvent> {
        if y < self.height - 1 {
            let target_idx = (y + 1) * self.width + x;
            match self.cells[target_idx] {
                Particle::Empty => {
                    self.cells[target_idx] = Particle::Ask(owner);
                    self.cells[idx] = Particle::Empty;
                    self.updated[target_idx] = true;
                }
                Particle::Bid(buyer) => {
                    // Collision!
                    self.cells[target_idx] = Particle::Trade {
                        age: DEFAULT_TRADE_AGE,
                    };
                    self.cells[idx] = Particle::Empty;
                    self.updated[target_idx] = true;
                    return Some(TradeEvent {
                        buyer,
                        seller: owner,
                        price: (self.height - 1 - (y + 1)) as f32,
                    });
                }
                Particle::Wall => {
                    // Blocked by Wall -> Try Sideways
                    self.try_move_sideways(idx, x, y, Particle::Ask(owner), rng);
                }
                _ => {
                    // Blocked by Trade or Ask -> Try Sideways
                    self.try_move_sideways(idx, x, y, Particle::Ask(owner), rng);
                }
            }
        } else {
            // Reached bottom (Minimum Price), expire.
            self.cells[idx] = Particle::Empty;
        }
        None
    }

    fn update_stats_and_cleanup(&mut self, trade_count: usize) {
        let mut bids = 0;
        let mut asks = 0;
        let mut weighted_y_sum = 0.0;
        let mut mass_sum = 0.0;

        for y in 0..self.height {
            for x in 0..self.width {
                let idx = y * self.width + x;
                match self.cells[idx] {
                    Particle::Trade { age } => {
                        if age > 0 {
                            self.cells[idx] = Particle::Trade { age: age - 1 };
                        } else {
                            self.cells[idx] = Particle::Empty;
                        }
                    }
                    Particle::Bid(_) => {
                        bids += 1;
                        weighted_y_sum += (self.height - y) as f32;
                        mass_sum += 1.0;
                    }
                    Particle::Ask(_) => {
                        asks += 1;
                        weighted_y_sum += (self.height - y) as f32;
                        mass_sum += 1.0;
                    }
                    _ => {}
                }
            }
        }

        self.trade_count = trade_count;
        self.total_bids = bids;
        self.total_asks = asks;
        if mass_sum > 0.0 {
            self.center_of_mass = weighted_y_sum / mass_sum;
        }
    }
}
