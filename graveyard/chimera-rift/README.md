# Chimera-Rift 🧬🌀

**Cross:** `chimera-lang` × `impossible-explorer`

## Concept
A 2D simulation where **ChimeraVM** agents navigate a maze-like world to find food.
The twist? They possess the ability to **Spawn Portals** (Bio-Spatial Manipulation).

## Lineage
- **Parent A**: `chimera-lang` (The Brain)
  - Provides the `ChimeraVM` genetic programming engine.
  - Agents evolve DNA to sense food/walls and output movement commands.
- **Parent B**: `impossible-explorer` (The Physics)
  - Provides the concept of "Portal Navigation" and non-Euclidean shortcuts.
  - Portals link two points in space, allowing instant traversal.

## Novel Trait: Evo-Portals
Agents have specific genes (OpCode sequences) that trigger:
- `Signal(0)`: Drop an **Anchor** (Portal Entry/Blue).
- `Signal(1)`: Drop a **Link** (Portal Exit/Orange) connected to the Anchor.

This allows agents to build their own shortcuts through the maze.
Evolutionary pressure (Food = Energy) should select for agents that learn to bridge gaps or bypass walls.

## Visuals
- **Agents**: Colored circles.
- **Food**: Green squares.
- **Walls**: Gray blocks.
- **Portals**: Blue (Entry) and Orange (Exit) circles connected by a line.

## Status
- Compiles: ✅
- Evolution: Active.
- Portals: Functional.
