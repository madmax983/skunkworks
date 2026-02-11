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
    /// Uses a generation-based check to avoid O(N) clearing every frame.
    updated: Vec<u8>,
    /// The current generation of updates. Increments every frame.
    /// When it wraps to 0, the `updated` vector is cleared.
    updated_gen: u8,
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
            updated: vec![0; width * height],
            updated_gen: 0,
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

        // Stats accumulators
        let mut bids = 0;
        let mut asks = 0;
        let mut weighted_y_sum = 0.0;
        let mut mass_sum = 0.0;
        let mut trades_created_count = 0;

        // Generation-based update tracking
        self.updated_gen = self.updated_gen.wrapping_add(1);
        if self.updated_gen == 0 {
            self.updated.fill(0);
            self.updated_gen = 1;
        }
        let current_gen = self.updated_gen;

        // Determine scan direction (randomized per frame)
        let forward_scan = rng.gen_bool(0.5);

        // Pass 1: Bids (Up) & Trade Decay
        for y in 0..self.height {
            for i in 0..self.width {
                let x = if forward_scan { i } else { self.width - 1 - i };
                let idx = y * self.width + x;

                if self.updated[idx] == current_gen {
                    continue;
                }

                match self.cells[idx] {
                    Particle::Bid(owner) => {
                        // Movement Logic
                        let mut final_y = Some(y); // Where the bid ends up (for stats)

                        if y > 0 {
                            let target_idx = (y - 1) * self.width + x;
                            match self.cells[target_idx] {
                                Particle::Empty => {
                                    self.cells[target_idx] = Particle::Bid(owner);
                                    self.cells[idx] = Particle::Empty;
                                    self.updated[target_idx] = current_gen;
                                    final_y = Some(y - 1);
                                }
                                Particle::Ask(seller) => {
                                    // Collision!
                                    self.cells[target_idx] = Particle::Trade { age: 5 };
                                    self.cells[idx] = Particle::Empty;
                                    self.updated[target_idx] = current_gen;
                                    trades_created_count += 1;
                                    trade_events.push(TradeEvent {
                                        buyer: owner,
                                        seller,
                                        price: (self.height - 1 - (y - 1)) as f32, // Invert Y for price
                                    });
                                    final_y = None; // Destroyed
                                }
                                _ => {
                                    // Sideways logic: Try to step around blockage
                                    let dxs = if rng.gen_bool(0.5) { [-1, 1] } else { [1, -1] };
                                    for dx in dxs {
                                        let nx = x as isize + dx;
                                        if nx >= 0 && nx < self.width as isize {
                                            let nx = nx as usize;
                                            let n_idx = y * self.width + nx;
                                            // Check updated generation and empty
                                            if self.updated[n_idx] != current_gen
                                                && matches!(self.cells[n_idx], Particle::Empty)
                                            {
                                                self.cells[n_idx] = Particle::Bid(owner);
                                                self.cells[idx] = Particle::Empty;
                                                self.updated[n_idx] = current_gen;
                                                final_y = Some(y); // Same y, different x
                                                break;
                                            }
                                        }
                                    }
                                }
                            }
                        } else {
                            // Reached top (Maximum Price), expire.
                            self.cells[idx] = Particle::Empty;
                            final_y = None;
                        }

                        // Stats Update
                        if let Some(pos_y) = final_y {
                            bids += 1;
                            weighted_y_sum += (self.height - pos_y) as f32;
                            mass_sum += 1.0;
                        }
                    }
                    Particle::Trade { age } => {
                        // Decay Logic
                        if age > 1 {
                            self.cells[idx] = Particle::Trade { age: age - 1 };
                        } else {
                            self.cells[idx] = Particle::Empty;
                        }
                    }
                    _ => {}
                }
            }
        }

        // Pass 2: Asks (Down)
        for y in (0..self.height).rev() {
            for i in 0..self.width {
                let x = if forward_scan { i } else { self.width - 1 - i };
                let idx = y * self.width + x;

                if self.updated[idx] == current_gen {
                    continue;
                }

                if let Particle::Ask(owner) = self.cells[idx] {
                    let mut final_y = Some(y);

                    if y < self.height - 1 {
                        let target_idx = (y + 1) * self.width + x;
                        match self.cells[target_idx] {
                            Particle::Empty => {
                                self.cells[target_idx] = Particle::Ask(owner);
                                self.cells[idx] = Particle::Empty;
                                self.updated[target_idx] = current_gen;
                                final_y = Some(y + 1);
                            }
                            Particle::Bid(buyer) => {
                                // Collision!
                                self.cells[target_idx] = Particle::Trade { age: 5 };
                                self.cells[idx] = Particle::Empty;
                                self.updated[target_idx] = current_gen;
                                trades_created_count += 1;
                                trade_events.push(TradeEvent {
                                    buyer,
                                    seller: owner,
                                    price: (self.height - 1 - (y + 1)) as f32,
                                });
                                final_y = None;

                                    // Fix: The Bid at target_idx was counted in Pass 1.
                                    // Since we just destroyed it, we must decrement the stats.
                                    bids -= 1;
                                    weighted_y_sum -= (self.height - (y + 1)) as f32;
                                    mass_sum -= 1.0;
                            }
                            _ => {
                                // Sideways logic
                                let dxs = if rng.gen_bool(0.5) { [-1, 1] } else { [1, -1] };
                                for dx in dxs {
                                    let nx = x as isize + dx;
                                    if nx >= 0 && nx < self.width as isize {
                                        let nx = nx as usize;
                                        let n_idx = y * self.width + nx;
                                        if self.updated[n_idx] != current_gen
                                            && matches!(self.cells[n_idx], Particle::Empty)
                                        {
                                            self.cells[n_idx] = Particle::Ask(owner);
                                            self.cells[idx] = Particle::Empty;
                                            self.updated[n_idx] = current_gen;
                                            final_y = Some(y);
                                            break;
                                        }
                                    }
                                }
                            }
                        }
                    } else {
                        // Reached bottom (Minimum Price), expire.
                        self.cells[idx] = Particle::Empty;
                        final_y = None;
                    }

                    // Stats Update
                    if let Some(pos_y) = final_y {
                        asks += 1;
                        weighted_y_sum += (self.height - pos_y) as f32;
                        mass_sum += 1.0;
                    }
                }
            }
        }

        self.trade_count = trades_created_count;
        self.total_bids = bids;
        self.total_asks = asks;
        if mass_sum > 0.0 {
            self.center_of_mass = weighted_y_sum / mass_sum;
        }

        trade_events
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
            // Age starts at 5. Since Pass 3 (decay) was merged into Pass 1/2,
            // and the trade is created *after* the cell is processed (or skipped),
            // it doesn't decay in the creation frame.
            Particle::Trade { age } => assert_eq!(age, 5),
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
