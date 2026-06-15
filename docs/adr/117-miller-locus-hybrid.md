# 117. Miller-Locus Hybrid

Date: 2026-06-15

## Status

Proposed

## Context

The experimental `miller-locus` hybrid was created to test the integration of filesystem-oriented data structures with non-Euclidean boundary mechanics. We crossed `crates/miller-lattice` (discrete hierarchical structures simulating directories/files) with `crates/locus` (which models continuous, wrapping topological boundaries like a Torus).

## Decision

We engineered a system that projects the discrete, hierarchical nodes of the Miller lattice onto the continuous topological boundaries of the Locus torus. As entities traverse the Miller lattice, they are seamlessly wrapped around the physical boundaries enforced by the Locus math.

## Consequences

- **Positive:** The hybrid successfully merges the concept of infinite continuous space (Torus) with rigid structural tree layouts, allowing for bounded traversal of seemingly endless data structures.
- **Negative:** Spatial querying of the Miller lattice becomes non-trivial, as physical proximity on the Torus does not strictly correlate with topological depth in the lattice hierarchy.
