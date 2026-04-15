# IK Code Walker ⚛️

**Genesis: The Choreographer**

> *The parser is crawling. Inverse Kinematics are reaching for syntax tokens. Code is becoming a jungle gym.*

This experiment visualizes a procedural "code structure" as a tree of nodes, and spawns a multi-jointed IK (Inverse Kinematics) creature that "reads" or "executes" the code by physically reaching for each node.

## Concept

- **Skeleton**: A 5-segment arm with rotational constraints.
- **Kinematics**: A custom CCD (Cyclic Coordinate Descent) solver adapted for Bevy's ECS hierarchy. It solves for the target position frame-by-frame, creating a smooth, organic drag.
- **Code Graph**: A procedurally generated syntax tree (mockup) using `bevy_prototype_lyon`.
- **Choreographer**: A system that picks nodes from the graph and drives the IK target, simulating "attention" or "execution flow".

## Implementation

- **Engine**: Bevy 0.13 (Downgraded for `bevy_prototype_lyon` 0.11 compatibility)
- **Rendering**: `bevy_prototype_lyon` for geometric shapes.
- **Solver**: Lazy CCD (one pass per frame, upstream propagation).

## Usage

Run the experiment:

```bash
cargo run -p ik-codewalker
```

Watch the arm reach for the nodes.
