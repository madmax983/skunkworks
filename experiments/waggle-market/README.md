# Waggle Market 🐝📈

**Genetic Cross**: `waggle-dance` × `market-sim` (via `market-sim` crate)

A simulation where the "Invisible Hand" is actually a swarm of bees dancing to signal price arbitrage opportunities.

## The Concept

- **The Field (Left)**: Contains "Sources" of supply (cheap goods) and demand (expensive buyers).
- **The Hive (Right)**: A Continuous Double Auction (CDA) market.
- **The Bees (Traders)**:
    1.  **Scout**: Wander the field to find Sources.
    2.  **Return**: Bring the information back to the Hive.
    3.  **Dance**: Waggle to signal the opportunity.
        -   **Dance Intensity**: Proportional to the profit margin (Source Value).
        -   **Dance Effect**: Places **Orders** in the Hive's Order Book.
            -   Found Supply -> Place **Asks** (Sell Orders).
            -   Found Demand -> Place **Bids** (Buy Orders).
    4.  **Recruit**: Other bees watching the dance get excited and fly to the same source, creating a "Trend" or "Flash Mob".

## Controls

-   **Left Click**: Add a new Source (Supply or Demand) at the mouse position.
-   **R**: Reset the simulation.

## Emergent Behavior

-   **Momentum Trading**: A single scout finding a high-value source can trigger a massive wave of recruitment, causing a sudden influx of orders and a price shift.
-   **Market Efficiency**: As bees exploit a source, the market price adjusts to reflect the external reality. (Note: In this version, external sources are infinite, so the market just saturates).

## Lineage

-   Inherits **Swarm Intelligence** from `waggle-dance`.
-   Inherits **Market Physics** from `market-sim`.
-   **Mutation**: Information propagation via biological signaling driving financial liquidity.
