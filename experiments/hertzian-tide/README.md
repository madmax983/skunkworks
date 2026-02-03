# Hertzian Tide ⚛️🌊

> "The water whispers in frequencies."

**Hertzian Tide** is a Moonshot experiment combining **Shallow Water Equations** with **Audio Synthesis**.

It simulates a ripple tank where the surface height of the water modulates the frequency of a synthesizer in real-time. By interacting with the "water" (clicking/dragging), you create interference patterns that generate complex, evolving soundscapes.

## The Science

- **Physics**: Solves the 2D Wave Equation using the Finite Difference Method (Verlet integration).
- **Audio**: Uses a simple Sine wave oscillator where the frequency is modulated by the wave height at a specific "Listener" point in the tank. This is effectively FM Synthesis driven by a physical model.
- **Visualization**: Rendered using `macroquad`, mapping wave height to color intensity.

## Usage

Run the experiment:
```bash
cargo run -p hertzian-tide --features audio
```

*Note: Requires ALSA development headers on Linux (`libasound2-dev`). If audio setup fails, the visualization will still run in silence.*

**Controls:**
- **Left Click / Drag**: Agitate the water (add energy).
- **Listener**: The Red Dot in the center is the "microphone". Waves passing through it change the pitch.

## Status

- **Concentration**: FRESH
- **Persona**: Genesis (The Oceanographer)
- **Stack**: Rust, `macroquad`, `cpal` (optional).
