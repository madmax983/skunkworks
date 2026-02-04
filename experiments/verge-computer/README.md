# Verge Computer ⚛️⏱️

**Genesis: The Horologist**

A mechanical computer simulation where a **Verge Escapement** regulates a **CPU clock cycle**.
Built with Bevy, Bevy Rapier 2D, and Lyon.

## Concept
Smashing together:
*   **Verge Escapement** (An Anchor escapement in this 2D sim)
*   **CPU Clock Cycle Visualization**

A crown wheel (Escape Wheel) is driven by a mainspring (Constant Torque).
An anchor escapement locks and releases the wheel, creating a discrete "tick".
Each physical "tick" triggers a CPU micro-operation (Fetch -> Decode -> Execute).

The CPU currently executes a **Fibonacci Sequence** program stored in its memory.

## Mechanism
*   **Physics:** `bevy_rapier2d` simulates the rigid body dynamics of the gear train and escapement.
*   **Logic:** A custom VM (`cpu.rs`) interprets instructions (`LOAD`, `ADD`, `MOV`, `JMP`).
*   **Visuals:** `bevy_prototype_lyon` renders the "Brass" gear and "Steel" anchor.

## Running
```bash
cargo run -p verge-computer
```

## Program (Fibonacci)
The ROM contains:
```assembly
0: LOAD R0, 0
1: LOAD R1, 1
2: LOAD R2, 0
3: ADD R2, R0
4: ADD R2, R1
5: MOV R0, R1
6: MOV R1, R2
7: JMP 2
```

## Architecture
*   `mechanism.rs`: Physics body generation (Gears, Anchors).
*   `cpu.rs`: The virtual machine state and instruction set.
*   `view.rs`: Vector graphics rendering.
*   `lib.rs`: The plugin wiring it all together.
