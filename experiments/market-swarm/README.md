# Market Swarm 🐝📈

A hybrid experiment combining **Market Flow** and **Luminous Flock**.

Visualizes market dynamics as a swarm of boids. Traders are represented as autonomous agents (Boids) flocking around the price line.

## Lineage

- **Parent A**: `experiments/market-flow` (Market Simulation, Order Book, Heatmap)
- **Parent B**: `experiments/luminous-flock` (Boid Physics, Flocking, Synchronization)

## Features

- **Market Simulation**: A cellular automaton simulates Limit Order Book dynamics (Bids, Asks, Trades).
- **Swarm Intelligence**: Traders (Boids) flock towards the current price.
  - **Bulls (Green)**: Flock slightly above the price, anticipating growth.
  - **Bears (Red)**: Flock slightly below the price, anticipating decline.
- **Emergent Volatility**: The tightness of the flock represents market consensus. A scattered flock indicates uncertainty/volatility.
- **Audio Synthesis**: Generative audio based on price frequency and trade volume (inherited from `market-flow`).

## Controls

- `q`: Quit
- `r`: Reset Simulation

## The Splice Surgeon's Notes

"The market is not a machine, it is an organism. This experiment proves it. The traders do not calculate; they swarm. Greed and Fear are just cohesion and separation weights in the flocking algorithm."
