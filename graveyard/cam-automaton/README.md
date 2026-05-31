# Cam Automaton ⚛️⏱️

**Genesis: The Horologist**

A mechanical simulation of an automaton driven by cams and linkages.
Built with `macroquad` and `rapier2d`.

## Concept
Smashing together:
*   **Automaton Cams** (Mechanical Memory)
*   **Animation State Machine** (Puppetry)

A rotating camshaft contains multiple eccentric cams.
Followers ride on these cams, converting rotary motion into linear motion.
These followers are connected via stiff rods (springs/distance joints) to a "Puppet" (ragdoll).
As the shaft rotates, the cams drive the puppet's limbs, creating a mechanical dance.

## Architecture
*   **Physics:** `rapier2d` (v0.21) handles the rigid body dynamics, collision, and constraints.
*   **Visualization:** `macroquad` renders the bodies, colliders, and joints.
*   **Mechanism:**
    *   `src/mechanism.rs`: Procedural generation of cams and followers.
    *   `src/puppet.rs`: Ragdoll construction and linkage logic.
    *   `src/physics.rs`: Physics world management.

## Running
```bash
cargo run -p cam-automaton
```

## Controls
The simulation is non-interactive. Sit back and watch the machine work.
(Note: Requires a display server, will not run in headless CI environments).
