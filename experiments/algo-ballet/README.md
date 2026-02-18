# Algo Ballet ⚛️💃

**Genesis: The Choreographer**

> *The array is the stage. The elements are the dancers. The algorithm is the choreography.*

This experiment combines **Ballet Notation (Laban Movement Analysis)** with **Data Traversal Visualization**.
It visualizes sorting algorithms not just as swapping bars, but as a procedural dance performed by a corps de ballet.

## Concept
Each array element is a dancer with procedural limbs.
The Sorting Algorithm (Director) issues instructions to the dancers.
These instructions are interpreted through **Laban Effort** parameters:

- **Comparison**: `Weight: Light`, `Time: Sustained`. (Careful observation, gentle gestures).
- **Swap**: `Weight: Strong`, `Time: Sudden`. (Decisive, powerful leaps).

The motion is generated procedurally using inverse kinematics and interpolation curves modulated by these effort parameters.

## Tech Stack
- **Bevy 0.14**: ECS and App structure.
- **Bevy Prototype Lyon**: Vector graphics for the dancers.
- **Custom Kinematics**: Simple interpolation driven by Laban parameters.

## Running
```bash
cargo run -p algo-ballet
```

Watch the Bubble Sort unfold as a dance.
