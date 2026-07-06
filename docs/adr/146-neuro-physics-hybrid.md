# 146. Neuro-Physics Hybrid

Date: 2026-07-06

## Status

Proposed

## Context

The Splice Surgeon created a new experimental hybrid called `neuro-physics` (Neural Muscle Contraction). This cross explores mapping by crossing the discrete components of `crates/neuro-sim` with `crates/physics-pbd`.

## Decision

Cross the biological Spiking Neural Network of `neuro-sim` with the soft-body Position Based Dynamics of `physics-pbd`. Spiking Neural Network (SNN) activations drive the physical tension and expansion constraints of soft-body PBD chains.

## Consequences

- **Positive:** Creates an organic, bio-mechanical simulation where chaotic neural 'thoughts' visually actuate muscle tissue across a hanging soft-body grid.
- **Negative:** Links neural activity to physical structure, which may cause chaotic physical instability.
