# Clockwork Concerto ⚛️⏱️🎵

**Genesis: The Horologist**

A "Moonshot" experiment combining **Verge Escapement** and **Music Box Cylinder**.
It simulates a mechanical CPU that plays procedural music, regulated by a physics-based escapement.

## Concept
*   **The Engine**: A massive 12-tooth Crown Wheel driven by torque (Arrow Keys).
*   **The Regulator**: A Verge Escapement (Anchor & Pendulum) that physically "ticks".
*   **The Brain**: A 4-register CPU that executes one step per tick.
*   **The Output**: A rotating Music Cylinder where instructions are pins. Execution triggers "Hammers" (Visual & Logged).

## Controls
*   **UP Arrow**: Wind the spring (Apply CW Torque). Clock runs faster.
*   **DOWN Arrow**: Brake / Reverse Torque.
*   **Console**: Watch the logs for "🎵 Play Note: ..."

## Implementation
*   **Physics**: `bevy_rapier2d` simulates the rigid body dynamics of the escapement.
*   **Logic**: `cpu.rs` implements the `Note` instruction.
*   **Visuals**: `bevy_prototype_lyon` renders the gears. `Gizmos` render the music cylinder.

## Why?
Because a computer is just a clock that counts logic instead of seconds. And if it counts, it can sing.
