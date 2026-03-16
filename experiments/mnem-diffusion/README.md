# Mnemonic Diffusion (mnem-diffusion)

> "The decay of knowledge acts as a catalyst for new patterns."

**Mnemonic Diffusion** is a hybrid visualization of digital entropy mapped onto a continuous reaction-diffusion substrate.

## Concept & Lineage

This experiment is a cross between **mnem-rot** and **gray-scott**. It demonstrates 'Mnemonic Diffusion', where codebase files act as active chemical catalysts on a 2D Gray-Scott substrate.

*   **From `mnem-rot` (Parent A):** Codebase graph reading, force-directed layout, entropy calculation, and visual glitching of source code.
*   **From `gray-scott` (Parent B):** The 2D Reaction-Diffusion grid simulating the diffusion of virtual chemicals U and V.
*   **The Novel Trait (Phenotype):** 'Mnemonic Diffusion'. Nodes in the codebase graph physically drop the 'V' chemical (representing fungus or rot) onto the substrate based on their current entropy level and file size. The text decay creates a continuous, morphological environment—a macroscopic-microscopic feedback loop of information loss.

## Controls

*   **Right Click + Drag:** Pan the camera.
*   **Scroll:** Zoom in/out.
*   **Hover:** Heal a node, view its corrupted content, and stop it from dropping the 'V' chemical.

## Technical Details

*   **Stack:** Rust, `macroquad` (graphics), `gray-scott` (shared library for reaction-diffusion).
*   **Integration:** The node graph lives in a continuous real-world space, and its properties map directly to grid cells of the `GrayScott` system each frame.

## Purpose

To explore how the decay of structured information (source code) can drive the emergence of complex, organic patterns (Reaction-Diffusion).
