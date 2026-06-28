# 119. Git-Poincare Hybrid

Date: 2026-06-16

## Status

Accepted

## Context

The experimental `git-poincare` hybrid aims to map chronological codebase history directly into a continuous non-Euclidean hyperbolic space. We crossed `crates/git-associates` (which parses discrete Git commit metadata, measuring churn and developer intent over time) with `crates/poincare-disk` (which provides continuous non-Euclidean hyperbolic boundary mapping).

## Decision

We project the chronological commit history onto a continuous 2D hyperbolic space (the Poincaré disk). As an observer traverses back in time through the repository's history, older commits are physically translated to recede toward the infinite boundary edge of the disk.

## Consequences

- **Positive:** Creates an emergent visual representation where recent, highly active files dominate the central Euclidean-like space, while deeply coupled ancient files are mathematically compressed into an infinitely dense ring at the horizon. This visually reinforces temporal recency vs. historical depth.
- **Negative:** Simulating and rendering large volumes of Git history points mapped via Mobius transformations to the boundary edge can be computationally expensive and may result in visual clutter at the horizon.
