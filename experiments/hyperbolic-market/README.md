# Hyperbolic Market 📈🌌

> "Where infinite liquidity meets infinite volatility."

A continuous double auction market simulation running on the **Poincaré Disk** model of hyperbolic geometry.

![Hyperbolic Market](https://i.imgur.com/placeholder.png)

## Concept

In standard financial models, price movements are often assumed to be linear or log-linear. In **Hyperbolic Market**, we explore the idea of **Financial Relativity**:

*   **The Center ($r=0$)**: Represents the "Equilibrium Price" or "Fair Value". Here, space is Euclidean-like. Traders move predictably.
*   **The Boundary ($r=1$)**: Represents "Extreme Prices" (High/Low). Here, space expands exponentially.
    *   **Geodesic Trading**: Traders (Bids and Asks) do not move in straight lines. They follow **geodesics** (paths of shortest distance in curved space).
    *   **Volatility as Curvature**: As traders approach extreme prices (the boundary), their paths curve dramatically due to the hyperbolic metric. A small visual step corresponds to a massive "financial" distance.

## Simulation Rules

*   **Bids (Green)**: Spawn at the "Bottom" (Low Price) and move "Up" along geodesics towards the Top.
*   **Asks (Red)**: Spawn at the "Top" (High Price) and move "Down" along geodesics towards the Bottom.
*   **Trades (Flash)**: Occur when a Bid and an Ask collide visually (Euclidean distance on the disk).
*   **Price**: Determined by the Y-coordinate of the collision on the disk.

## Lineage 🧬

*   **Parent A**: `experiments/market-flow` (The Order Book mechanics, Bid/Ask duality).
*   **Parent B**: `experiments/hyperbolic-ants` (The Poincaré Disk visualization and geometry).
*   **Novel Trait**: **Relativistic Arbitrage**. The market depth is infinite near the boundary, yet visually finite.

## Controls

*   `Q` / `Esc`: Quit the simulation.

## Implementation Details

*   **Engine**: `ratatui` for TUI rendering.
*   **Geometry**: `poincare-disk` crate for Möbius transformations and hyperbolic distance calculations.
*   **Physics**: Movement uses `mobius_add` to apply local translations in the frame of reference of the trader, naturally generating geodesic trajectories.
