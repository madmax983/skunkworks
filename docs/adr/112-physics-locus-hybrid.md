# 112. Physics-Locus Hybrid Experiment

Date: 2026-06-09

## Status
Accepted

## Context
The repository contains numerous experimental prototypes. The recent execution of the `physics-locus` hybrid by the Splice Surgeon successfully crossed rigid body dynamics (`physics-pbd`) with topological boundaries (`locus`). However, this architectural mutation was not formally recorded in the system architecture diagrams.

## Decision
Document the `physics-locus` hybrid experiment architecture. A new ADR records the creation of this mutation, and the codebase architecture diagrams are updated to reflect the cross. `physics-locus` demonstrates "Topological Soft-Body Physics" by projecting the rigid structural constraints and kinetic velocity of Position Based Dynamics particles directly onto continuous non-Euclidean boundary wrapping (e.g., Torus, Klein Bottle).

## Consequences
* **Positive:** The "Topological Soft-Body Physics" pattern is formally documented, showing how rigid simulations can be mapped to continuous topological boundary wrapping.
* **Negative:** Further expands the `docs/architecture.md` map and ADR registry.
