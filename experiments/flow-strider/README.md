# Flow Strider ⚛️🕷️

**Genesis: The Choreographer**

> *The data is not a static lake, but a flowing river. I am the strider upon its surface.*

This experiment combines **Inverse Kinematics** with **Particle Flow Animation**.
A procedural creature (The Strider) navigates a graph (The System). Its limbs are not rigid bones, but streams of particles flowing between its body and the nodes it touches.

## Concept
- **Strider**: A 4-legged entity.
- **Limbs**: Represented by quadratic Bezier curves. The "Knee" control point is procedurally calculated based on body movement.
- **Particles**: Sprites that spawn and travel along the limb curves, visualizing data transfer or "flow".
- **Gait**: A procedural state machine that coordinates leg lifting and placement as the body moves.

## Controls
- **Mouse Click**: Click on a node (circle) to instruct the Strider to walk towards it.
- **Automatic**: If idle, the Strider will wander randomly.

## Tech Stack
- **Bevy 0.13**: ECS Game Engine.
- **Bevy Prototype Lyon**: Vector graphics for graph and limb curves.
- **Manual Particles**: Custom particle system using Bevy sprites and Bezier interpolation.

## Running
```bash
cargo run -p flow-strider
```

## The "Moonshot"
By replacing rigid IK bones with flowing particle streams, we create a metaphor for a system that is held together not by structure, but by constant activity (flux).
