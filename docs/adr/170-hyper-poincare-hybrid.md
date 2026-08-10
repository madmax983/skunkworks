# 170. Hyper-Poincare Hybrid

Date: 2026-08-10

## Status

Accepted

## Context

The Splice Surgeon created a new experimental hybrid called `hyper-poincare` (Hyperbolic 4D Rotation). This cross explores encoding 4D structural primitives (`crates/hyper-system`) directly into a continuous non-Euclidean boundary space (`crates/poincare-disk`).

## Decision

We project a rotating 4-dimensional hypercube down to 3D and then squash it into the non-Euclidean boundary of the Poincaré disk.

## Consequences

- **Positive:** Offers a compelling visualization of exponential compression as higher-dimensional coordinates map towards the edge of a finite geometric boundary.
- **Negative:** Complex projection mathematics can cause floating point precision loss near the edge of the disk.
