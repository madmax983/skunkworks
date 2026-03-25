# Hologram Tank

This experiment combines `experiments/ripple-tank` and `experiments/hologram-text` to create an "Acoustic Holography" simulation.

## Concept
The 2D acoustic wave tank's pressure field is treated as an optical interference pattern.

## Novel Trait
**Spectral Wave Mechanics**. As standing waves and interference patterns form in the physical wave tank, they are transformed via a 2D Fast Fourier Transform (FFT) into a holographic projection. This visualizes the resonant modes of the acoustic space in the frequency domain, merging continuous spatial physics with spectral holography.

## Lineage
- **From `ripple-tank`**: The 2D wave equation simulation, providing a continuous grid of physical displacement (pressure).
- **From `hologram-text`**: The FFT-based optical interference rendering pipeline, applying reference beam modulation and reconstruction.
