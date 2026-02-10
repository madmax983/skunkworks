# Mnemonic Rot (mnem-rot)

> "The data is not gone. It is merely forgetting."

**Mnemonic Rot** is a visualization of digital entropy and link decay within a software repository.

## Concept

This experiment visualizes the codebase as a living neural network or a constellation of knowledge.
*   **Nodes:** Source files (`.rs`).
*   **Edges:** Dependencies (`use`, `mod`).

Over time, the system simulates **Entropy**:
*   **Decay:** Nodes lose "health" and their connections weaken.
*   **Glitch:** As a node's health fails, its content—the actual source code—begins to corrupt. Characters swap, data is lost, and meaning dissolves into noise.
*   **Maintenance:** The user acts as a "Gardener" or "Pulse". Hovering over a node restores its health, "remembering" the code and fixing the corruption.

## Controls

*   **Right Click + Drag:** Pan the camera.
*   **Scroll:** Zoom in/out.
*   **Hover:** Heal a node and view its (potentially corrupted) content.

## Technical Details

*   **Stack:** Rust, `macroquad` (graphics), `walkdir` (filesystem), `regex` (parsing).
*   **Glitch Algorithm:** A stochastic text corrupter that simulates bit rot, case flipping, and "Zalgo" interference based on an intensity parameter.

## Purpose

To demonstrate that knowledge requires active maintenance. Without attention, structure degrades into chaos.
