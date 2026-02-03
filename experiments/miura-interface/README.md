# Miura Interface 🦢

> "Origami is constrained geometry."

**Miura Interface** is an experimental responsive layout engine based on the **Miura-ori** tessellation. It demonstrates how physical folding mechanics can map to UI properties like responsiveness and density.

## Concept ⚛️

Combine **Miura-ori** (a rigid origami fold used for solar panels) with **Responsive UI**.
- **Expansion (Rho = 1.0):** The UI is flat, wide, and readable.
- **Contraction (Rho -> 0.0):** The UI folds accordion-style. Content becomes denser or disappears (LOD).

This experiment simulates the exact kinematic geometry of the Miura fold, parameterized by the fold angle $\rho$.

## Controls 🕹️

- **Space:** Toggle Auto-Responsive Mode (binds fold angle to terminal width).
- **Left/Right:** Manually fold/unfold.
- **WASD:** Rotate the 3D view.
- **Q:** Quit.

## Technical Details 📐

- **Engine:** `ratatui` for TUI, `nalgebra` for 3D math.
- **Projection:** Orthographic projection of the 3D mesh onto the 2D terminal canvas.
- **Overlay:** Text widgets (`Paragraph`) are projected from 3D face centroids to 2D screen coordinates, adapting to the fold state.

## Moonshot 🚀

This explores the idea of "Spatial Interfaces" where navigation isn't just scrolling, but *unfolding* data. Imagine a filesystem where directories are folded bundles that expand when you focus on them.

---
*Genesis: The Origamist*
