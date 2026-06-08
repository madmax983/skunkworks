# 110. Miller-Platter Hybrid Experiment

Date: 2026-06-08

## Status
Proposed

## Context
The repository contains numerous experimental prototypes designed to crossbreed biological or physical simulations with digital data structures. The recent execution of the `miller-platter` hybrid by the Splice Surgeon successfully crossed the rigid 3D directory crystal generator (`miller-lattice`) with a 2D continuous scalar thermodynamic field (`platter`). However, this architectural mutation was not formally recorded in the system architecture diagrams, obscuring its lineage and structural mechanism.

## Decision
Document the `miller-platter` hybrid experiment architecture. A new ADR records the creation of this mutation, and the codebase architecture diagrams are updated to reflect the cross. `miller-platter` operates by taking the 3D depth map of the `miller-lattice` crystal and projecting it onto the 2D `platter` grid, where the structural depth generates heat that diffuses as a thermodynamic shadow.

## Consequences
* **Positive:** The "Bio-Digital Isomorphism" pattern is formally documented, improving discoverability of how continuous physical simulations (platter) can map discrete digital hierarchies (miller-lattice).
* **Negative:** Further expands the `docs/architecture.md` map and ADR registry, requiring ongoing maintenance if the hybrid mutates further.
