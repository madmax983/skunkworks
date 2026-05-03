# 🧬 market-platter

**Market Heatmap Scan**

This experiment is a hybrid cross between:
- **`market-sim`**: The parent crate providing the Continuous Double Auction (CDA) physics particle system.
- **`platter`**: The parent crate providing a continuous 2D scalar field representation and decay logic.

## Novel Trait & Lineage

This experiment demonstrates a **Market Heatmap Scan**.

The discrete market trades (from `market-sim`) are embedded into a 2D scalar field (`platter`). As trades happen in the market grid, they act as massive heat pulses that saturate the continuous scalar field at their exact collision point (x, y coordinate).

The field naturally decays over time. The result is a fading visual heatmap of market activity.

## Predicted Phenotype

A visual "trade radar" or market heatmap. Discrete market interactions (Bids and Asks colliding) leave continuous, fading trails in the 2D grid. Areas of high liquidity will show up as bright, persistent hotspots, while sparse price levels will remain dark.

## Usage

```bash
cargo run -p market-platter
```
