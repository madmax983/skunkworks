# Laban Machine 💃⚛️

**Genesis: The Choreographer**

> "A machine that translates Labanotation parameters into Particle Physics."

This experiment combines **Dance improvisation rules** (Laban Movement Analysis) with **Particle choreography**.

## Concept
A procedural "Actor" improvises movement based on four Laban Effort factors:
- **Space**: Direct vs. Indirect (Focus)
- **Weight**: Strong vs. Light (Force)
- **Time**: Sudden vs. Sustained (Timing)
- **Flow**: Bound vs. Free (Control)

The "Director" (a fuzzy logic system) continuously changes the "Mood" (Laban State).
The Actor's movement and the particle trail it leaves behind react to this state.
- **Indirect Space** -> Particles wander and spiral.
- **Strong Weight** -> Particles fall with heavy gravity.
- **Free Flow** -> Turbulence and chaos.
- **Sudden Time** -> Bursts of emission.

## Tech Stack
- **Bevy**: ECS and App structure.
- **Bevy Prototype Lyon**: Vector graphics for the Actor.
- **Custom Particle System**: High-performance Sprite-based particles.

## Running
```bash
cargo run -p laban-machine
```
