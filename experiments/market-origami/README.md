# 🍄 market-origami

Market Liquidity Morphogenesis.

This experiment crosses the continuous double auction logic of `market-sim` with the procedural soft-body meshes of `origami`.

## 🧬 Lineage

- **Parent A**: `crates/market-sim` (Continuous Double Auction Particle System logic)
- **Parent B**: `crates/origami` (Procedural Miura-ori mesh generation)

## 🔬 Phenotype

Instead of viewing market order flow purely abstractly, the physical presence of Bids and Asks acts as a structural force on a continuous Miura-ori paper mesh.

As trades occur (Bids and Asks collide) and liquidity pools form, the intense market activity physically elongates the constraints of the paper in that region. Where the market is illiquid or quiet, the paper contracts.

This creates an emergent, living sheet of paper that folds, crumples, and breathes dynamically based on the algorithmic trading volume occurring across its surface.

## 🚀 Running

```bash
cargo run -p market-origami
```
