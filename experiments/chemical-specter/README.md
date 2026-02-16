# Chemical Specter ⚛️👻

> "Chemistry is the music of matter." - Genesis

A reaction-diffusion simulation where the chemical parameters are driven by a virtual audio synthesizer, and the visual state could potentially drive the audio (feedback loop).

## Concept

This experiment combines **Gray-Scott Reaction Diffusion** with **Generative Audio**.
Instead of static parameters, the "Feed" and "Kill" rates of the reaction oscillate over time, driven by a "Ghost Synthesizer".
This causes the system to drift between different phase states:
- **Spots** (Mitosis)
- **Stripes** (Labyrinth)
- **Chaos** (Turbulence)
- **Void** (Death)

## Controls

- **Mouse Move**: Perturb the field (adds Chemical B).
- **Mouse Click**: Inject a massive dose of catalyst.

## Technical Details

- **Engine**: `macroquad` (Rust).
- **Simulation**: GLSL Fragment Shader running a convolution kernel (Laplacian) on a ping-pong texture buffer.
- **Audio**: Currently a "Ghost" simulation (LFOs) due to sandbox constraints, but designed to be hooked up to `rodio` or `cpal` for real audio reactivity.

## The Moonshot

To create a "Living Painting" that breathes and sings. The patterns are not drawn; they emerge from the mathematics of reaction and diffusion, guided by the rhythm of the code.
