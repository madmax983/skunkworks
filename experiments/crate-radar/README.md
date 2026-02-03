# Crate Radar 📡

**Parents**: `experiments/cargo-compass` + `experiments/search-sonar`

Visualizes the dependency graph of the current crate as a radar screen.

## Concept

Navigate your dependencies like a submarine captain. The current crate is the center of the universe. Dependencies are blips on the radar.

- **Angle**: Deterministic hash of the package name.
- **Radius**: Deterministic hash (for now).
- **Scanning**: The radar sweep highlights dependencies as it passes over them, revealing details in the side panel.

## Usage

```bash
cargo run -p crate-radar
```

Run this inside a rust project (it looks for `Cargo.toml`).

## Lineage

- **From `cargo-compass`**: Metadata extraction logic using `cargo_metadata`.
- **From `search-sonar`**: The radar visualization engine (Canvas, Sweep line, Blip rendering).
- **Novel Trait**: Mapping abstract dependency graph data to polar coordinates for visualization.
