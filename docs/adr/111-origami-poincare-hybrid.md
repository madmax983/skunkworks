# 111. Origami-Poincare Hybrid Experiment

Date: 2026-06-09

## Status
Proposed

## Context
The repository contains numerous experimental prototypes. The recent execution of the `origami-poincare` hybrid by the Splice Surgeon successfully crossed the procedural Miura-ori soft-body mesh generator (`origami`) with a continuous non-Euclidean hyperbolic space (`poincare-disk`). However, this architectural mutation was not formally recorded in the system architecture diagrams.

## Decision
Document the `origami-poincare` hybrid experiment architecture. A new ADR records the creation of this mutation, and the codebase architecture diagrams are updated to reflect the cross. `origami-poincare` demonstrates "Hyperbolic Soft-Body Morphogenesis" by mapping the continuous, physical 3D vertices of a Miura-ori paper mesh directly onto the non-Euclidean space of the Poincaré disk using Mobius transformations.

## Consequences
* **Positive:** The "Hyperbolic Soft-Body Morphogenesis" pattern is formally documented, capturing how physical material stress can interact with infinite mathematical bounds.
* **Negative:** Further expands the `docs/architecture.md` map and ADR registry.
