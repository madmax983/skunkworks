# Hologram Text 🌟

A simulation of **Digital Holography** and the **Holographic Principle** in the terminal.

## Concept

This experiment visualizes how information (text) can be encoded into a 2D wave interference pattern (a hologram) and then reconstructed by applying a reference beam (inverse transform).

1.  **Object Wave:** The text is rendered as a 2D bitmap.
2.  **Reference Beam:** A simulated light wave strikes the text at an angle.
3.  **Hologram:** The interference pattern (FFT of the modulated object) is stored. This looks like random noise (speckle).
4.  **Reconstruction:** By applying the conjugate reference beam (Inverse FFT with phase shift), the original text is recovered.

## Controls

- **Arrow Keys:** Adjust the angle of the reconstruction beam.
- **Enter:** Reset angle to the perfect recording angle (clear image).
- **Type Text:** Update the hologram buffer in real-time.
- **ESC:** Quit.

## Theory

The experiment demonstrates that the spatial information of the text is distributed across the entire frequency domain of the hologram. Even if you only reconstruct a part of it (or look at it from a wrong angle), the information is there, just distorted.

"The text is everywhere and nowhere until you look."

---
*Built by Nova 🌟*
