# Hyperbolic Mycelium 🍄‍🟫

**Lineage:** `etymological-mycelium` × `hyperbolic-lexicon`

This experiment visualizes the evolution of a codebase as a fungal network growing through Hyperbolic Space (the Poincaré Disk).

## Concept

*   **Code as Mycelium:** Git commits are nodes in a fungal network. The network grows from the initial commit (the spore) outwards.
*   **Hyperbolic Growth:** Because codebases grow exponentially (or at least branch heavily), the infinite boundary of the Poincaré Disk provides the necessary space to visualize deep histories without clutter.
*   **Semantic Drift:** Commits drift away from their parents based on "semantic distance" (Levenshtein distance of commit messages or diffs).

## Controls

*   **WASD / Arrow Keys:** Pan the camera (Möbius transformation).
*   **Scroll:** Zoom (Scale the view).
*   **Space:** Pulse the network (simulate growth spurt).

## Implementation Details

*   **Engine:** `macroquad`
*   **Geometry:** `poincare-disk`
*   **Data Source:** `git2` (local repository history)
