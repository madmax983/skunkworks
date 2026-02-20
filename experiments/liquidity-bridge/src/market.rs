//! # Market Simulator: The Physics of Finance 📈
//!
//! Adapted for Liquidity Bridge Experiment.
//!
//! The market grid now has **Terrain**:
//! * **Solid**: Standard market ether (Bids/Asks move freely).
//! * **Gap**: The Spread. Bids/Asks cannot enter unless...
//! * **Bridge**: Built by Ants. Allows Bids/Asks to cross.

use rand::Rng;

/// The default duration (in frames) that a Trade particle persists.
pub const DEFAULT_TRADE_AGE: u8 = 5;

#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Terrain {
    Solid,
    Gap,
    Bridge,
}

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
        age: u8,
    },
}

#[derive(Clone, Copy, Debug)]
pub struct Cell {
    pub particle: Particle,
    pub terrain: Terrain,
}

impl Cell {
    pub fn new(terrain: Terrain) -> Self {
        Self {
            particle: Particle::Empty,
            terrain,
        }
    }
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
    pub width: usize,
    pub height: usize,
    pub cells: Vec<Cell>,

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
        let mut cells = Vec::with_capacity(width * height);
        for _ in 0..(width * height) {
            cells.push(Cell::new(Terrain::Solid));
        }

        Self {
            width,
            height,
            cells,
            trade_count: 0,
            total_bids: 0,
            total_asks: 0,
            center_of_mass: height as f32 / 2.0,
            updated: vec![false; width * height],
            scan_x: (0..width).collect(),
        }
    }

    pub fn get_particle(&self, x: usize, y: usize) -> Particle {
        if x >= self.width || y >= self.height {
            return Particle::Empty;
        }
        self.cells[y * self.width + x].particle
    }

    pub fn get_terrain(&self, x: usize, y: usize) -> Terrain {
        if x >= self.width || y >= self.height {
            return Terrain::Gap; // Out of bounds is a gap
        }
        self.cells[y * self.width + x].terrain
    }

    pub fn set_particle(&mut self, x: usize, y: usize, p: Particle) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x].particle = p;
        }
    }

    pub fn set_terrain(&mut self, x: usize, y: usize, t: Terrain) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x].terrain = t;
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
                if let Particle::Bid(owner) = self.cells[idx].particle {
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
                if let Particle::Ask(owner) = self.cells[idx].particle {
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

    fn can_move_into(&self, idx: usize) -> bool {
        let cell = &self.cells[idx];
        match cell.terrain {
            Terrain::Solid => true,
            Terrain::Bridge => true,
            Terrain::Gap => false,
        }
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

                // Check if target is accessible (Terrain check) AND empty
                if self.can_move_into(n_idx)
                   && !self.updated[n_idx]
                   && matches!(self.cells[n_idx].particle, Particle::Empty)
                {
                    self.cells[n_idx].particle = particle;
                    self.cells[current_idx].particle = Particle::Empty;
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

            // Terrain Check
            if !self.can_move_into(target_idx) {
                // Blocked by Gap - Try sideways or stay put?
                // If blocked by gap, maybe try sideways to find a bridge?
                self.try_move_sideways(idx, x, y, Particle::Bid(owner), rng);
                return None;
            }

            match self.cells[target_idx].particle {
                Particle::Empty => {
                    self.cells[target_idx].particle = Particle::Bid(owner);
                    self.cells[idx].particle = Particle::Empty;
                    self.updated[target_idx] = true;
                }
                Particle::Ask(seller) => {
                    // Collision!
                    self.cells[target_idx].particle = Particle::Trade {
                        age: DEFAULT_TRADE_AGE,
                    };
                    self.cells[idx].particle = Particle::Empty;
                    self.updated[target_idx] = true;
                    return Some(TradeEvent {
                        buyer: owner,
                        seller,
                        price: (self.height - 1 - (y - 1)) as f32,
                    });
                }
                _ => {
                    self.try_move_sideways(idx, x, y, Particle::Bid(owner), rng);
                }
            }
        } else {
            // Reached top (Maximum Price), expire.
            self.cells[idx].particle = Particle::Empty;
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

            // Terrain Check
            if !self.can_move_into(target_idx) {
                // Blocked by Gap
                 self.try_move_sideways(idx, x, y, Particle::Ask(owner), rng);
                return None;
            }

            match self.cells[target_idx].particle {
                Particle::Empty => {
                    self.cells[target_idx].particle = Particle::Ask(owner);
                    self.cells[idx].particle = Particle::Empty;
                    self.updated[target_idx] = true;
                }
                Particle::Bid(buyer) => {
                    // Collision!
                    self.cells[target_idx].particle = Particle::Trade {
                        age: DEFAULT_TRADE_AGE,
                    };
                    self.cells[idx].particle = Particle::Empty;
                    self.updated[target_idx] = true;
                    return Some(TradeEvent {
                        buyer,
                        seller: owner,
                        price: (self.height - 1 - (y + 1)) as f32,
                    });
                }
                _ => {
                    self.try_move_sideways(idx, x, y, Particle::Ask(owner), rng);
                }
            }
        } else {
            // Reached bottom (Minimum Price), expire.
            self.cells[idx].particle = Particle::Empty;
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
                match self.cells[idx].particle {
                    Particle::Trade { age } => {
                        if age > 0 {
                            self.cells[idx].particle = Particle::Trade { age: age - 1 };
                        } else {
                            self.cells[idx].particle = Particle::Empty;
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_gap_blocking() {
        let mut grid = Grid::new(10, 10);

        // Create a gap at y=5
        for x in 0..10 {
            grid.set_terrain(x, 5, Terrain::Gap);
        }

        // Place a Bid at (5, 6)
        grid.set_particle(5, 6, Particle::Bid(1));

        // Update
        grid.update();

        // Bid should be blocked by Gap at y=5, so it stays at y=6 (or moves sideways)
        // Check row 5 is still empty
        match grid.get_particle(5, 5) {
            Particle::Empty => {},
            _ => panic!("Bid crossed the Gap!"),
        }
    }

    #[test]
    fn test_bridge_crossing() {
        let mut grid = Grid::new(10, 10);

        // Create a gap at y=5
        for x in 0..10 {
            grid.set_terrain(x, 5, Terrain::Gap);
        }

        // Build a bridge at (5, 5)
        grid.set_terrain(5, 5, Terrain::Bridge);

        // Place a Bid at (5, 6)
        grid.set_particle(5, 6, Particle::Bid(1));

        // Update
        grid.update();

        // Bid should move into the Bridge at (5, 5)
        match grid.get_particle(5, 5) {
            Particle::Bid(1) => {}, // Success
            _ => {
                // It might have been blocked if update order was weird, but let's see.
                // Or maybe it moved sideways?
                // But (5,5) is the only valid move (Bridge).
                // Sideways from (5,6) are (4,6) and (6,6).
                // If Bid moves sideways, it fails test.
                // But process_bid prefers UP if possible.
                // UP is (5,5). Terrain is Bridge. Allowed.
                panic!("Bid failed to cross the Bridge! Particle at (5,5): {:?}", grid.get_particle(5, 5));
            }
        }
    }
}
