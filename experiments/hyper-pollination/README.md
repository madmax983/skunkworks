# Hyper-Pollination

**Parents**: `experiments/hyper-garden` + `experiments/hyper-flock`

**Concept**: A 4D ecosystem where procedurally generated plants (Git History) are pollinated by flocking boids (System Processes?). The entire universe breathes and distorts based on real-time system metrics (CPU, Memory, Swap, Load).

## Features

- **4D L-System Plants**: Generated from Git Commit Hashes.
- **4D Boids**: Flocking behavior in 4D space.
- **Pollination**: Boids seek out plant tips and transfer color traits.
- **System Distortion**: The 4D space stretches and rotates based on your computer's load.

## Controls

- **Arrow Keys**: Rotate Camera (3D Projection)
- **W/S**: Zoom In/Out
- **Space**: (Future) Reset Simulation

## How to Run

```bash
cargo run -p hyper-pollination
```

## Lineage

- **Math**: Combined `Vec4` logic from `hyper-garden` and `hyper-flock`.
- **Plants**: Adapted L-System generator from `hyper-garden`.
- **Boids**: Adapted Flocking logic from `hyper-flock` with added `seek` and `pollinate` behaviors.
- **Monitor**: Reused `SystemMonitor` logic.
