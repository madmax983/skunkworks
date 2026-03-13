# Chaos-Reaper 🍄💀⚔️

**"Chaotic garbage collection under duress"**

A hybridization of `chaos-pendulum` and `myco-reaper`.

## Concept
The "Heap" is a growing fungal ecosystem, with nodes forming complex dependencies and references. But here, the garbage collection must happen under extremely volatile conditions. A chaotic double pendulum swings violently through the "heap" forest.

The pendulum bob acts as the "Chaos Reaper." Any allocated memory node that the pendulum bob collides with is immediately killed and its data corrupted, regardless of whether it was reachable from the root.

This forces the underlying Garbage Collection system (whether manual, reference counting, or mark-and-sweep) to constantly attempt to clean up and repair the broken, decaying references left behind in the wake of the pendulum's chaotic destruction.

## Lineage
- **From `myco-reaper`**: The fungal memory ecosystem (Heap, Nodes, Roots). The Garbage Collection modes (Manual, Reference Counting, Mark & Sweep) and the organic "growth" logic of the nodes.
- **From `chaos-pendulum`**: The `PendulumSystem` providing a mathematically chaotic double-pendulum physics model using Verlet integration.
- **Novel Trait (Phenotype)**: **Chaotic Garbage Collection**. The environment isn't just slowly growing; it's actively being destroyed by an unpredictable physical force, creating a visual tug-of-war between organic data allocation and chaotic data corruption.

## Controls
- `R`: Cycle GC Mode (Manual -> RC -> MarkSweep).
- `A`: Toggle Auto-Allocation.
- `C`: Create a detached reference cycle (Circular Garbage).
- `Space`: Manual Mark+Sweep.
- `M`: Manual Mark.
- `S`: Manual Sweep.

## Running
```bash
cargo run -p chaos-reaper
```