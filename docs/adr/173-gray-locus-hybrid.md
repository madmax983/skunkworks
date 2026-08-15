# 173. Gray-Locus Hybrid

Date: 2026-08-10

## Status

Accepted

## Context

The Splice Surgeon created the experimental hybrid `gray-locus` (Topological Reaction-Diffusion). This cross explores mapping a continuous reaction-diffusion simulation (`crates/gray-scott`) onto a non-Euclidean topological space (`crates/locus`).

## Decision

We map a chemical seed drifting through space onto a topological boundary (e.g., Klein Bottle). As it moves, it continuously drops the `V` chemical into the `gray-scott` morphogenetic substrate. The chemical interactions wrap around the topological boundaries.

## Consequences

- **Positive:** Creates an emergent biological visualization where growing Turing patterns form organic highways that reflect the topological path of the seed, demonstrating biological computation seeded via non-Euclidean movement.
- **Negative:** Continuous diffusion across non-Euclidean seams may cause unpredictable localized instability or pattern tearing if diffusion rates are improperly tuned.
