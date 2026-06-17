# 122. Gray-Physics Hybrid

Date: 2026-06-17

## Status

Proposed

## Context

The Splice Surgeon created a new experimental hybrid called `gray-physics`, exploring morphogenetic soft-body deformation. This experiment crosses the continuous chemical reaction-diffusion simulation (Turing patterns) from `crates/gray-scott` with the soft body mechanics and constraints from `crates/physics-pbd` (Position Based Dynamics).

## Decision

We map the continuous chemical Turing patterns to actively control the stiffness and constraints of the PBD soft-body simulation. A high concentration of the "kill" chemical physically alters the structural distance constraints, creating structural weaknesses or inducing movements directly driven by the chemical reaction.

## Consequences

- **Positive:** Enables the simulation of organic biological morphogenesis, where chemical gradients dictate physical form and movement. It creates dynamic, self-altering soft bodies governed by emergent chemical patterns.
- **Negative:** Evaluating chemical concentration maps and updating hundreds or thousands of PBD distance constraints per frame introduces significant computational overhead, making real-time simulation challenging for large meshes.
