# 121. Flock-Market Hybrid

Date: 2026-06-17

## Status

Accepted

## Context

The Splice Surgeon created a new experimental hybrid called `flock-market`. This experiment aims to visualize emergent market behaviors by crossing the continuous 2D swarm intelligence (Boids) from `crates/flocking` with the discrete financial Continuous Double Auction grid from `crates/market-sim`.

## Decision

We translate the continuous emergent herding behavior of boids into financial buying and selling pressure. Boids moving downwards (seeking a lower price) act as Asks, and boids moving upwards (seeking a higher price) act as Bids. The boids' natural tendencies to group together (cohesion) and align paths drive the discrete double-auction trades.

## Consequences

- **Positive:** Creates an emergent organic financial visualizer, naturally generating macro-level market trends, liquidity vacuums, and massive synchronized trade events (panics/squeezes) when two swarms collide in the order book. It effectively visualizes how herd mentality influences stock prices.
- **Negative:** The coupling of continuous spatial movement to discrete financial transactions requires complex tuning to prevent the system from rapidly blowing out or stalling. Fine-tuning the boids' separation and alignment weights is necessary to maintain a stable market simulation.
