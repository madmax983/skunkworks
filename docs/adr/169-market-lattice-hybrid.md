# 169. Market-Lattice Hybrid

Date: 2026-08-10

## Status

Accepted

## Context

The Splice Surgeon created a new experimental hybrid called `market-lattice` (Architectural Financial Resistance). This cross explores mapping architectural constraints onto financial simulation by crossing the hierarchical file system structure (`crates/miller-lattice`) with the continuous trading dynamics (`crates/market-sim`).

## Decision

We inject discrete structural crystal nodes into a continuous market grid. The hierarchical structures act as physical constraints or "walls" within the market, forcing bids and asks to collide and navigate around the architecture.

## Consequences

- **Positive:** Provides an emergent visualization of how physical bounds and structure can constrain or channel financial pressure.
- **Negative:** Increased computational cost from collision detection between thousands of bids/asks against complex hierarchical crystal geometries.
