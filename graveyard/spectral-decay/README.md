# Spectral Decay ⚛️

"Memory is not a storage device, but a reconstruction process."

This experiment simulates the degradation of a digital image stored in a "Holographic Memory" (Frequency Domain), subject to simulated physical entropy.

## Concept
The image is converted to the frequency domain (2D FFT) and stored as a buffer of Complex numbers.
Each frame, the memory is subjected to:
- **Phase Drift:** High frequencies rotate their phase randomly, simulating loss of coherence.
- **Amplitude Decay:** High frequencies decay faster (Low Pass Filter), simulating diffusion.
- **Cosmic Rays:** Random spikes in the spectral domain, causing wave-like interference patterns in the spatial domain.

The image is reconstructed (Inverse FFT) in real-time to visualize the decay.

## Controls
- **Space:** Pause/Resume decay.
- **R:** Reset memory to original state.

## Tech Stack
- **Bevy 0.14**: Visualization and ECS.
- **RustFFT**: Fast Fourier Transform.
- **Image**: Texture management.

## Moonshot
This is part of the **Genesis: The Archivist** directive to visualize digital decay and lossy memory simulation.
