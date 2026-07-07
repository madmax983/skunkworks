# Arthropod Market 🐜📈

**Lineage:** `crates/arthropod` × `crates/market-sim`

"Interacting with the invisible hand."

## Concept
This experiment brings the interactive UI elements of `arthropod` to the continuous double auction market simulation of `market-sim`.

## Novel Trait
**Interactive Market Making:** The immediate mode UI allows users to dynamically inject liquidity into the market. By clicking the buttons, the user manually spawns new `Bid` and `Ask` particles into the simulation, acting as an active participant or market maker in the continuous double auction, directly driving the physical market depth.

## Tech Stack
- `arthropod` for the interactive GUI buttons.
- `market-sim` for the continuous double auction particle physics.
- `macroquad` for real-time rendering.
