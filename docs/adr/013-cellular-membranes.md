# 13. Cellular Membranes (Grid Architecture)

Date: 2024-05-22

## Status

Accepted

## Context

The `chimera-lang` experiment simulates a biological organism on a 2D grid. Currently, the grid is an open plane where molecules and organelles move freely. This limits the ability to create complex spatial structures, compartments, or "cell walls" that are fundamental to biological complexity.

We want to introduce a mechanism to compartmentalize the grid, allowing for the creation of mazes, protected storage areas, and complex organelle circuits.

## Decision

We will implement **Cellular Membranes** as a set of walls between grid cells.

1.  **State**: The `ChimeraVM` (under `nova` feature) will store a 16x16 grid of bitmasks (`Vec<Vec<u8>>`).
    *   Bits: 1 = North, 2 = East, 4 = South, 8 = West.
    *   Walls are shared: A wall North of (0,0) is the same as a wall South of (15,0) (due to toroidal wrapping) or relative neighbor. Note: Currently the grid wraps. We should decide if walls wrap. Yes, consistent with toroidal geometry.

2.  **Opcodes**:
    *   `Membrane(direction)`: Toggles a wall in the specified direction from the current context location (`context_loc`). This operation updates both the current cell's mask and the neighbor's reciprocal mask.
    *   `Osmosis(direction)`: Attempts to move the `context_loc` in the specified direction. If a wall exists, the movement fails (and returns 0/false to stack, or just doesn't move). If no wall, it succeeds (returns 1/true).

3.  **Organelle Interaction**:
    *   `Ribosome` organelles (which move autonomously on the grid) will be blocked by walls. This allows creating tracks or cages for them.
    *   `Migrate` and `Conjugate` operations might also be constrained, but for now, we focus on `Ribosome` and explicit `Osmosis` movement.

4.  **Visualization**:
    *   The TUI will render vertical walls as `|` between cells.
    *   The TUI will render horizontal walls using `Underline` modifiers (South wall).

## Consequences

*   **Complexity**: Increases the state complexity of the VM.
*   **Visuals**: Improves the visual density and structure of the Petri Dish.
*   **Gameplay**: Enables new puzzle mechanics and logic gates based on spatial containment.
*   **Performance**: Negligible impact on performance (bitwise checks).

## Compliance

*   Follows the "Mad Scientist" theme of evolving the language.
*   Uses `nova` feature flag to avoid bloating the core.
