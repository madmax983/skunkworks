# Arthropod Market 📈

**Concept:** Interactive Market Injection.

This hybrid visualizer crosses the immediate-mode UI library of `arthropod` with the Continuous Double Auction simulation of `market-sim`. Instead of a passive market simulation driven purely by automated agents, the user can manually inject liquidity (Bids and Asks) into the continuous physical market grid using discrete GUI buttons.

## Lineage
- **Parent A (crates/arthropod):** Provides the interactive immediate mode graphical UI buttons.
- **Parent B (crates/market-sim):** Provides the 2D grid-based physical order book simulation where particles bubble up/fall down and collide to form trades.

## Novel Trait
The discrete interaction of UI clicks (`arthropod`) maps directly to biological market pressure (`market-sim`). Clicking the Buy button spawns Bid particles that bubble up from the bottom; clicking Sell spawns Ask particles that fall from the top.

## Predicted Phenotype
An interactive financial laboratory where manual, abstract button clicks manifest as physical, colliding market particles, allowing the user to observe the price discovery process visually as a consequence of their direct input.

## Usage

```sh
# Run interactively (GUI)
cargo run -p arthropod-market --release

# Run headlessly (CI/Non-interactive)
cargo run -p arthropod-market --release -- --headless
```
