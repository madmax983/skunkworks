# Syntax Spider ⚛️🕷️

**"The code is a web, and I am the weaver."**

This experiment visualizes the codebase as a radial graph and features a procedurally animated spider that traverses it.
It combines **Inverse Kinematics (IK)** with **Code Navigation**.

## Concept
The project implements a 2-bone IK solver to control an 8-legged spider.
The spider's body is physically simulated using `bevy_rapier2d`.
The legs are procedurally animated: they find "foot holds" on the graph nodes or empty space, and the IK solver positions the joints to make the movement look organic.

## Controls
- **WASD**: Apply force to the spider body to move it.
- **Arrow Keys**: Pan the camera (though it follows the spider by default).
- **Z / X**: Zoom in/out.

## Tech Stack
- **Bevy**: Game Engine (ECS).
- **Bevy Rapier 2D**: Physics.
- **Bevy Prototype Lyon**: Vector Graphics (for drawing legs and nodes).

## Running
```bash
cargo run -p syntax-spider
```

## Implementation Details
- **IK Solver**: Uses Law of Cosines to solve 2-bone chains in 2D.
- **Gait Controller**: Predicts future body position based on velocity and triggers leg steps when the current foot position is too far or "uncomfortable". It ensures not all legs step at once (randomized cooldowns).
- **Graph Layout**: Scans the current directory and arranges files/folders in a simple radial layout.

## Future Ideas
- **Web Swing**: Shoot a web to swing between nodes.
- **Parsing**: Color nodes based on file type or syntax errors.
- **Predators**: "Bug" entities that eat code, which the spider must hunt.
