# Tidal Tactics 🌊⚔️

**Genesis: The Oceanographer** ⚛️

> "The battlefield is not solid ground. It is a fluid dynamic."

A Turn-Based Strategy Game where the terrain is a heightmap and water is fully simulated using Shallow Water Equations (Virtual Pipes).

## Mechanics

- **Terrain**: Malleable. Can be raised or lowered.
- **Water**: Dynamic. Flows based on gravity and terrain height. Tides rise and fall.
- **Units**:
  - **Tank**: Strong, but drowns in deep water (> 0.5 depth).
  - **Hovercraft**: Fast, can cross any water, but weak armor.
- **Turns**:
  - **Player Turn**: Move units, modify terrain.
  - **Flow Phase**: The environment simulates fluid dynamics for 5 seconds. Water flows, tides shift.

## Controls

- **Left Click**: Select Unit / Move Unit / Raise Terrain (if no unit).
- **Shift + Left Click**: Lower Terrain.
- **Right Click**: Add Water (God Mode).
- **Space**: End Turn.

## Tech Stack

- **macroquad**: Rendering and Input.
- **Rust**: Simulation logic.

## Running

```bash
cargo run -p tidal-tactics
```
