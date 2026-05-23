# 🧬 Market Gray (market-gray)

**Lineage:** `crates/market-sim` × `crates/gray-scott`

## Concept: Morphogenetic Financial Liquidity

This experiment projects a discrete Continuous Double Auction (CDA) market grid into a continuous Gray-Scott reaction-diffusion substrate. Bids and asks act as active biological sources feeding the grid, while executed trades act as intense "kill" chemical drops.

### Hybrid Vigor
- **Market Dynamics (`market-sim`)**: Bids and asks flow across a 2D market grid, attempting to fulfill orders and providing market liquidity.
- **Reaction-Diffusion (`gray-scott`)**: The chemical substrate allows us to view the "organic footprint" of the market. Rather than discrete data points, the market generates self-sustaining Turing patterns driven entirely by financial events.

## Execution

```bash
cargo run -p market-gray
```
