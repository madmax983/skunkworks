# 175. Neuro-Poincare Hybrid

Date: 2026-08-17

## Status

Accepted

## Context

The Splice Surgeon created a new experimental hybrid called `neuro-poincare` (Hyperbolic Neural Networks). This cross explores mapping a biological Spiking Neural Network (`crates/neuro-sim`) into a non-Euclidean boundary space (`crates/poincare-disk`).

## Decision

We map biological neural firing sequences and synaptic connections onto the Poincaré disk. Synaptic delays are proportionally warped by the hyperbolic distance constraints between neurons, mimicking relativistic spatial warping as signals propagate towards the boundary.

## Consequences

- **Positive:** Creates an emergent bio-neural visualization demonstrating temporal warping, where massive synaptic transmission delays occur near the edge while the center fires rapidly.
- **Negative:** Evaluating exact hyperbolic distances between fully connected neurons creates significant computational overhead and complexity.
