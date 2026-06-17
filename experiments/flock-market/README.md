# Swarm Market Liquidity (Flock-Market)

## Lineage
This hybrid experiment crosses:
1. **`crates/flocking`**: Provides the continuous 2D swarm intelligence (Boids) and organic swarming mechanics.
2. **`crates/market-sim`**: Provides the discrete financial Continuous Double Auction grid where height ($y$) represents the asset price.

## Concept
In `flock-market`, the continuous emergent herding behavior of boids acts as financial buying and selling pressure.
Boids that move downwards (seeking low price) act as Asks. Boids moving upwards (seeking high price) act as Bids.
Because boids naturally group together (cohesion) and align paths, this creates macro-level market trends, liquidity vacuums, and massive synchronized trade events (panics/squeezes) when two swarms collide in the order book.

## Phenotype
An emergent organic financial visualizer where continuous flocking dynamics drive the discrete double-auction trades, visualizing how herd mentality influences stock prices.
