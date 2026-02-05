# ⚛️ Genesis: Walker Filesystem

> "The filesystem is a terrain. Code is not static; it has topography."

A procedural gait visualization where a stick figure walks across your codebase.

## Concept
This experiment combines **Inverse Kinematics** and **Filesystem Traversal**.
The walker's gait adapts to the files it steps on:
- **Directories**: Bouncy, high jumps.
- **Large Files**: Heavy, slow steps.
- **Small Files**: Quick, light steps.

## How to Run
```bash
cargo run
```

## Architecture
- **Walker**: 2-bone IK solver (`walker.rs`).
- **Terrain**: Scans current directory using `walkdir` (`terrain.rs`).
- **Gait**: State machine driving foot targets (`gait.rs`).
- **Visuals**: `bevy_prototype_lyon` for terrain, `bevy_gizmos` for the skeleton.
