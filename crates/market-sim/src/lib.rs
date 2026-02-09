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

/// A fundamental unit of the market simulation.
#[derive(Clone, Copy, PartialEq, Debug)]
pub enum Particle {
    /// Empty space. No order exists here.
    Empty,

    /// A Buyer's order (Bid).
    ///
    /// Moves **UP** (towards lower y-indices, representing higher prices).
    /// Contains the `ID` of the buyer.
    Bid(usize),

    /// A Seller's order (Ask).
    ///
    /// Moves **DOWN** (towards higher y-indices, representing lower prices).
    /// Contains the `ID` of the seller.
    Ask(usize),

    /// The remnant of a successful transaction.
    ///
    /// Created when a `Bid` and `Ask` collide. It decays over time,
    /// serving as a visual indicator of market activity.
    Trade {
        /// How many frames this particle will persist before disappearing.
        age: u8
    },
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
    pub fn get(&self, x: usize, y: usize) -> Particle {
        if x >= self.width || y >= self.height {
            return Particle::Empty;
        }
        self.cells[y * self.width + x]
    }

    /// Sets the particle at the specified coordinates.
    ///
    /// Does nothing if coordinates are out of bounds.
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
        let mut trades = 0;
        let mut bids = 0;
        let mut asks = 0;
        let mut weighted_y_sum = 0.0;
        let mut mass_sum = 0.0;

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

        // Pass 1: Bids (Up)
        // Iterate Top to Bottom so we don't move the same particle twice in one pass?
        // Actually for UP movement, we should iterate Top to Bottom (0..height).
        // If we process row 0, then row 1...
        // Row 1 moves to Row 0. Now it's in Row 0. Next loop iteration (Row 2) moves...
        // Wait. If we iterate 0..height:
        // y=0: Check.
        // y=1: Move to 0. Marked updated.
        // y=2: Move to 1. Marked updated.
        // This is correct. If we iterated Bottom to Top (height..0):
        // y=9: Move to 8.
        // y=8: (Now contains particle from 9). Move to 7.
        // One particle would teleport to the top in a single frame!
        // So: Iterating 0..height prevents teleportation for Upward movement.
        for y in 0..self.height {
            for &x in &self.scan_x {
                let idx = y * self.width + x;
                if self.updated[idx] {
                    continue;
                }

                if let Particle::Bid(owner) = self.cells[idx] {
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
                                self.cells[target_idx] = Particle::Trade { age: 5 };
                                self.cells[idx] = Particle::Empty;
                                self.updated[target_idx] = true;
                                trades += 1;
                                trade_events.push(TradeEvent {
                                    buyer: owner,
                                    seller,
                                    price: (self.height - 1 - (y - 1)) as f32, // Invert Y for price
                                });
                            }
                            _ => {
                                // Sideways logic: Try to step around blockage
                                let dxs = if rng.gen_bool(0.5) { [-1, 1] } else { [1, -1] };
                                for dx in dxs {
                                    let nx = x as isize + dx;
                                    if nx >= 0 && nx < self.width as isize {
                                        let nx = nx as usize;
                                        let n_idx = y * self.width + nx;
                                        if !self.updated[n_idx]
                                            && matches!(self.cells[n_idx], Particle::Empty)
                                        {
                                            self.cells[n_idx] = Particle::Bid(owner);
                                            self.cells[idx] = Particle::Empty;
                                            self.updated[n_idx] = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // Reached top (Maximum Price), expire.
                        self.cells[idx] = Particle::Empty;
                    }
                }
            }
        }

        // Pass 2: Asks (Down)
        // For DOWN movement, we must iterate Bottom to Top (height..0) to prevent teleportation.
        // y=9: Check.
        // y=8: Move to 9.
        // y=7: Move to 8.
        // If we iterated 0..height:
        // y=0: Move to 1.
        // y=1: (Now has particle). Move to 2.
        // Teleportation!
        for y in (0..self.height).rev() {
            for &x in &self.scan_x {
                let idx = y * self.width + x;
                if self.updated[idx] {
                    continue;
                }

                if let Particle::Ask(owner) = self.cells[idx] {
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
                                self.cells[target_idx] = Particle::Trade { age: 5 };
                                self.cells[idx] = Particle::Empty;
                                self.updated[target_idx] = true;
                                trades += 1;
                                trade_events.push(TradeEvent {
                                    buyer,
                                    seller: owner,
                                    price: (self.height - 1 - (y + 1)) as f32,
                                });
                            }
                            _ => {
                                // Sideways logic
                                let dxs = if rng.gen_bool(0.5) { [-1, 1] } else { [1, -1] };
                                for dx in dxs {
                                    let nx = x as isize + dx;
                                    if nx >= 0 && nx < self.width as isize {
                                        let nx = nx as usize;
                                        let n_idx = y * self.width + nx;
                                        if !self.updated[n_idx]
                                            && matches!(self.cells[n_idx], Particle::Empty)
                                        {
                                            self.cells[n_idx] = Particle::Ask(owner);
                                            self.cells[idx] = Particle::Empty;
                                            self.updated[n_idx] = true;
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // Reached bottom (Minimum Price), expire.
                        self.cells[idx] = Particle::Empty;
                    }
                }
            }
        }

        // Pass 3: Stats & Cleanup
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
                        weighted_y_sum += (self.height - y) as f32; // Height - y gives "height from bottom"
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

        self.trade_count = trades;
        self.total_bids = bids;
        self.total_asks = asks;
        if mass_sum > 0.0 {
            self.center_of_mass = weighted_y_sum / mass_sum;
        }

        trade_events
    }
}
