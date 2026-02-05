# Hydro-Acoustics ⚛️🌊

**Genesis Experiment: Wave Interference + Audio Synthesis**

Simulating water in all its forms—crashing waves, turbulent vortices, serene ripples—with physical accuracy and visual beauty.

## Concept
This experiment combines a GPU-accelerated 2D Wave Equation simulation (`wgpu` Compute Shaders) with real-time audio synthesis (`cpal`).
The surface of the water acts as a modulation source for an audio oscillator. "Hydrophones" placed in the water read the local pressure/height and drive the frequency and amplitude of the sound.

## Controls
- **Mouse Click/Drag**: Disturb the water surface (create ripples).
- **Audio**: The sound changes based on the wave activity at the center and corners of the tank.

## Architecture
- **Physics**: 2D Damped Wave Equation solved via explicit integration on GPU.
- **Visuals**: Height-mapped water rendering with custom ocean palette.
- **Audio**: "Hydrophone" data is read back from the GPU asynchronously to avoid stalling the render loop, then fed into a CPU-based audio engine.

## Status
- [x] GPU Wave Simulation (512x512 grid)
- [x] Interactive Ripples
- [x] Hydrophone Readback (Async Buffer Mapping)
- [x] Audio Modulation
- [ ] Refraction/Caustics (Future Work)

## Build
```bash
cargo run -p hydro-acoustics
```
*Note: Requires a GPU compatible with wgpu (Vulkan/Metal/DX12).*
*Audio requires ALSA dev headers on Linux (`libasound2-dev`). If missing, it runs in visual-only mode.*
