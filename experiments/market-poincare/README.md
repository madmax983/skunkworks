# market-poincare

**Parents**: `crates/market-sim` + `crates/poincare-disk`
**Concept**: Hyperbolic Market Order Book

## Traits
* **Lineage from `market-sim`**: Discrete grid-based order book simulating the collision of Bids and Asks to generate Trades.
* **Lineage from `poincare-disk`**: Non-Euclidean hyperbolic space where distance expands exponentially towards the boundary.
* **Phenotype**: The traditional Euclidean order book grid is projected into the interior of the Poincaré disk. Bids and Asks flow through hyperbolic space via Möbius transformations. When they collide and execute trades, they create hyperbolic "flashes" (✸) that spin towards the boundary, visualizing market volatility trapped inside a mathematical black hole.
