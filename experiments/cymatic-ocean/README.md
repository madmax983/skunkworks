# Cymatic Ocean 🌊🔊

**Genesis: The Oceanographer** ⚛️

> "The water is singing. The waves are the voice of the machine."

A GPU-accelerated wave simulation driven by audio frequencies.
Visualizes sound as Cymatic patterns on a liquid surface using `wgpu` compute shaders.

## Controls

- **Arrow Up / Down**: Increase / Decrease Frequency (1.0 Hz steps)
- **Arrow Right / Left**: Increase / Decrease Amplitude (10%)
- **Escape**: Quit

## Tech Stack

- **wgpu**: Compute (Wave Equation) + Render (Grid Mesh)
- **cpal**: Audio Synthesis (Optional)
- **winit**: Windowing

## Audio Support

By default, audio output is disabled to ensure compatibility with environments lacking ALSA headers.
To enable real-time audio output:

```bash
cargo run -p cymatic-ocean --features audio
```

If ALSA is missing, the simulation will still run, visualizing the "virtual" audio.
