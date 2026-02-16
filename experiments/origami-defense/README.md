# Origami Defense 🦢🏰

> "The battlefield is not static. It folds."

**Origami Defense** is a hybrid experiment combining **Tower Defense** with **Rigid Origami Physics**.

## Concept

The game is played on a **Miura-ori** tessellation. The map is a 3D surface that can be folded and unfolded by the player in real-time.

- **Dynamic Topology**: Enemies move along the grid faces. Towers fire projectiles in 3D space.
- **Tactical Folding**:
    - **Unfolded (Flat)**: High surface area creates more income (Solar Panel metaphor).
    - **Folded (Compressed)**: Brings enemies and towers closer together in 3D space, effectively increasing tower range and overlapping fields of fire.

## Controls

- **Arrow Left/Right**: Fold / Unfold the map.
- **WASD**: Move Cursor.
- **Space**: Build Tower (Cost: 20).
- **I/K/J/L**: Rotate Camera (Pitch/Yaw).
- **Q**: Quit.

## Lineage 🧬

- **Parent A**: `experiments/miura-interface` (The Geometry Engine).
    - Provided the `MiuraPattern` struct and the 3D-to-2D projection logic.
- **Parent B**: `experiments/chimera-defense` (The Game Logic).
    - Provided the Tower/Enemy entities and the `ChimeraVM` integration for tower logic.

## Emergent Trait

**"topological_range_compression"**: The ability to modify the metric of the battlefield to gain a tactical advantage. A tower that is out of range of an enemy on the grid can hit it if the intermediate space is "folded away".

---
*The Splice Surgeon*
