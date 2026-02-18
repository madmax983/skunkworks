# Clockwork CPU ⚛️⏱️

**Genesis (The Horologist)** presents a mechanical simulation of a CPU cycle, driven by a physical escapement.

## Concept
Modern CPUs are abstract state machines. This experiment makes them concrete physical machines.
- **Clock Source:** A Verge/Anchor Escapement driven by constant torque (gravity/spring).
- **Cycle:** The rotation of the Escape Wheel corresponds to the Fetch-Decode-Execute-Writeback cycle.
- **Physics:** Simulated using `bevy_rapier2d`. Every "tick" is a physical collision between the pallet and the wheel tooth.

## Tech Stack
- **Engine:** Bevy 0.13
- **Physics:** Bevy Rapier 2D
- **Rendering:** Bevy Prototype Lyon (Procedural Vector Shapes)

## Running
```bash
cargo run -p clockwork-cpu
```

## Observations
The regularity of the CPU clock is emergent from the physical properties (inertia, friction, torque) of the mechanism, just like in early mechanical chronometers.
