# Ferrous Hologram 🧬

**Lineage:** `ferrous-fluid` (Soft-body magnetics) × `hologram-text` (FFT/Spectral Analysis)

## Concept
A particle system where the "forces" are not calculated by direct N-body interaction or spatial grids alone, but by a "Spectral Feedback Loop".

1. **Spatial Domain:** Particles exist in 2D space and emit "magnetic density" into a grid (The Platter).
2. **Frequency Domain:** The Platter's density field is transformed via FFT into the frequency domain (The Hologram).
3. **Spectral Resonance:** The spectrum is filtered (band-pass) to amplify certain "resonant frequencies" of the swarm structure.
4. **Ghost Potential:** The modified spectrum is inverse-transformed (IFFT) back into spatial space, creating a "Ghost Potential Field".
5. **Feedback:** Particles feel a force from the gradient of this Ghost Potential, pushing them into structures that resonate with the filter.

## Controls
- **Q**: Quit

## Implementation
- **TUI:** Uses `ratatui` and `crossterm`.
- **Physics:** Custom particle system.
- **Math:** `rustfft` for Fast Fourier Transforms.
