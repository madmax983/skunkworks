# Flood Strategy 🌊🛡️

**A Moonshot Experiment by Genesis**

> "Run for the hills, the tide is rising!"

This experiment combines **Shallow Water Equations** (Fluid Dynamics) with **Real-Time Strategy** (Pathfinding/Survival).

## Concept
A strategy game where the primary antagonist is **Physics**. You control the terrain to protect autonomous units from a rising tide.

- **Physics**: A "Virtual Pipes" implementation of Shallow Water Equations (SWE) simulates realistic water flow, pressure, and velocity over a heightmap.
- **AI**: Autonomous units (Red Dots) try to reach the goal (Green Zone) using local greedy pathfinding, avoiding water that is too deep.
- **Interaction**: You play as a God/Terraformer. You can raise mountains to block the water or dig canals to divert it.

## Controls

| Input | Action |
|-------|--------|
| **Left Click** | Raise Terrain (Build Dam) |
| **Right Click** | Lower Terrain (Dig Canal) |
| **Middle Click** | Spawn Water Source |
| **Space** | Pause/Resume Simulation |
| **T** | Toggle Dynamic Tides |
| **R** | Reset Simulation |

## Tech Stack
- **Engine**: `macroquad` (Rust Game Framework)
- **Physics**: Custom SWE Solver (Pipe Method)
- **AI**: Gradient Descent Pathfinding

## How it works
The fluid simulation runs at a higher frequency than the frame rate for stability.
Units update less frequently to simulate "reaction time".
The tide is a sine wave applied to the boundary conditions.
