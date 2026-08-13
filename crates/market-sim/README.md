# Market Simulator: The Physics of Finance 📈

This crate implements a **Continuous Double Auction (CDA)** mechanism using a particle system.

Instead of a traditional order book (a list of numbers), the market is simulated as a physical 2D grid where:

*   **Price** is represented by the Y-axis (Height).
    *   Top of the grid ($y=0$) = **High Price**.
    *   Bottom of the grid ($y=H-1$) = **Low Price**.
*   **Bids (Buyers)** are particles that spawn at the bottom (low price) and "bubble up" towards higher prices.
    *   This represents a buyer gradually increasing their offer to find a seller.
*   **Asks (Sellers)** are particles that spawn at the top (high price) and "fall down" towards lower prices.
    *   This represents a seller gradually lowering their asking price to find a buyer.

## The Interaction

When a **Bid** (moving up) collides with an **Ask** (moving down), a transaction occurs!

1.  The two particles annihilate each other.
2.  A `Trade` particle is created at the collision point (representing the execution price).
3.  The `Trade` particle decays over time (visualized as a flash).
4.  A `TradeEvent` is emitted.

## Installation

Add this to your `Cargo.toml`:

```toml
[dependencies]
market-sim = "0.1.0"
```

## Example

```rust
fn main() {
use market_sim::{Grid, Particle};

// Create a 10x100 market grid
let mut market = Grid::new(10, 100);

// Place a Buyer (Bid) at the bottom (Low Price)
market.set(5, 99, Particle::Bid(1)); // Buyer ID 1

// Place a Seller (Ask) at the top (High Price)
market.set(5, 0, Particle::Ask(2)); // Seller ID 2

// Run the simulation loop
loop {
    let trades = market.update();

    // Eventually, they will meet in the middle!
    if !trades.is_empty() {
        let t = &trades[0];
        println!("Trade executed! Buyer {} bought from Seller {} at price ${}",
            t.buyer, t.seller, t.price);
        break;
    }
}
}
```
