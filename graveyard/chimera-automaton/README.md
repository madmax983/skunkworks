# Chimera Automaton 🧬🤖

**Lineage:** `cam-automaton` × `chimera-lang`

A biomechanical hybrid where a genetic program (Chimera DNA) drives a physical machine (Rapier2D physics vehicle).

## Concept

The **Chimera Automaton** replaces the static mechanical linkages of `cam-automaton` with a dynamic, programmable brain using the `ChimeraVM`. The organism's DNA writes values to specific memory locations (Grid cells) which are physically wired to the motor controllers of the vehicle's joints.

*   **Brain:** `ChimeraVM` (Genetic Logic)
*   **Body:** `rapier2d` (Physics Simulation)
*   **Visualization:** `macroquad` (Rendering)

## Interface

*   **Output (Brain -> Body):**
    *   `Grid[0][0]` -> Left Wheel Motor Velocity
    *   `Grid[0][1]` -> Right Wheel Motor Velocity

*   **Input (Body -> Brain):**
    *   (Planned) Sensors writing to Grid/Stack.

## Controls

The simulation runs automatically. The vehicle's behavior is determined by the hardcoded DNA in `main.rs`.

## Running

```bash
cargo run -p chimera-automaton
```
