# 175. Neuro-Poincare Hybrid

Date: 2026-08-18

## Status

Accepted

## Context

The Splice Surgeon created a new experimental hybrid called `neuro-poincare` (Hyperbolic Neural Networks). This cross explores mapping a Spiking Neural Network (SNN) into a continuous non-Euclidean topology by crossing `crates/neuro-sim` with `crates/poincare-disk`.

## Decision

We map biological firing sequences onto a Poincaré disk. Synapses that span across the disk or towards the boundary experience massive transmission delays proportional to the hyperbolic distance, mimicking relativistic spatial warping.

## Consequences

- **Positive:** Demonstrates the viability of mapping a biological neural network into non-Euclidean space. Generates a temporal warping effect on biological brain waves where massive synaptic transmission delays occur near the edge while the center fires rapidly.
- **Negative:** The temporal warping might cause instability or require specific balancing of excitation/inhibition and synaptic delays depending on the size and scope of the network.
