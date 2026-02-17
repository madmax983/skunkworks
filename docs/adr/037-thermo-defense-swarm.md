# 037. Thermo-Defense: Swarm Intelligence & Heat Diffusion

## Status
Accepted

## Context
We are conducting a "Moonshot" experiment (`experiments/thermo-defense`) to study emergent defense strategies in a simulated data center environment. The goal is to observe how a swarm of simple agents (Termites) can construct cooling structures and fortifications against an adaptive enemy (Locusts) in real-time.

Key constraints:
*   **Scale:** The simulation requires thousands of agents to observe emergent swarm behavior (Stigmergy).
*   **Environment:** The world is a dynamic grid with continuous variables (Heat, Pheromones) that diffuse over time.
*   **Performance:** The simulation must run at interactive framerates (60 FPS) to allow for user intervention.

A naive single-threaded loop where each agent updates the grid directly would lead to:
1.  **Race Conditions:** Multiple agents modifying the same cell simultaneously.
2.  **Order Dependence:** The simulation outcome would depend on the iteration order of agents.
3.  **Performance Bottlenecks:** Unable to utilize multi-core CPUs for the large number of agents.

## Decision
We decided to adopt a **Hybrid Parallel Architecture** using `rayon` for data parallelism and a **Double-Buffered Cellular Automaton** for the environment.

### 1. Environmental Diffusion (Cellular Automaton)
The grid (Heat, Pheromones) is modeled as a Cellular Automaton.
*   **Double Buffering:** We maintain two grid states: `current` and `next`.
*   **Parallel Update:** We use `rayon` to iterate over the `next` grid in parallel. Each cell calculates its new state based on the `current` state of its neighbors. This ensures determinism and thread safety without locks.

### 2. Agent Decision (Read-Only Parallelism)
Agents determine their actions based on the current state of the world.
*   **Parallel Iteration:** We use `agents.par_iter_mut()` to update agent internal state (position, target selection) and decide on an action.
*   **Read-Only Environment:** Agents read from the immutable `current` grid to make decisions. They **do not** write to the grid directly during this phase.
*   **Action Generation:** Each agent produces an `Option<GridAction>` (e.g., `Move`, `Build`, `Attack`) instead of mutating the world.

### 3. Action Resolution (Sequential Mutation)
*   **Collection:** All generated actions are collected into a buffer.
*   **Sequential Application:** A single-threaded loop applies the actions to the grid. This handles collision resolution (e.g., two agents trying to build in the same spot) and ensures a consistent final state.

## Consequences

### Positive
*   **Scalability:** The system can support thousands of agents and large grids by utilizing all available CPU cores.
*   **Determinism:** The separation of "Decision" and "Resolution" phases makes the simulation logic easier to reason about and debug.
*   **Stigmergy Support:** The diffusion model naturally supports indirect communication via pheromones, which is essential for swarm intelligence.

### Negative
*   **Memory Overhead:** Double buffering the grid doubles the memory requirement for the environment.
*   **Sequential Bottleneck:** The final "Action Resolution" phase is single-threaded. If every agent generates a complex action every frame, this could become a bottleneck.
*   **Latency:** Information travels at a maximum speed of one cell per tick (for diffusion), which is physically realistic but may be too slow for some game mechanics.
