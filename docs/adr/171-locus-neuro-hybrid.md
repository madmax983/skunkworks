# 171. Locus-Neuro Hybrid

Date: 2026-08-10

## Status

Accepted

## Context

The Splice Surgeon created a new experimental hybrid called `locus-neuro` (Topological Brain Simulation). This cross explores mapping a biological Spiking Neural Network (`crates/neuro-sim`) onto a continuous topological space (`crates/locus`).

## Decision

We map individual neurons onto a 2D grid wrapped in a continuous topology (e.g., Torus or Klein Bottle). Action potentials propagating out of one boundary wrap around to stimulate neurons on the opposite side.

## Consequences

- **Positive:** Simulates biologically plausible neural feedback loops that continuously loop and intersect with themselves across a non-Euclidean space.
- **Negative:** High density of topological edge wraparounds may cause unintended global seizures within the simulated neural network.
