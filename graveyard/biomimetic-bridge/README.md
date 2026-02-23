# Biomimetic Bridge 🐜🌉

An emergent simulation of Army Ants forming bridges over gaps to allow the colony to cross.

## The Concept
In nature, army ants link their bodies together to span gaps that are too wide for a single ant to cross. This behavior is decentralized; no single ant decides to build a bridge. Instead, it emerges from simple local rules:
1.  **Crowding Rule**: If an ant encounters a gap and is pushed by many neighbors (high density), it freezes and becomes part of the terrain (a bridge).
2.  **Unbridging Rule**: If a bridge ant detects low traffic over itself (low density), it unfreezes and rejoins the foraging swarm.

This creates a dynamic structure that forms only when needed (high traffic) and dissolves when the colony has passed or found a better route, effectively acting as a biological load balancer.

## Controls
-   **Left Click**: Dig a Gap (Drag to paint void).
-   **Right Click**: Fill Gap (Create solid ground).
-   **Space**: Spawn 50 more ants at the mouse cursor.
-   **R**: Reset the simulation.

## How to Run
```bash
cargo run -p biomimetic-bridge
```

## Observations
-   Ants will naturally pile up at the edge of the gap.
-   Once density reaches a critical threshold (3+ neighbors), they start falling into the gap and freezing, extending the walkable area.
-   Eventually, a bridge reaches the other side.
-   If the flow of ants stops, the bridge "evaporates" from the tail end as ants leave.
