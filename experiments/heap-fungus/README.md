# Heap Fungus 🍄💾

**"Fungal Decomposition + Garbage Collection"**

A biological visualization of memory management.

## The Concept

In this simulation, the computer's memory heap is a forest floor.
- **Allocations**: New mushrooms (Green) sprouting from the soil.
- **References**: Mycelial strands (Hyphae) connecting objects.
- **Roots**: The "Global Roots" (Blue) that anchor the living network.
- **Garbage Collection**: A natural cycle of life and death.

## Phases

1.  **GROW**: New objects are allocated and linked randomly.
2.  **MARK**: A pulse of energy flows from the Roots through the Hyphae. Reachable objects glow **Cyan**.
3.  **DECAY**: Unreachable objects, cut off from the energy source, begin to rot (**Brown**).
4.  **SWEEP**: Decomposer agents (Red dots) consume the rotting matter, returning the space to the void.

## Implementation

- **Language**: Rust
- **Visuals**: `macroquad`
- **Graph**: `petgraph` (StableGraph)

## Controls

- `Space`: Force jump to the next phase.

## Running

```bash
cargo run -p heap-fungus
```
