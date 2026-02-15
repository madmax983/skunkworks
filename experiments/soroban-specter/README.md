# Soroban Specter 🧮👻

**Lineage:** `soroban-market` × `fluid-specter`

**Concept:** "Turbulent Accounting"

This experiment visualizes the physical impact of high-frequency trading calculations. A Japanese Abacus (Soroban) performs calculations for a simulated trading bot. The movement of the beads—representing arithmetic operations—injects energy and turbulence into a background fluid simulation.

## Features

- **Fluid Simulation**: A 2D Navier-Stokes solver (from `fluid-specter`) visualizes the "atmosphere" of the market.
- **Soroban Logic**: Real arithmetic performed on a simulated Soroban (from `soroban-market`).
- **Interaction**:
  - **Bead Movement**: Every time a bead moves (Heaven or Earth), it creates a "splash" in the fluid at that location.
  - **Velocity Injection**: The direction of the bead's movement determines the direction of the fluid impulse.
- **Market Simulation**: A random walk price model drives the calculation of a Simple Moving Average (SMA).

## Controls

- **Automatic**: The simulation runs automatically.
- **Mouse**: Click and drag to manually stir the fluid (interacting with the "Market Ether").

## Technical Details

- **Rendering**: `macroquad`
- **Logic**: `crates/soroban`
- **Physics**: Grid-based fluid solver (Stable Fluids)

## Lineage Documentation

- **Parent A (Logic)**: `soroban-market` provided the `Market`, `Trader`, and `Soroban` logic.
- **Parent B (Physics/Visuals)**: `fluid-specter` provided the `FluidSolver` and spectral rendering aesthetic.
- **Novel Trait**: The mapping of discrete arithmetic states (bead positions) to continuous fluid dynamics.
