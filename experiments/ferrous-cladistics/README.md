# 🧲 Ferrous Cladistics

> **"Code clusters like iron filings in a magnetic field."**

## Concept
A visualization where code functions are harvested from the disk and represented as magnetic particles. Their "magnetic charge" is determined by their function signature (return type, argument count, shared types). Functions with similar signatures attract and cluster, forming a physical taxonomy (cladistics) of the codebase.

## Lineage
- **Parent A**: `experiments/ferrous-graph` (Magnetic physics, TUI rendering)
- **Parent B**: `experiments/code-bio-dome` (Function harvesting logic)
- **Novel Trait**: **Magnetic Cladistics**. Self-organizing code taxonomy via magnetic physics.

## How it Works
1.  **Harvest**: Scans the current directory (or parent directories) for Rust functions using Regex.
2.  **Physics**: Each function becomes a particle (`Body`).
3.  **Cladistics Force**:
    - Particles attract if they have the same Return Type (+1.0).
    - Particles attract if they have the same Argument Count (+0.5).
    - Particles attract if they share Argument Types (+0.2 per match).
4.  **Magnetism**: Particles leave a magnetic trail on a background grid (`Platter`), which influences other particles, creating "paths" of least resistance.

## Controls
- `Q`: Quit
- `+/-`: Zoom
- `Arrows`: Pan
- `R`: Reset view

## Colors
- **Yellow**: `Result`
- **Blue**: `Option`
- **Green**: `String`
- **Cyan**: `Vec`
- **Red**: `bool`
- **Gray**: `()` or void

## Status
Compiles and runs. Verified by The Splice Surgeon 🧬.
