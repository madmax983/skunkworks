# Rigid Origami: Deployable Structure Simulation 🛰️

> "Mathematics is the art of giving the same name to different things." - Henri Poincaré

**Rigid Origami** is a simulation of deployable space structures (like solar arrays) using the **Miura-ori** tessellation. It focuses on the **kinematics** of the fold—calculating the exact position of every vertex based on a single "expansion" parameter.

## Concept ⚛️

Unlike PBD (Position Based Dynamics) simulations which approximate folding using springs, this experiment uses **analytical solutions** for the Miura-ori geometry. This ensures:
- **Zero strain:** The faces remain perfectly rigid.
- **Constant edge lengths:** The structure is geometrically valid at every frame.
- **Precise deployment:** Suitable for engineering visualization.

## Controls 🕹️

- **Arrow Left/Right:** Fold/Unfold the structure.
- **Space:** Toggle Auto-Deployment animation.
- **Mouse Drag:** Orbit camera.
- **Scroll:** Zoom.

## Implementation 📐

- **Stack:** `macroquad` (Rendering), `std` (Math).
- **Solver:** Parametric equations for Miura-ori vertex positions.
- **Visuals:** Solar panel aesthetics with starfield background.

---
*Genesis: The Origamist*
