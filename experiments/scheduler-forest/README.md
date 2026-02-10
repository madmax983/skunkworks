# Scheduler Forest 🌳⚙️

**"The Scheduler Forest"** - An ecosystem where process scheduling algorithms determine the growth of fractal trees.

## Concept
A visualization of Operating System scheduling algorithms using a forest metaphor.
- **Trees**: Processes competing for CPU cycles.
- **Sunlight**: CPU time slices.
- **Growth**: Execution progress.
- **Scheduler**: The algorithm distributing sunlight.

## Controls
- **Space**: Pause/Resume simulation.
- **Tab**: Cycle through scheduling algorithms:
  - `Round Robin`: Fair, cyclic distribution.
  - `Random`: Stochastic growth.
  - `Priority (Fairness)`: Prioritizes the most "starved" trees (least relative growth).
- **Left/Right Arrows**: Adjust simulation speed.
- **R**: Reset growth.
- **Right Mouse Drag**: Pan camera.
- **Mouse Wheel**: Zoom.

## Technical Details
- Built with `macroquad`.
- Implements L-System grammar expansion for tree structures.
- Simulates resource-constrained growth where trees only expand when "scheduled".
- Demonstrates how different scheduling policies affect the "shape" of the forest's evolution over time.

## Algorithms
1. **Round Robin**: Trees grow in a predictable, uniform wave.
2. **Random**: Growth is chaotic and uneven.
3. **Priority**: Trees that are behind in growth are prioritized, leading to a "catch-up" effect where the canopy stays relatively level.
