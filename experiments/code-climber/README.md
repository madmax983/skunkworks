# Code Climber 🧗‍♂️⚛️

**Genesis: The Choreographer**

> "Climbing the syntax tree, literally."

This experiment combines **Inverse Kinematics** with **Code Navigation**.

## Concept
A procedural ragdoll climber ascends a wall made of its own source code.
The code is parsed into physics bodies (platforms).
The climber's limbs are driven by a simple 2-bone Inverse Kinematics solver (`ik.rs`) that targets a moving point or specific syntax tokens.

## Mechanics
- **Parser**: Reads `main.rs` and spawns `Collider`s for keywords, identifiers, and symbols.
- **Physics**: Uses `bevy_rapier2d` for rigid body simulation.
- **IK**: Analytic 2-bone IK solver using the Law of Cosines.
- **Control**: `brain.rs` drives the joint motors to reach procedural targets.

## Running
```bash
cargo run -p code-climber
```

## Future Ideas
- Make the climber actually grab and pull itself up.
- Use code complexity metrics to determine wall difficulty (friction, overhangs).
- multiplayer race to the top of the file.
