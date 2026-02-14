# Market Fold 📈🦢

A hybrid experiment combining **Rigid Origami Kinematics** with **Market Simulation Physics**.

## Concept

This experiment visualizes a financial market as a **Miura-ori** folded surface. The structural integrity and geometry of the mesh are driven by market liquidity and volatility.

*   **Expansion (Flatness)**: Driven by trade volume. High liquidity (many trades) creates a stable, flat surface (Expansion ~ 1.0).
*   **Contraction (Folding)**: Driven by inactivity or volatility. Low liquidity causes the market to crumple and fold in on itself (Expansion < 1.0).
*   **Surface Color**:
    *   **Green**: Active Bids (Buyers).
    *   **Red**: Active Asks (Sellers).
    *   **White Flash**: Trade execution.

## Lineage 🧬

*   **Parent A**: `experiments/rigid-origami`
    *   Provided the `MiuraGrid` kinematics and `macroquad` rendering logic.
*   **Parent B**: `experiments/heap-market` (via `crates/market-sim`)
    *   Provided the Continuous Double Auction (CDA) simulation logic.

## Controls

*   **Mouse Drag**: Rotate Camera.
*   **Mouse Wheel**: Zoom.
*   **Observation**: Watch the market fold and unfold as buyers and sellers interact.

## Novel Trait

**"Financial Folding"**: The geometry of the world is determined by the economy. A crash is a physical collapse of the space.
