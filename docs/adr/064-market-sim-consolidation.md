# 064. Market Sim Consolidation

Date: 2024-05-28

## Status

Accepted

## Context

The `market-sim` crate provides a Continuous Double Auction (CDA) implemented as a physics-based particle system. Originally extracted (ADR 026) to serve `market-flow`, `market-rogue`, `market-swarm`, and `chimera-market`.

A newer experiment, `thermo-market`, duplicated the entire `market-sim` logic simply to introduce a new particle type: a `Wall` that blocks movement. This created an immediate fork in the simulation engine, resulting in duplicated code and inconsistent features across market simulations.

## Decision

We have updated the `market-sim` crate to include the new `Wall` particle natively, eliminating the need for `thermo-market` to maintain a duplicated fork.

1.  **Added `Particle::Wall`:** The `Particle` enum in `market-sim` now supports a `Wall` variant.
2.  **Updated Grid Update Logic:** The `update` function handles the `Wall` particle by blocking Bid and Ask movement and forcing them sideways.
3.  **Refactored Experiments:** Refactored `thermo-market` to consume the shared `market-sim` crate. Updated `market-rogue` and `market-flow` to gracefully handle the new `Wall` variant if encountered.

## Consequences

### Positive
*   **Single Source of Truth:** Enforced a single, canonical implementation for market physics.
*   **Code Reduction:** Reduced codebase bloat by eliminating duplicated grid and physics logic in `thermo-market`.
*   **Enhanced Capability:** All current and future market experiments can now utilize `Wall` particles for structural simulation constraints.

### Negative
*   **Enum Expansion:** The `Particle` enum grows, potentially increasing its size and slightly affecting cache performance, though this is negligible for current workloads. Existing experiments needed minor updates to handle the new match arm.
