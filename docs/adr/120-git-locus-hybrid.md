# 120. Git-Locus Hybrid

Date: 2026-06-16

## Status

Proposed

## Context

The experimental `git-locus` hybrid aims to map historical codebase activity onto continuous topological spaces. We crossed `crates/git-associates` (which parses discrete Git commit metadata, measuring churn and developer intent over time) with `crates/locus` (which provides continuous non-Euclidean boundary wrapping such as Torus, Klein Bottle, and Mobius surfaces).

## Decision

We project the chronological commit history onto a continuous 2D boundary grid governed by topological rules. As codebase activity expands past visual Euclidean boundaries, the history naturally wraps and intersects itself along the chosen topological surface.

## Consequences

- **Positive:** Creates overlapping "ghost timelines" of the repository's evolution. This allows observers to see how deeply coupled files interact with each other even when separated by immense linear time, yielding novel insights into long-term codebase structural loops.
- **Negative:** Projecting linear time onto wrapping topologies requires complex spatial mappings. It can be difficult for a user to distinguish between spatial proximity caused by recent temporal coupling versus proximity caused by topological boundary wrapping.
