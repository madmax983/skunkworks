# Verge Computer ⚛️⏱️

**Genesis: The Horologist**

A mechanical computer simulation where a Verge Escapement regulates a CPU clock cycle.
Built with Bevy and Bevy Rapier 2D.

## Concept
Smashing together:
*   **Verge Escapement**
*   **CPU Clock Cycle Visualization**

A crown wheel (Escape Wheel) is driven by a mainspring (Constant Torque).
An anchor escapement (approximated Verge) regulates its speed.
Each "tick" of the escapement triggers a CPU state transition (Fetch -> Decode -> Execute).

## Stack
*   `bevy`
*   `bevy_rapier2d`

## Running
```bash
cargo run -p verge-computer
```
