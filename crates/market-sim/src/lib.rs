//! # Market Simulator: The Physics of Finance 📈
//!
//! This crate implements a **Continuous Double Auction (CDA)** mechanism using a particle system.
//!
//! Instead of a traditional order book (a list of numbers), the market is simulated as a physical 2D grid where:
//!
//! *   **Price** is represented by the Y-axis (Height).
//!     *   Top of the grid ($y=0$) = **High Price**.
//!     *   Bottom of the grid ($y=H-1$) = **Low Price**.
//! *   **Bids (Buyers)** are particles that spawn at the bottom (low price) and "bubble up" towards higher prices.
//!     *   This represents a buyer gradually increasing their offer to find a seller.
//! *   **Asks (Sellers)** are particles that spawn at the top (high price) and "fall down" towards lower prices.
//!     *   This represents a seller gradually lowering their asking price to find a buyer.
//!
//! ## The Interaction
//!
//! When a **Bid** (moving up) collides with an **Ask** (moving down), a transaction occurs!
//!
//! 1.  The two particles annihilate each other.
//! 2.  A `Trade` particle is created at the collision point (representing the execution price).
//! 3.  The `Trade` particle decays over time (visualized as a flash).
//! 4.  A `TradeEvent` is emitted.
//!
//! ## Example
//!
//! ```
//! use market_sim::{Grid, Particle};
//!
//! // Create a 10x100 market grid
//! let mut market = Grid::new(10, 100);
//!
//! // Place a Buyer (Bid) at the bottom (Low Price)
//! market.set(5, 99, Particle::Bid(1)); // Buyer ID 1
//!
//! // Place a Seller (Ask) at the top (High Price)
//! market.set(5, 0, Particle::Ask(2)); // Seller ID 2
//!
//! // Run the simulation loop
//! loop {
//!     let trades = market.update();
//!
//!     // Eventually, they will meet in the middle!
//!     if !trades.is_empty() {
//!         let t = &trades[0];
//!         println!("Trade executed! Buyer {} bought from Seller {} at price ${}",
//!             t.buyer, t.seller, t.price);
//!         break;
//!     }
//! }
//! ```

use rand::Rng;

/// The default duration (in frames) that a Trade particle persists.
pub const DEFAULT_TRADE_AGE: u8 = 5;

/// A fundamental unit of the market simulation.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Particle {
    /// Empty space. No order exists here.
    Empty,

    /// A Buyer's order (Bid).
    ///
    /// * **Movement**: Up (decreases Y index).
    /// * **Goal**: Reach higher prices (lower Y).
    /// * **Payload**: Contains the `ID` of the buyer.
    Bid(usize),

    /// A Seller's order (Ask).
    ///
    /// * **Movement**: Down (increases Y index).
    /// * **Goal**: Reach lower prices (higher Y).
    /// * **Payload**: Contains the `ID` of the seller.
    Ask(usize),

    /// The remnant of a successful transaction.
    ///
    /// Created when a `Bid` and `Ask` collide. It decays over time,
    /// serving as a visual indicator of market activity.
    Trade {
        /// How many frames this particle will persist before disappearing.
        age: u8,
    },

    /// A structural barrier.
    /// Blocks movement.
    Wall,
}

/// A record of a successful transaction between a Buyer and a Seller.
#[derive(Debug, Clone, Copy, PartialEq)]
pub struct TradeEvent {
    /// The ID of the buyer (from the `Bid` particle).
    pub buyer: usize,
    /// The ID of the seller (from the `Ask` particle).
    pub seller: usize,
    /// The price at which the trade occurred.
    ///
    /// Calculated from the Y-coordinate of the collision:
    /// $Price = Height - 1 - Y$
    pub price: f32,
}

/// The simulation environment.
///
/// Represents the market as a 2D grid of particles.
///
/// *   **Width**: Represents simulated time or parallel order streams.
/// *   **Height**: Represents the Price axis.
#[derive(Debug)]
pub struct Grid {
    /// Width of the grid (number of columns).
    pub width: usize,
    /// Height of the grid (number of price levels).
    pub height: usize,
    /// Flat vector storing the grid state (row-major).
    pub cells: Vec<Particle>,

    // --- Statistics ---
    /// Number of trades that occurred in the last update.
    pub trade_count: usize,
    /// Total number of active Bids currently in the grid.
    pub total_bids: usize,
    /// Total number of active Asks currently in the grid.
    pub total_asks: usize,
    /// The "Center of Mass" of the market activity (weighted average price).
    pub center_of_mass: f32,

    // --- Internal Simulation State ---
    /// Tracks which cells have been updated in the current tick to prevent double-movement.
    updated: Vec<bool>,
    /// Randomized column iteration order to prevent directional bias.
    scan_x: Vec<usize>,
}

impl Grid {
    /// Creates a new, empty market grid.
    ///
    /// # Arguments
    ///
    /// * `width` - Number of columns.
    /// * `height` - Number of rows (price levels).
    ///
    /// # Examples
    ///
    /// ```
    /// use market_sim::Grid;
    /// let market = Grid::new(20, 100);
    /// assert_eq!(market.width, 20);
    /// ```
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

    /// Retrieves the particle at the specified coordinates.
    ///
    /// Returns `Particle::Empty` if coordinates are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use market_sim::{Grid, Particle};
    /// let mut grid = Grid::new(5, 5);
    /// grid.set(0, 0, Particle::Bid(1));
    /// assert_eq!(grid.get(0, 0), Particle::Bid(1));
    /// assert_eq!(grid.get(10, 10), Particle::Empty); // Out of bounds
    /// ```
    pub fn get(&self, x: usize, y: usize) -> Particle {
        if x >= self.width || y >= self.height {
            return Particle::Empty;
        }
        self.cells[y * self.width + x]
    }

    /// Sets the particle at the specified coordinates.
    ///
    /// Does nothing if coordinates are out of bounds.
    ///
    /// # Examples
    ///
    /// ```
    /// use market_sim::{Grid, Particle};
    /// let mut grid = Grid::new(5, 5);
    /// grid.set(1, 1, Particle::Ask(2));
    /// assert_eq!(grid.get(1, 1), Particle::Ask(2));
    /// ```
    pub fn set(&mut self, x: usize, y: usize, p: Particle) {
        if x < self.width && y < self.height {
            self.cells[y * self.width + x] = p;
        }
    }

    /// Advances the simulation by one time step.
    ///
    /// This method performs three passes:
    ///
    /// 1.  **Bids (Up)**: Moves `Bid` particles up (y-1). If they hit an `Ask`, a trade occurs.
    /// 2.  **Asks (Down)**: Moves `Ask` particles down (y+1). If they hit a `Bid`, a trade occurs.
    /// 3.  **Stats**: Calculates market statistics and decays `Trade` particles.
    ///
    /// Returns a list of `TradeEvent`s that occurred during this step.
    ///
    /// # Stochastic Movement
    ///
    /// If a particle is blocked by another particle of the same type, it may attempt to move
    /// sideways (left or right) to find a path around it, simulating market "noise" or
    /// searching for liquidity.
    pub fn update(&mut self) -> Vec<TradeEvent> {
        let mut rng = rand::thread_rng();
        let mut trade_events = Vec::new();

        self.updated.fill(false);

        // Randomize column scan order to prevent left-bias or right-bias in movement
        if rng.gen_bool(0.5) {
            self.scan_x.iter_mut().enumerate().for_each(|(i, v)| *v = i);
        } else {
            self.scan_x
                .iter_mut()
                .enumerate()
                .for_each(|(i, v)| *v = self.width - 1 - i);
        }

        // Clone scan_x to avoid borrowing self while mutating it in the loop
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
                    self.try_move_sideways(idx, x, y, Particle::Bid(owner), rng);
                }
                _ => {
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
                    self.try_move_sideways(idx, x, y, Particle::Ask(owner), rng);
                }
                _ => {
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

#[cfg(test)]
mod tests {
    use super::*;

    #[test]
    fn test_bid_movement() {
        let mut grid = Grid::new(10, 10);
        // Place a bid at (5, 5)
        grid.set(5, 5, Particle::Bid(1));

        grid.update();

        // Should move UP (y - 1) -> (5, 4)
        assert_eq!(grid.get(5, 4), Particle::Bid(1));
        assert_eq!(grid.get(5, 5), Particle::Empty);
    }

    #[test]
    fn test_ask_movement() {
        let mut grid = Grid::new(10, 10);
        // Place an ask at (5, 5)
        grid.set(5, 5, Particle::Ask(2));

        grid.update();

        // Should move DOWN (y + 1) -> (5, 6)
        assert_eq!(grid.get(5, 6), Particle::Ask(2));
        assert_eq!(grid.get(5, 5), Particle::Empty);
    }

    #[test]
    fn test_collision_move_into_each_other() {
        let mut grid = Grid::new(10, 10);
        // Bid at (5, 5). Moves to 4.
        grid.set(5, 5, Particle::Bid(1));
        // Ask at (5, 3). Moves to 4.
        grid.set(5, 3, Particle::Ask(2));

        // Note:
        // Pass 1 (Bids): Bid at 5 checks 4.
        // If 4 is empty, it moves to 4.
        // Pass 2 (Asks): Ask at 3 checks 4.
        // Now 4 contains the Bid! Collision!

        let events = grid.update();

        assert_eq!(events.len(), 1);
        let event = events[0];
        assert_eq!(event.buyer, 1);
        assert_eq!(event.seller, 2);
        // Price calculation: Height - 1 - Y.
        // Collision happened at y=4.
        // Price = 10 - 1 - 4 = 5.0.
        assert_eq!(event.price, 5.0);

        // Grid should show a Trade particle at (5, 4)
        match grid.get(5, 4) {
            // Age starts at DEFAULT_TRADE_AGE, but decays by 1 in the same tick (Pass 3)
            Particle::Trade { age } => assert_eq!(age, DEFAULT_TRADE_AGE - 1),
            _ => panic!(
                "Expected Trade particle at (5, 4), found {:?}",
                grid.get(5, 4)
            ),
        }
    }

    #[test]
    fn test_collision_bid_hits_ask() {
        // Test the case where Bid moves directly into Ask
        let mut grid = Grid::new(10, 10);
        // Bid at (5, 5). Moves to 4.
        grid.set(5, 5, Particle::Bid(1));
        // Ask at (5, 4).
        grid.set(5, 4, Particle::Ask(2));

        // Pass 1 (Bids): Bid at 5 checks 4.
        // 4 contains Ask. Collision!
        // Bid removed. Ask removed (replaced by Trade).

        let events = grid.update();
        assert_eq!(events.len(), 1);
        assert_eq!(events[0].price, 5.0); // 10 - 1 - 4
    }
}

#[cfg(test)]
mod sentry_tests {
    use super::*;

    #[test]
    fn test_sideways_movement_blocked_by_trade() {
        // Grid 3x3
        let mut grid = Grid::new(3, 3);
        // Trade at (1, 0) - Blocking Bid from moving UP
        grid.set(
            1,
            0,
            Particle::Trade {
                age: DEFAULT_TRADE_AGE,
            },
        );
        // Bid at (1, 1) - Blocked
        grid.set(1, 1, Particle::Bid(1));

        // Update
        grid.update();

        // Bid should move to (0, 1) or (2, 1) (Same Y, different X)
        // (1, 1) should be Empty
        assert_eq!(
            grid.get(1, 1),
            Particle::Empty,
            "Bid failed to move from (1,1)"
        );

        let in_left = matches!(grid.get(0, 1), Particle::Bid(1));
        let in_right = matches!(grid.get(2, 1), Particle::Bid(1));

        assert!(
            in_left || in_right,
            "Bid did not move sideways to (0,1) or (2,1)"
        );
    }

    #[test]
    fn test_ask_sideways_movement_blocked_by_trade() {
        // Grid 3x3
        let mut grid = Grid::new(3, 3);
        // Trade at (1, 2) - Blocking Ask from moving DOWN
        grid.set(
            1,
            2,
            Particle::Trade {
                age: DEFAULT_TRADE_AGE,
            },
        );
        // Ask at (1, 1) - Blocked
        grid.set(1, 1, Particle::Ask(2));

        // Update
        grid.update();

        // Ask should move to (0, 1) or (2, 1) (Same Y, different X)
        // (1, 1) should be Empty
        assert_eq!(
            grid.get(1, 1),
            Particle::Empty,
            "Ask failed to move from (1,1)"
        );

        let in_left = matches!(grid.get(0, 1), Particle::Ask(2));
        let in_right = matches!(grid.get(2, 1), Particle::Ask(2));

        assert!(
            in_left || in_right,
            "Ask did not move sideways to (0,1) or (2,1)"
        );
    }

    #[test]
    fn test_trapped_by_trades() {
        // Grid 3x3
        let mut grid = Grid::new(3, 3);
        // Surround (1, 1)
        grid.set(
            1,
            0,
            Particle::Trade {
                age: DEFAULT_TRADE_AGE,
            },
        ); // Top (Up blocked)
        grid.set(
            0,
            1,
            Particle::Trade {
                age: DEFAULT_TRADE_AGE,
            },
        ); // Left (Sideways blocked)
        grid.set(
            2,
            1,
            Particle::Trade {
                age: DEFAULT_TRADE_AGE,
            },
        ); // Right (Sideways blocked)

        // Bid at (1, 1)
        grid.set(1, 1, Particle::Bid(1));

        grid.update();

        // Should stay put
        assert_eq!(grid.get(1, 1), Particle::Bid(1), "Bid should be trapped");
    }

    #[test]
    fn test_trade_decay() {
        let mut grid = Grid::new(3, 3);
        // Trade with age 1
        grid.set(1, 1, Particle::Trade { age: 1 });

        grid.update();

        // Should decay to 0 then Empty (logic: if age > 0 { age - 1 } else { Empty })
        // Wait. Code says:
        // if age > 0 { self.cells[idx] = Particle::Trade { age: age - 1 }; }
        // else { self.cells[idx] = Particle::Empty; }

        // So age 1 becomes age 0. It is NOT Empty yet.
        match grid.get(1, 1) {
            Particle::Trade { age } => assert_eq!(age, 0),
            _ => panic!("Expected Trade with age 0"),
        }

        grid.update();
        // Now age 0 becomes Empty.
        assert_eq!(grid.get(1, 1), Particle::Empty);
    }

    #[test]
    fn test_bid_blocked_by_stationary_bid_moves_sideways() {
        // Grid 3x3
        let mut grid = Grid::new(3, 3);
        // Trade at (1, 0) - Blocking Bid A
        grid.set(
            1,
            0,
            Particle::Trade {
                age: DEFAULT_TRADE_AGE,
            },
        );
        // Bid A at (1, 1) - Blocked by Trade
        grid.set(1, 1, Particle::Bid(10));
        // Bid B at (1, 2) - Blocked by Bid A initially
        grid.set(1, 2, Particle::Bid(20));

        grid.update();

        // Expectation:
        // Bid A moves Sideways to (0,1) or (2,1).
        // Bid B moves UP to (1,1) (filling the void).

        // Verify Bid A moved sideways
        let a_left = matches!(grid.get(0, 1), Particle::Bid(10));
        let a_right = matches!(grid.get(2, 1), Particle::Bid(10));
        assert!(a_left || a_right, "Bid A did not move sideways");

        // Verify Bid B moved UP
        assert_eq!(
            grid.get(1, 1),
            Particle::Bid(20),
            "Bid B should move up to (1,1)"
        );

        // Verify original spot of Bid B is empty
        assert_eq!(grid.get(1, 2), Particle::Empty);
    }

    #[test]
    fn test_bid_blocked_by_wall_moves_sideways() {
        // Grid 3x3
        let mut grid = Grid::new(3, 3);
        // Wall at (1, 0) - Blocking Bid A
        grid.set(1, 0, Particle::Wall);
        // Bid A at (1, 1) - Blocked by Wall
        grid.set(1, 1, Particle::Bid(10));

        grid.update();

        // Expectation:
        // Bid A moves Sideways to (0,1) or (2,1).

        // Verify Bid A moved sideways
        let a_left = matches!(grid.get(0, 1), Particle::Bid(10));
        let a_right = matches!(grid.get(2, 1), Particle::Bid(10));
        assert!(a_left || a_right, "Bid A did not move sideways when blocked by Wall");
    }
}
