# 144. Arthropod-Physics Hybrid

Date: 2026-07-06

## Status

Accepted

## Context

The Splice Surgeon created a new experimental hybrid called `arthropod-physics` (Interactive Structural Rigging). This cross explores mapping by crossing the discrete components of `crates/arthropod` with `crates/physics-pbd`.

## Decision

Cross the immediate mode UI library `arthropod` with the Position Based Dynamics engine `physics-pbd`. The discrete button clicks interact directly with the physical constraints, exploding the continuous particle physics or adding dynamically connected structural chain elements.

## Consequences

- **Positive:** Translates discrete GUI interactions into continuous physical constraints, providing a novel way to interact with soft-body simulations.
- **Negative:** Increases visual feedback but may couple UI timing with physics timestep stability.
