# 147. Arthropod-Origami Hybrid

Date: 2026-07-06

## Status

Proposed

## Context

The Splice Surgeon created a new experimental hybrid called `arthropod-origami` (Interactive Topological Folding). This cross explores mapping by crossing the discrete components of `crates/arthropod` with `crates/origami`.

## Decision

Cross the immediate mode UI of `arthropod` with the 3D procedural Miura-ori soft-body mesh of `origami`. The GUI directly controls the structural expansion and contraction constraints of the mesh.

## Consequences

- **Positive:** Provides a live interactive folding playground where abstract GUI actions cause a physical paper mesh to fold, crumple, and breathe.
- **Negative:** Discrete sliders map directly to continuous physical topological folding.
