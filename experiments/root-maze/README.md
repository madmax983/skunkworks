# Root Maze 🌳

**"Biological Pathfinding"**

This experiment simulates a root system solving a maze to find water.
It combines **Root Growth Simulation** with **Pathfinding**.

## The Metaphor

- **Roots**: The agents. They grow, branch, and consume energy.
- **Nutrients/Energy**: Roots need energy to grow. They lose energy over time (metabolism) and gain it from soil (trace amounts) or water (jackpot).
- **Tropism**: Roots sense the gradient of "water smell" and gravity.
- **The Maze**: A harsh environment of Hard Rock (impassable), Soft Soil (diggable), and Water (goal).

## Controls

- **Space**: Pause/Resume growth.
- **R**: Reset the simulation (new maze).
- **G**: Toggle grid visibility (to see just the roots).
- **Click**: Plant a new seed at the mouse cursor.

## Algorithm

1. **Maze Generation**: Cellular Automata (Game of Life rules) creates organic cave-like structures.
2. **Growth**:
   - Each `RootTip` casts rays to sense the environment.
   - It prefers Water > Empty > Soft Soil > Hard Rock.
   - It has a bias downwards (Geotropism).
   - It moves towards the best direction with some random "wobble".
   - It leaves a trail of `RootSegment`s.
3. **Branching**: If energy is high, a tip may split into two.
4. **Death**: If a tip hits rock or runs out of energy, it dies.

## Running

```bash
cargo run -p root-maze
```
