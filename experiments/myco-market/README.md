# 🍄 myco-market

Pheromone-Guided Market Liquidity.

This experiment crosses the continuous double auction logic of `market-sim` with the slime mold pathfinding agents of `myco-transit`.

## 🧬 Lineage

- **Parent A**: `crates/market-sim` (Continuous Double Auction Particle System logic)
- **Parent B**: `experiments/myco-transit` (Pheromone pathfinding, `Agent` sensing, `World` grid diffusion)

## 🔬 Phenotype

Instead of placing rigid orders that only move up or down blindly, Bids and Asks act as foraging agents.
When trades occur (Bids and Asks collide), a massive deposit of "liquidity pheromones" is released.

Subsequent Bids and Asks use biological sensing (sampling left, forward, and right) to steer toward high-pheromone trails while moving toward their target price side (buyers go up, sellers go down).

This results in an emergent visualization of **liquidity pools** and **order flow routing**. The market participants self-organize into highly efficient trading highways, bridging the spread organically rather than via random walks.

## 🚀 Running

```bash
cargo run -p myco-market
```
