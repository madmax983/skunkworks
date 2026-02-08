# Syntax Resonance ⚛️🎼

**"The Codebase is a Resonant Body"**

A cross-sensory experiment that sonifies the structure of source code by treating it as a physical space where sound waves propagate.

## Concept

Imagine your code as a physical architecture.
- **Files** are regions in a vast echo chamber.
- **Structs & Enums** are solid walls and pillars that reflect sound.
- **Functions** are energy sources that "pluck" the fabric of space when executed.
- **Loops** are standing waves, oscillating with intensity.

As the "execution" (a parser) walks through your code, it excites the grid. The resulting sound is the natural resonance of your software architecture. Complex, spaghetti code might sound chaotic and noisy. Clean, modular code might sound harmonious and rhythmic.

## Visuals

The visualization shows a 2D wave simulation (FDTD).
- **Red/Blue waves**: High/Low pressure zones.
- **White blocks**: Structs/Enums (Walls).
- **Yellow text**: Current code construct being "played".

## Usage

```bash
cargo run -p syntax-resonance --features audio -- <path-to-repo>
```

If no path is provided, it plays its own source code.

## Controls

The simulation runs automatically. Sit back and listen to the song of your code.

## Technical Details

- **Parser**: `syn` + `walkdir` traverses the AST.
- **Audio**: `resonance-audio` (FDTD Wave Solver) + `cpal`.
- **Visuals**: `macroquad`.
