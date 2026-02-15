# Myco-GC: Fungal Garbage Collector 🍄🗑️

**"Fungal decomposition + Garbage collection algorithms"**

A biological simulation of memory management where a fungal colony acts as a distributed Garbage Collector.

## The Concept

In this simulation, the "Heap" is a physical terrain.
- **Objects** (Memory Allocations) are nodes in a graph.
- **Roots** are the source of life (Global Variables).
- **References** are edges connecting objects.
- **Live Objects** are reachable from Roots and glow green.
- **Dead Objects** (Garbage) are unreachable, turn brown, and emit "Rot" pheromones.

The **Fungus** (Mycelium) is the garbage collector:
- **Spores** germinate and send out hyphae.
- **Hyphae** follow the scent of Rot (Dead Objects).
- **Consumption**: When hyphae reach a dead object, they decompose it (deallocate), releasing space and gaining energy to grow further.
- **Symbiosis**: The fungus cleans the heap, preventing memory leaks (accumulation of dead objects).

## Visuals

- **Green Circles**: Live Objects (Safe).
- **Red/Brown Circles**: Dead/Rotting Objects (Food for the fungus).
- **White Trails**: Fungal Hyphae growing towards rot.
- **Grey Lines**: References.
- **Grid**: The underlying memory address space.

## Controls

- **Space**: Spawn a cluster of objects (Allocations).
- **Click**: Spawn a single object and link it.
- **G**: Create Garbage (Cut references randomly).
- **R**: Reset the simulation.
- **Esc**: Quit.

## Technical Details

- **Engine**: `macroquad` (Rust).
- **Simulation**:
    - **Mark Phase**: BFS from roots to determine liveness.
    - **Rot Phase**: Dead objects accumulate "rot" value over time.
    - **Growth Phase**: Hyphae use a sensor-based steering behavior (Physarum-like) to follow gradients.

## Run

```bash
cargo run -p myco-gc
```
