# 145. Arthropod-Lattice Hybrid

Date: 2026-07-06

## Status

Proposed

## Context

The Splice Surgeon created a new experimental hybrid called `arthropod-lattice` (Interactive Codebase Crystallography). This cross explores mapping by crossing the discrete components of `crates/arthropod` with `crates/miller-lattice`.

## Decision

Cross the immediate mode UI elements of `arthropod` directly with the procedural generation of `miller-lattice`. This allows discrete button clicks to spawn new hierarchical file systems in continuous 3D space.

## Consequences

- **Positive:** Enables an interactive structural playground for viewing continuous rendering of crystalline structures.
- **Negative:** Abstract GUI directly influences the growth parameters of the environment, improving real-time experimentation.
