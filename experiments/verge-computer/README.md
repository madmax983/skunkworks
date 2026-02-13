# ⚛️ Verge Computer

> "The clockwork that computes."

This experiment simulates a mechanical computer driven by a **Graham Escapement**.
A physics-based **Escape Wheel** (30 teeth) is driven by a **Mainspring** (torque) and regulated by an **Anchor** (pendulum).
Each "tick" of the escapement drives a single clock cycle of a simple **CPU** executing a Fibonacci sequence.

## Mechanism
- **Physics Engine**: `bevy_rapier2d`.
- **Rendering**: `bevy_prototype_lyon`.
- **Escapement**: Graham/Deadbeat style with tangent pallets.
- **CPU**: A simple register machine (4 registers) with `LOAD`, `ADD`, `MOV`, `JMP` instructions.

## How to Run
```bash
cargo run
```
You should see:
- A brass Escape Wheel turning clockwise.
- A steel Anchor rocking back and forth.
- A green indicator flashing on each tick.
- Text overlay showing the CPU state (PC, Registers).

## Concept
Combining horology and computing:
- **Tick**: One escapement cycle = One CPU clock edge.
- **Regulation**: The pendulum period determines the CPU clock speed.
- **Power**: The mainspring provides the energy for computation.

## Status
- [x] Physics Simulation (Escapement works!)
- [x] Visuals (Gears and Anchor rendered with Lyon)
- [x] CPU Logic (Fibonacci program running)
