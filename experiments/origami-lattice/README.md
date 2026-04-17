# Origami Lattice

A hybrid biological and physical visualization engine crossing `miller-lattice` with `origami`.

## Lineage

* **miller-lattice:** Provides the rigid, crystalline 3D hierarchical structure mapping of a codebase. Nodes represent files and directories built deterministically from the root path.
* **origami:** Provides the continuous, procedural 3D Miura-ori soft body mesh. Uses Position Based Dynamics to simulate a soft sheet of paper.

## Novel Trait

**Structural Codebase Morphogenesis.** We take the rigid, discrete lattice of the codebase and project it down onto a continuous soft-body mesh.

Directories pull *up* on the fabric of the codebase, while files pull *down*. As the hierarchical weight of the repository strains the mesh, it dynamically buckles, crumples, and breathes. Instead of viewing a codebase as a static graph, we visualize the *physical tension* of the repository's structure dynamically warping a topological space.
