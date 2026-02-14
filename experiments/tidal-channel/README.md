# Tidal Channel ⚛️🌊

**Moonshot:** Tidal Forces + Pull-based Reactive Systems

A visualizer for the concept of "Task Breakdown via Tidal Forces".
As large "Task Bodies" approach the "Execution Singularity", the immense gravity gradient (tidal force) tears them apart into smaller sub-tasks, which are then consumed.

## 🕹️ Controls

- **Left Click**: Spawn Task Bodies (Planets).
- **Right Click (Hold)**: Move the Singularity.
- **Scroll**: Adjust Singularity Mass.
- **Up/Down Arrows**: Adjust Fracture Threshold (Roche Limit).

## 🔭 The Science

- **Gravity**: Newtonian $F = G \frac{M m}{r^2}$
- **Tidal Force**: $F_{tidal} \approx 2 G M R / d^3$
- **Fracture**: Occurs when $F_{tidal} > \text{Threshold}$.
    - Splits body into 2 smaller bodies (Mass/2, Radius/$\sqrt{2}$).
    - Conserves Mass and Momentum.
    - Adds slight lateral separation velocity.

## 🛠️ Stack

- `macroquad`: 2D rendering and game loop.
- `rand`: Procedural generation.
