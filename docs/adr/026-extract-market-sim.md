# 026. Abstract Market Simulation Logic

## Status
Accepted

## Context
The `chimera-sovereignty` experiment (ADR 018) required an economic system to facilitate trade and resource exchange between organisms. However, embedding complex trading logic directly into the organism's loop or the game's main update function would create tight coupling and limit the ability to experiment with different market mechanisms (e.g., auctions vs. direct trade).

## Decision
We will extract the core market simulation logic into a dedicated crate: `crates/market-sim`.

### Key Components:
1.  **Continuous Double Auction (CDA):** The market operates as a 2D grid where price is represented by the Y-axis.
2.  **Particle System:** Orders are represented as particles (`Bid` moving up, `Ask` moving down).
3.  **Collision-Based Matching:** Trades occur when a `Bid` particle collides with an `Ask` particle, creating a `TradeEvent`.
4.  **Stochastic Movement:** Particles include random lateral movement to simulate market noise and search behavior.

## Consequences
### Positive
*   **Decoupling:** The economic simulation runs independently of the biological simulation, communicating only via buy/sell orders and trade events.
*   **Visualizability:** The particle-based approach is inherently visual, allowing for intuitive TUI representations of market depth and activity.
*   **Flexibility:** The same market engine can be reused for other economic experiments or different "commodities" within the same world.

### Negative
*   **Abstraction Cost:** Mapping biological needs (energy, space) to abstract market orders adds a layer of complexity for the organism logic.
*   **Performance:** Simulating thousands of particles every tick is more computationally expensive than a simple order book matching algorithm.
