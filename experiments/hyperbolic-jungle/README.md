# 🌴 Hyperbolic Jungle 🥁

**Lineage:** `hyperbolic-mold` × `rhythmic-jungle`

A visualization of rhythmic synchronization agents inhabiting the Poincaré Disk model of hyperbolic geometry.

## Concept

Agents move through a negatively curved space (Hyperbolic Plane) where space expands exponentially as you move away from the center. Each agent carries a "Euclidean Rhythm" (a pattern of pulses distributed as evenly as possible).

When agents encounter each other (based on Hyperbolic Distance), they attempt to synchronize their BPM and pulse density.

## The Experiment

*   **Traits from `hyperbolic-mold`**:
    *   Movement on the Poincaré Disk.
    *   Möbius transformations for position updates.
    *   Boundary handling (infinite horizon).

*   **Traits from `rhythmic-jungle`**:
    *   Euclidean Rhythm generation (Bjorklund's algorithm).
    *   Agent interaction logic (BPM convergence, pulse mutation).
    *   Visual representation of rhythm density (Color).

*   **Emergent Behavior**:
    *   **Relativistic Rhythms**: Due to hyperbolic geometry, "local" clusters form differently than in Euclidean space. An agent near the edge has just as much room as one in the center, but appears compressed to the observer.
    *   **Infinite Synchronization**: Can the entire disk synchronize when the number of agents needed to cover a ring grows exponentially with radius?

## Controls

*   **Run**: `cargo run -p hyperbolic-jungle`

## Status

Compiles. Verified by The Splice Surgeon.
