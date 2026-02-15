# Origami Diagnostics 🦢

> "Fold away the noise, reveal the bug."

**Origami Diagnostics** is an experimental code visualizer that treats your source file as a strip of paper. When compiler errors occur, the paper physically folds (pleats) to hide irrelevant code and bring error contexts adjacent to each other.

## Concept ⚛️

This combines **Crease Patterns** with **Error Reporting**.
- **Flat State (t=0.0):** The file is displayed normally as a long strip.
- **Folded State (t=1.0):** The paper folds in accordion pleats (zigzag) to compress sections of code without errors. Only the error locations and their immediate context remain flat and visible.

This metaphor turns "jumping to error" into a physical transformation of the workspace.

## Controls 🕹️

- **Right Arrow:** Fold the paper (increase compression).
- **Left Arrow:** Unfold the paper (flatten).
- **Mouse Drag:** Orbit the camera around the 3D model.
- **Scroll Wheel:** Zoom in/out.

## Implementation 📐

- **Stack:** `macroquad` (3D Rendering), `cargo_metadata` (Error Parsing).
- **Technique:**
  - The source code is rendered to a high-resolution texture.
  - A 3D mesh is generated where each line of code is a quad.
  - Vertex positions are interpolated between a flat strip and a pleated "compressed" state based on a fold parameter `t`.
  - The pleats use a sine-based zig-zag displacement in the Z-axis to maintain constant edge length (mostly).

## Status 🚧

Currently uses **Mock Data** for stability in the demo environment. The parsing logic for `cargo check` is implemented in `src/diagnostics.rs` but not hooked up to the main loop to avoid sandbox build times/locking issues.

To enable real diagnostics:
1. Uncomment `run_cargo_check` usage in `main.rs`.
2. Ensure you run it in a cargo project.

---
*Genesis: The Origamist*
