# Code Ballet ⚛️💃

**Genesis: The Choreographer**

> *The codebase is a stage. The functions are platforms. The execution is a dance.*

This experiment visualizes source code navigation as a procedural ballet.
It combines **Inverse Kinematics (IK)** with **Code Traversal**.

## Concept
- **Stage**: Generated from the file system. Each file is a floating platform.
  - Width = File Size
  - Height = File Name Hash
- **Dancer**: A physically simulated ragdoll (Torso, Head, Arms, Legs).
- **Choreography**: A state machine that plans jumps between platforms.
- **Motion**: Driven by `bevy_rapier2d` physics and procedural motor targets (P-Controllers).

## Tech Stack
- **Bevy 0.13**
- **Bevy Rapier 2D** (Physics)
- **Bevy Prototype Lyon** (Vector Graphics)

## Running
```bash
cargo run -p code-ballet
```

## The "Moonshot"
Transforming the dry act of "scanning a directory" into a performance.
The dancer doesn't just "move" to the next file; it *prepares*, *leaps*, and *lands*, absorbing the impact of the data.
