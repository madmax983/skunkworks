# 172. Neuro-Origami Hybrid

Date: 2026-08-10

## Status

Accepted

## Context

The Splice Surgeon resurrected the experimental hybrid `neuro-origami` (Neural Morphogenesis). This cross maps a Spiking Neural Network (`crates/neuro-sim`) onto a procedural Miura-ori soft-body mesh (`crates/origami`).

## Decision

We map neural action potentials (spikes) directly to the structural extension factor of the Miura-ori mesh, causing the geometry to physically deform in response to cognitive loads.

## Consequences

- **Positive:** Yields a real-time structural visualization where biological neural activity translates into physical architectural deformation.
- **Negative:** Fast firing rates can cause the mesh geometry to tear or clip if damping parameters are not tuned carefully.
