# 13. Cellular Membranes for Chimera Grid

Date: 2024-05-24

## Status

Accepted

## Context

The `chimera-lang` experiment aims to simulate biological processes. A key aspect of biology is compartmentalization (cell membranes, organelles). Currently, the "Petri Dish" grid is an open space where entities and signals move freely. To support more complex organism structures and specialized organelles, we need a way to define boundaries.

## Decision

We will implement a "Cellular Membrane" system using a secondary grid layer in the `ChimeraVM`.

1.  **Data Structure**: Add `membranes: Vec<Vec<u8>>` to `ChimeraVM` (under `nova` feature).
    *   This is a 16x16 grid of bitmasks.
    *   Bits represent walls in cardinal directions: North (1), East (2), South (4), West (8).
    *   Walls are "reciprocal": Setting a South wall at (y, x) implies a North wall at (y+1, x).

2.  **Opcodes**:
    *   `Membrane(dir)`: Toggles the wall in the specified direction. Updates both the local cell and the neighbor to ensure consistency.
    *   `Osmosis(dy, dx)`: Attempts to move through a wall. Requires significantly higher energy than standard movement. Fails if energy is insufficient.

3.  **Interaction with Existing Mechanics**:
    *   `Migrate`: Standard movement logic will now check for walls. If a wall exists in the target direction, movement is blocked (and a small energy penalty is applied).
    *   **Diffusion**: Biological diffusion (hormones, waste, light) will be blocked by membranes, allowing for concentration gradients within compartments.

4.  **Visualization**:
    *   The TUI will render walls. South walls will be rendered as underscores. East walls will be rendered as vertical bars `|` between cells.

## Consequences

*   **Positive**:
    *   Enables creation of complex, multi-cellular structures.
    *   Allows for "immune system" logic where viruses/agents are trapped.
    *   Adds strategic depth to resource management (active transport vs. passive diffusion).
*   **Negative**:
    *   Slightly increases memory footprint of the VM.
    *   Pathfinding (if implemented later) becomes more complex.
    *   Diffusion algorithms are slightly more expensive due to wall checks.
