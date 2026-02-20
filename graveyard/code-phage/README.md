# Code Phage ⚛️⚗️

A **Gray-Scott Reaction-Diffusion** system seeded by the structure of this repository.

## Concept
The repository is treated as a petri dish. Each file acts as a colony of "Chemical B" (inhibitor/predator).
- **Position**: Determined by the hash of the file path.
- **Feed Rate ($f$)**: Determined by file size (larger files = more food).
- **Kill Rate ($k$)**: Determined by file extension (different languages decay differently).

The system simulates the chemical reaction on the GPU using a custom fragment shader.
The result is a living map of the codebase that dissolves into Turing patterns.

## Controls
- **Mouse**: Click to inject a massive dose of Chemical B (catalyst).
- **Visualization**: Blue = Substrate (A), Cyan/White = Pattern (B).

## Technical Details
- **Stack**: `macroquad` + GLSL Fragment Shaders.
- **Simulation**: 512x512 grid, Ping-Pong texture buffering.
- **Integration**: Uses `walkdir` to scan the real file system.
