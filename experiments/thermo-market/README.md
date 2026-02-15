# Thermo-Market 📈🌡️

> "The Invisible Hand is Hot."

**Thermo-Market** is a hybrid experiment exploring the thermodynamics of financial markets. It combines a **Continuous Double Auction** market simulation with a **Thermodynamic Construction** simulation.

## The Concept

In this world, **Trading Activity generates Heat**.
- **Bids** (Green) and **Asks** (Red) move through the grid to find prices.
- When they collide, a **Trade** occurs (Yellow Flash).
- Each Trade releases **Heat** (Energy) into the environment.
- **Heat** creates **Volatility** (Random Jitter) in the market particles, making efficient trading harder.

To maintain market stability, **Termites** (Regulators / Infrastructure Bots) inhabit the grid:
- They sense Heat gradients (Pheromones).
- They build **Cooling Fins** (Walls) to absorb and dissipate heat.
- However, these **Walls** (White) also act as **Barriers** to order flow, potentially segmenting the market.

## Lineage 🧬

This experiment is a splice of:
- **Parent A**: `experiments/process-auction` (Market Logic, Bidding/Asking agents).
- **Parent B**: `experiments/thermo-termites` (Heat Diffusion, Termite Construction, WGPU/Macroquad rendering).
- **Mutation**: `market-sim` logic was mutated to include `Particle::Wall`, allowing physical structures to impede economic flow.

## Controls

- `Space`: Pause/Resume.
- `1`: View Heat Map + Market Overlay (Default).
- `2`: View Pheromone Map.
- `3`: View Market Only (High Contrast).
- `T`: Toggle Termite visibility.
- `A`: Toggle Air visibility (Flow visualization).
- `Esc` / `Q`: Quit.

## Legend

- **Green Dot**: Bid (Buyer).
- **Red Dot**: Ask (Seller).
- **Yellow Flash**: Trade Execution.
- **White/Grey Block**: Wall (Cooling Fin).
- **Blue Dot**: Termite (Empty).
- **White Dot**: Termite (Carrying Wall Material).
- **Background**: Heat Map (Blue = Cold, Red = Hot).

## Observations

- Watch how "Hot" zones (high trading volume) attract Termites.
- Observe how walls eventually cool the zone but might block new orders from reaching the liquidity pool.
- The system seeks a balance between Liquidity (Flow) and Stability (Temperature).
