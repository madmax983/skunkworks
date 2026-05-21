# Market Rogue 📉⚔️

**Genetic Cross**: `market-flow` × `git-rogue`

A roguelike where the "dungeon" is a volatile market order book generated from your git history.

## Lineage
- **Allele A (market-flow)**: Grid-based fluid simulation of market orders (Bids/Asks) and price dynamics.
- **Allele B (git-rogue)**: Git repository crawling to generate procedural levels from commit history.
- **Emergent Trait**: Each commit becomes a playable level. The commit hash determines the "Market Price" (terrain layout), and the commit message length determines "Volatility" (spawn rate of hazards).

## Concept
You are a Trader trying to navigate the chaotic floor of the Exchange.
- **Green Blocks (Bids)**: Upward pressure. Hitting them pushes you UP.
- **Red Blocks (Asks)**: Downward pressure. Hitting them pushes you DOWN.
- **Goal**: Reach the **EXIT** on the right side of the screen to advance to the next commit (Level).
- **Game Over**: If you are pushed off the top or bottom of the screen, you are **LIQUIDATED**.

## Controls
- **Arrow Keys**: Move against the market forces.
- **R**: Restart current level.
- **Q**: Quit.

## Build & Run
```bash
cargo run -p market-rogue
```
Must be run inside a git repository to generate levels.
