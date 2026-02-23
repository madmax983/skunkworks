# 51. Catalog System Visualization Experiments

Date: 2024-05-23

## Status

Proposed

## Context

The repository contains a rich set of experiments that visualize real-time system metrics or simulate complex bio-digital feedback loops. Specifically:

1.  **Process Canopy (`process-canopy`)**: Maps OS processes to L-System trees, where CPU usage drives growth and scheduling affects sunlight exposure.
2.  **Sys Dance (`sys-dance`)**: Maps system performance metrics (RAM, CPU, Swap) to Labanotation parameters to drive a procedural dancer's IK rig.
3.  **Ferrous Genesis (`ferrous-genesis`)**: Implements an Amorphous Cellular Automaton where particles run `chimera-lang` bytecode to modulate their magnetism, creating self-organizing structures.

However, these experiments are currently undocumented in `docs/architecture.md`. This omission hinders the discoverability of these projects and obscures the reusable patterns they establish, particularly regarding `sysinfo` integration and `chimera-lang` embedding.

## Decision

We will formally catalog these experiments in `docs/architecture.md` under a new or existing category to highlight the pattern of "Bio-Digital Isomorphism" (the mapping of digital state to biological or physical forms).

The documentation will include:
-   **Class Diagrams** for `process-canopy` and `sys-dance` to illustrate the data flow from system monitors to procedural generators.
-   **Sequence Diagrams** for `ferrous-genesis` to illustrate the feedback loop between the Chimera VM and the magnetic field.

## Consequences

### Positive
-   **Discoverability**: Developers exploring the repo can easily find and understand these experiments.
-   **Pattern Recognition**: Establishing the "Monitor -> Model -> Render" pattern for system visualization encourages consistent implementation in future experiments.
-   **Chimera Usage**: Documenting `ferrous-genesis` provides a concrete example of embedding `chimera-lang` in a physics simulation.

### Negative
-   **Maintenance**: As these are experimental projects, their architecture may change rapidly, requiring updates to the central documentation map.
