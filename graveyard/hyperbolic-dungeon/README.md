# Hyperbolic Dungeon

**"The Non-Euclidean Rogue"**

A roguelike set on the Poincaré disk projection of the hyperbolic plane. Space expands exponentially as you explore.

## Concept
- **Infinite Dungeon**: Generates endlessly based on a {4,5} tiling (4 squares meet at a vertex, or 5 squares meet at a vertex?). The code uses `{4,5}` logic (squares).
- **Hyperbolic Geometry**: Movement and visibility follow hyperbolic rules.
- **Topological weirdness**: The world is a tree (Bethe Lattice) visually mapped to the disk.

## Features
- [x] Hyperbolic Tiling Rendering
- [x] Infinite procedural generation (Tree-based)
- [ ] Entities (Player, Enemies)
- [ ] Combat
- [ ] Items
- [ ] Improved Visuals (FOV, Fog)

## Controls
- `WASD` / Arrows: Move
- `Q`: Quit

## Dev Notes
- Uses `ratatui` for TUI rendering.
- Uses `poincare-disk` crate for math.
