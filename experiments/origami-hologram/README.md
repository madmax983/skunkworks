# Origami Hologram

**Status:** CONDEMNED (Grace Period)
**Lineage:** `origami` × FFT holography concepts.

## Description

This experiment crosses a procedural Miura-ori mesh generation algorithm (`origami`) with a simple Fast Fourier Transform (FFT) based holographic simulation using `rustfft`. The simulation projects the three-dimensional geometry of the folded origami sheet onto a two-dimensional holographic recording plane, treating the `Z` height of the folds as the amplitude (or phase) of a scattered wavefront. It then computationally reconstructs the wavefield using an inverse FFT to display the resulting 2D spatial intensity pattern.

- **Organism Core:** Procedural Miura-ori mesh structure.
- **Holographic Substrate:** 2D Complex Wavefield array processed via `rustfft`.
- **Emergence:** As the origami mesh breathes (expanding and contracting via an oscillating `extension_factor`), the spatial frequencies of its folds change. The reconstructed hologram allows the viewer to observe this structural change not as a 3D object, but as a shifting interference pattern.

## Execution Quality
Currently functional but minimal. Shows the structural mapping but lacks advanced phase modulation or deeper interaction.

## Controls
- **Arrow Keys:** Adjust the phase shift angle of the reconstruction beam (`reconstruction_angle_x`, `reconstruction_angle_y`).
- **Q / Esc:** Quit.

## Lineage Details
- **Parent A (origami):** Provides `generate_miura_mesh` and structural constraints.
- **Parent B (rustfft):** Provides the mathematical substrate for calculating the 2D Inverse Fast Fourier Transform, converting frequency space back into a spatial intensity map.
