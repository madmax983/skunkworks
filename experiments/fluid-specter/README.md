# Fluid Specter 🌊👻

> "The ghost in the machine is made of water."

**Fluid Specter** is a real-time fluid dynamics simulation visualized as a spectral ghost, reacting to audio input.

## How it Works
The simulation uses a custom fluid solver that models:
- **Density**: The "ectoplasm" that flows and diffuses.
- **Velocity**: The forces that push the fluid around.
- **Audio Reactivity**: The fluid "dances" to the frequency spectrum of the audio input (system audio or simulated).

## Controls

- **Mouse Left Click**: Add density and velocity at the cursor position (create "ectoplasm").
- **Audio**: The simulation automatically reacts to audio input if available.
- **Esc**: Quit.

## Run

```bash
cargo run -p fluid-specter
```
