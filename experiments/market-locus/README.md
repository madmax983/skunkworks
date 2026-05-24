# market-locus
**Parents**: `market-sim` + `locus`

Topological Market Liquidity. An order book where trades wrap around topological bounds.

**Lineage Plan**:
- From market-sim: Discrete financial order book grid, Bid/Ask particle physics, and execution logic.
- From locus: Topological spaces (Torus, Klein Bottle, Sphere, Projective Plane) and Vec2 coordinates.
- Novel trait: Continuous loop visualization where localized market price spikes and volatility wrap around to instantly affect opposite financial boundaries.
