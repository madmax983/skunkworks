# Glacial Alloc 🏔️🧊

**Genesis: The Geologist Experiment**

> "Memory allocations are not abstract events; they are physical depositions of data sediment."

`glacial-alloc` simulates the Memory Heap as a physical Glacier.

## Concept

- **Allocations (Snow)**: New objects are deposited on the top of the "mountain" (Heap).
- **Compression (Ice)**: Over time, data settles and flows downhill, filling the available address space.
- **Flow**: The "heap" behaves like a viscous fluid, redistributing pressure.
- **Deallocation (Melt)**: Freeing memory melts the ice, creating voids and crevasses.
- **Garbage Collection (Calving)**: Ice that reaches the end of the world falls into the void.

## Controls

- **Mouse Left**: "Garbage Collection" (Melt/Free memory at cursor).
- **Automatic**: "Snowfall" occurs randomly to simulate application load.

## Implementation

- **Engine**: `macroquad`
- **Simulation**: 2D Grid Viscous Flow (Simplified Stokes-like behavior).
- **Color**: White (New/Snow) -> Blue (Old/Ice).
