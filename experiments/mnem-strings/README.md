# Mnemonic Strings (mnem-strings) 🎻📉

**Lineage:** `mnem-rot` (Codebase Rot Graph) × `ferrous-strings` (Continuous Acoustic Physics).

## Concept

Acoustic Code Rot. The nodes of a codebase dependency graph are mapped to a continuous space, and the edges connecting them are represented as physical acoustic strings.
As the codebase "rots" (entropy increases), the health of the nodes decreases, which detunes the connected strings by dropping their tension.

When a maintainer (the user) heals or interacts with the codebase by hovering over nodes, the graph structure plucks the connected strings.
- High-health, maintained code produces tuned, clear, and stable frequencies.
- Low-health, rotting code produces chaotic, dissonant, and heavily damped audio frequencies.

This translates the visual representation of structural decomposition directly into an audio-visual symphony of rot.

## Controls

*   **Right Click + Drag:** Pan the camera.
*   **Scroll:** Zoom in/out.
*   **Hover:** Heal a node, reveal its corrupted content, and physically pluck its connected strings.

## Technical Details

*   **Stack:** Rust, `macroquad` (graphics), `cpal` (audio synthesis), `walkdir` (filesystem).
*   **Genetics:** Inherits `ferrous-strings` Karplus-Strong string synthesis model and applies it directly to the network structure of `mnem-rot`.
