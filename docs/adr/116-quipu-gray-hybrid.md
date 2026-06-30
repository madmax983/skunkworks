# 116. Quipu-Gray Hybrid

Date: 2026-06-15

## Status

Accepted

## Context

The experimental `quipu-gray` hybrid explores "Knotted Morphogenesis". We crossed `crates/quipu` (representing discrete, hierarchical knots on cords) with `crates/gray-scott` (representing continuous reaction-diffusion chemical physics). The goal is to evaluate if structural abstractions can interact with continuous fields.

## Decision

We implemented a bidirectional feedback loop where the discrete structural knots of a Quipu cord act as continuous chemical catalysts within a 2D Gray-Scott reaction-diffusion grid. As the chemical concentrations fluctuate, they inversely influence the placement or characteristics of the knots.

## Consequences

- **Positive:** Proves the viability of crossing purely abstract, data-focused models (like Quipu) with cellular automata-driven environments, opening new avenues for complex adaptive systems.
- **Negative:** Managing the state synchronization between a discrete hierarchical list and a dense 2D float array adds overhead and complexity to the main loop.
