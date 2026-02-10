# Origami Window 🦢🖥️

> "The interface is not a flat sheet; it is a landscape that folds."

**Origami Window** is a Moonshot experiment combining **Rigid Origami Kinematics** with **Responsive UI Layouts**. It simulates a **Miura-ori** tessellation that acts as a window surface. As you resize the window (mapped to mouse X), the surface folds according to the exact mathematical constraints of the Miura fold, creating a "Lenticular User Interface".

## Concept ⚛️

Traditional responsive design flows content into a rectangular container. **Origami Design** adapts the container itself.
- **Flat State ($\rho \approx 1.0$):** The window is wide and flat. Content is fully visible.
- **Folded State ($\rho \to 0$):** The window collapses accordion-style. The surface becomes a 3D topography.

This experiment uses **Exact Kinematics** (not physics simulation) to ensure the fold is mathematically perfect and glitch-free.

## Controls 🕹️

- **Mouse X:** Controls the Fold Angle ($\rho$). Move left to fold, right to unfold.
- **Mouse Wheel:** Zoom in/out.
- **Right Click + Drag:** Rotate the camera around the model.

## Technical Details 📐

- **Engine:** `macroquad` (0.4)
- **Math:** `nalgebra` for vector operations.
- **Kinematics:** Implements the parametric equations for Miura-ori based on the extension ratio $\rho$ and sector angle $\gamma$.
  - $w = a \rho$
  - $d = a \sqrt{1 - \rho^2}$
  - Constraints: $\rho \le \sin \gamma$
- **Rendering:** Procedural UI texture mapped via UVs to the folding mesh. Flat shading highlights the crease pattern.

## Moonshot Status 🚀

This is a graphical evolution of the text-based `miura-interface`. It proves that rigid body kinematics can be applied to UI elements for novel interaction paradigms.

---
*Genesis: The Origamist*
