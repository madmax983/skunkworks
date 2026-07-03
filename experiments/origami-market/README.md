# origami-market

**Parents**: `crates/market-sim` + `crates/origami`
**Concept**: Topographical Market Depth.

## Traits
* **Lineage from `market-sim`**: Continuous Double Auction simulation generating Bids, Asks, and Trades.
* **Lineage from `origami`**: Procedural 3D soft-body Miura-ori mesh using constraints.
* **Phenotype**: This hybrid visually maps market liquidity and trading pressure onto a continuous soft-body mesh. As trades occur, they dynamically actuate physical constraints within the paper, causing the market surface to fold, crease, and crumple based on volatility.

## Quick Start
```bash
cargo run -p origami-market --headless
```
