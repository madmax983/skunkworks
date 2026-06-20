# 🌊 Sediment Flow

**Lineage:** `code-erosion` × `karman-text-street`

A hybrid experiment combining geological terrain generation from git history with real-time Lattice Boltzmann Method (LBM) fluid dynamics.

## Concept

- **Terrain (Parent A):** Generated from the repository's git commit history. Files are mapped to coordinates; commits cause "uplift", creating mountains of code. (`code-erosion`)
- **Fluid (Parent B):** A wind-tunnel style fluid simulation flows continuously across the map. (`karman-text-street`)
- **Emergence:** The fluid interacts with the code terrain. Mountains act as obstacles, creating turbulence, eddies, and wake flows. The "shape" of the codebase determines the aerodynamic properties of the history.

## Controls

- `Space`: Pause/Resume commit replay.
- `+ / -`: Increase/Decrease replay speed.
- `r`: Reset simulation.
- `q` / `Esc`: Quit.

## Technical Details

- Uses `ratatui` for TUI rendering.
- Implements LBM (D2Q9) for fluid simulation.
- Uses `git log` parsing to drive terrain growth.
