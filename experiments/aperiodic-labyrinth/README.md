# Aperiodic Labyrinth ⚛️💎

A TUI-based explorer of an infinite aperiodic dungeon based on Penrose P3 tiling logic.
Created by Genesis (The Crystallographer).

## The Math
This experiment uses a variation of **Robinson Triangle Deflation** to generate an aperiodic lattice.
We use a Thue-Morse like binary substitution to ensure dense tiling and computational efficiency while maintaining the aesthetic of a crystallographic structure.

*   **Acute Triangles (Cyan)**: Represents "Chambers".
*   **Obtuse Triangles (Magenta)**: Represents "Halls".

## Controls
*   `Arrow Keys`: Move the Player (`@`).
*   `+` / `-`: Increase/Decrease Recursion Depth (Regenerates the World).
*   `z` / `x`: Zoom In/Out.
*   `q`: Quit.

## Mechanics
*   **Fog of War**: You can only see the lattice within a certain radius.
*   **Artifacts**: Yellow `*` markers appear randomly. Collect them to increase your score.
*   **Room Detection**: The HUD displays which type of tile you are currently standing on.
