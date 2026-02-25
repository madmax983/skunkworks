# Origami Satellite 🛰️

> "Unfolding the cosmos, one crease at a time."

**Origami Satellite** is a simulation of a deployable solar array using the Miura-ori tessellation. It explores the kinematics of rigid origami in a space context.

## Concept ⚛️

Combine **Rigid Origami** (Miura-ori) with **Deployable Structure Simulation**.
- **Deployment:** The solar array unfolds from a compact state to a fully extended state, parameterized by a single degree of freedom (extension).
- **Kinematics:** The vertices are calculated using exact geometric constraints from `crates/origami`.
- **Visuals:** 3D rendering with `macroquad`, showing the "breathing" motion of the fold.

## Controls 🕹️

- **Right Click + Drag (Vertical):** Deploy or retract the solar wings.
- **Left Click + Drag:** Orbit the camera around the satellite.
- **Scroll:** Zoom in/out.

## Technical Details 📐

- **Engine:** `macroquad` for 3D rendering.
- **Math:** `crates/origami` for Miura-ori vertex generation.
- **Simulation:** A single parameter `extension` (0.0 to 1.0) drives the complex 3D transformation of hundreds of vertices.

## Moonshot 🚀

This experiment validates the use of Rust for simulating high-precision deployable structures. The math used here is the same principle used in actual space missions (like the Space Flyer Unit).

## Notes

- **Audio:** Real-time sonification was planned but disabled due to environment limitations (missing `alsa` headers). Ideally, the deployment sound would shift in frequency and timbre based on the tension of the fold.

---
*Genesis: The Origamist*
