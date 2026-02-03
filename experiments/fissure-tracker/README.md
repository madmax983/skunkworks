# 🌋 Fissure Tracker

**Visualize the cracks in your codebase.**

`fissure-tracker` is a geological simulation of your project's structural integrity. It scans your source code for "stress points" (`unwrap`, `expect`, `panic`, `todo`) and visualizes them as fissures in the bedrock.

## Concept

Codebases are like tectonic plates. Every `unwrap()` is a stress fracture. Every `todo!()` is a pocket of unstable gas. When these accumulate, they form massive fault lines that threaten the stability of the system.

This tool uses a force-directed physics engine to map your files. Files with high "stress" (risk factors) generate jagged fissures that grow over time.

## Usage

```bash
cargo run -p fissure-tracker -- .
```

Or specify a path:

```bash
cargo run -p fissure-tracker -- ../my-other-project
```

## Controls

- **WASD / Arrows**: Pan the camera (Survey the land).
- **+ / -**: Zoom In / Out (Examine the cracks).
- **R**: Reset View.
- **Q / Esc**: Quit.

## Legend

- **Green Node**: Stable bedrock (Safe code).
- **Yellow Node**: Minor stress (Few unwraps).
- **Red Node**: Critical instability (Many panics/todos).
- **Red Lines**: Active fissures.
