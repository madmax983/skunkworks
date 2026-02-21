# Hyperbolic Chimera 🧬🧭

**Parents**: `experiments/chimera-lang` × `experiments/hyperbolic-dungeon`

ChimeraVM agents inhabiting the Poincaré Disk (Hyperbolic Plane).

## Concept

Agents are biological entities driven by a `ChimeraVM` genetic program. Instead of a 2D grid, they inhabit a non-Euclidean hyperbolic space.

*   **Sensors**: Agents sense their distance from the origin (center of the disk).
*   **Actuators**: Agents control their movement Angle and Speed via `GWrite` operations.
*   **Physics**: Movement is applied via Möbius transformations, simulating navigation in hyperbolic geometry.

## Run

```bash
cargo run -p hyperbolic-chimera
```

## Lineage

*   **From Chimera Lang**: The Virtual Machine, Genetics, and biological imperatives (Energy, Death).
*   **From Hyperbolic Dungeon**: The TUI rendering of the Poincaré disk and the underlying geometry math.
