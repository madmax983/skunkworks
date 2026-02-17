# Chimera Roots 🌿🧬

**Parents**: `experiments/algo-botany` + `experiments/chimera-lang`

An evolutionary simulation where plant roots are driven by genetic algorithms executing on a virtual machine (ChimeraVM).

## The Concept

Instead of using a fixed pathfinding algorithm (like A* in `algo-botany`), each root tip contains a **ChimeraVM** instance with a random genome.
The root "senses" its surroundings (Soil, Rock, Water) and executes its DNA to decide where to grow.

Through natural selection, roots that reach water or travel further survive and reproduce, passing on their code to the next generation.

## Lineage

- **From algo-botany**: The Grid system, rendering style, and the concept of roots seeking water.
- **From chimera-lang**: The `ChimeraVM` execution engine, `OpCode` instruction set, and genetic structure (DNA/Helix/Strand).
- **Novel Trait**: **Evolving Heuristics**. The pathfinding logic is not hardcoded but evolved. The roots *learn* how to navigate the soil.

## Controls

- **Left Click**: Place Rock (Obstacle).
- **Right Click**: Place Water (Goal).
- **Space**: Pause/Resume simulation.
- **N**: Force Next Generation.
- **E**: Toggle Auto-Evolution (automatically starts next gen when all roots die/finish).
- **Up/Down**: Increase/Decrease simulation speed.

## How it Works

1. **Initialization**: A population of 50 roots spawns at the seed location with random DNA (20 genes).
2. **Growth**:
   - Each tick, the root's VM receives the state of its 4 neighbors (Up, Right, Down, Left).
   - The VM executes for 10 cycles.
   - The VM outputs a direction (0-3) via the Stack.
   - The root moves if the target cell is valid (Soil/Water).
   - Hitting a Rock or Wall kills the root.
   - Reaching Water rewards the root.
3. **Evolution**:
   - Fitness is calculated based on distance to water (or reaching it).
   - The top 20% of roots are selected as parents.
   - Offspring are created via mutation (changing/inserting/deleting genes).
   - The cycle repeats.

## Emergent Behavior

Watch as the roots initially flail randomly (Brownian motion), but eventually evolve strategies to:
- Move consistently in one direction.
- Avoid obstacles.
- Homogenize towards the water.
