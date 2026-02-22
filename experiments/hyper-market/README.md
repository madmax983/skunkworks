# 🧬 Hyper Market

**Lineage:** `market-sim` (Logic) × `hyper-glass` (Visuals)

A 4D visualization of a Continuous Double Auction (CDA) market.

## Concept

Imagine a stock market where the Order Book is not a 2D list of numbers, but a physical 4D Hypercube.
-   **Y-Axis**: Price.
    -   **Bids (Green)** spawn at the bottom (Low Price) and bubble up.
    -   **Asks (Red)** spawn at the top (High Price) and fall down.
-   **X, Z, W Axes**: Parallel market streams, noise, or "Dark Pools". Particles can move sideways to avoid blockages or search for liquidity.
-   **Trades (Yellow Flash)**: Occur when a Bid collides with an Ask.

## The "Hyper" Twist

The physical constants of this universe are determined by your computer's real-time performance:
-   **Volatility (Temperature)**: Driven by **CPU Usage**. High load causes particles to jitter more in the X/Z/W dimensions, making the market more chaotic.
-   **Spacetime Distortion**: Driven by **RAM/Swap Usage**. The hypercube breathes and stretches based on memory pressure.

## Running

```bash
cargo run -p hyper-market
```

## Controls

-   **Arrow Keys / WASD**: Rotate Camera around the 4D projection.
-   **Volatility**: Increase your CPU load to see the market panic!
