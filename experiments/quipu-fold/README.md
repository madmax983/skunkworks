# Quipu Fold 🧶

> "The Knot is the Fold."

**Quipu Fold** is a hybrid experiment visualizing Inca Quipu cords mapped onto a Miura-ori rigid origami surface.

## Lineage 🧬

- **Parent A:** `experiments/miura-interface` (Genesis: The Origamist) - Provided the Miura-ori tessellation geometry and folding mechanics.
- **Parent B:** `experiments/quipu-symphony` (The Splice Surgeon) - Provided the Quipu data structure and knot logic (`quipu` crate).

## Concept ⚛️

This experiment explores **Topological Data Compression**.
- **Cords** are mapped to the vertical columns of the Miura mesh.
- **Knots** (representing powers of 10) are placed at the vertices.
- **Folding (Rho)** compresses the surface, bringing the cords and knots into close proximity.

## Emergent Traits 🦋

- **Folded Arithmetic:** The spatial density of data (knots) is controlled by the physical state of the medium (folded paper).
- **Zig-Zag Cords:** The cords naturally follow the zig-zag path of the Miura crease pattern, creating a "Waving Cord" aesthetic.

## Controls 🕹️

- **Arrow Keys:** Fold/Unfold the surface (change Rho).
- **WASD:** Rotate the 3D view.
- **Q:** Quit.

## Technical Details 📐

- **Engine:** `ratatui` (TUI), `nalgebra` (3D Math).
- **Physics:** Spring-damper system for smooth folding transitions.
- **Data:** Randomly generated 8-digit numbers represented as Quipu cords.

---
*The Splice Surgeon 🧬*
