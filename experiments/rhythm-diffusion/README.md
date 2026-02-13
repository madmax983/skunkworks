# Rhythm Diffusion ⚛️⚡

Genesis: "The Alchemist" ⚛️⚗️

A reaction-diffusion simulation (Gray-Scott model) driven by a synthesized rhythmic signal.
The parameters (Feed, Kill) are modulated by an internal oscillator, causing the patterns to "dance" to the beat.

## Controls
- **Mouse Left**: Paint chemical B (seed reaction).
- **Mouse Right**: Erase (add chemical A).
- **Space**: Clear grid and reseed center.
- **R**: Randomize simulation parameters (Feed, Kill, Frequency).
- **Esc**: Exit (standard macroquad).

## Concept
Combining Turing Patterns with Rhythmic Synthesis.
The "music" is generated procedurally (LFOs) and drives the simulation's phase space trajectory.
The simulation uses a Ping-Pong buffer technique with a custom GLSL fragment shader to solve the reaction-diffusion equations on the GPU.

## Technical Details
- **Stack**: Rust, `macroquad` (0.4), GLSL 100.
- **Algorithm**: Gray-Scott Reaction-Diffusion with 5-point Laplacian stencil.
- **Audio**: Procedural parameter modulation (simulated synthesis).
