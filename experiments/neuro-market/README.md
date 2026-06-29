# Neuro Market 🧠📈

> "The markets are driven by animal spirits, and not by reason." - John Maynard Keynes

**Concept**: Cognitive Market Dynamics.

**Novel Trait**: Crossing the biological Spiking Neural Network of `neuro-sim` with the Continuous Double Auction of `market-sim`. The electrical impulses (spikes) of individual neurons are translated directly into market activity (Bids and Asks), effectively turning the biological brain into a decentralized trading algorithm.

## Lineage Plan

*   **From `neuro-sim`**: The underlying Spiking Neural Network (SNN) based on Izhikevich neurons. We simulate a small brain of 10 neurons, randomly wired.
*   **From `market-sim`**: The 2D grid-based particle simulation representing a Continuous Double Auction order book.
*   **The Recombination**: When a neuron in the first half of the network spikes, it generates a "Bid" (buying pressure). When a neuron in the second half spikes, it generates an "Ask" (selling pressure). The chaotic, interconnected firings of the network create organic, unpredictable liquidity waves in the market.

## Installation & Execution

To run this hybrid experiment:

```bash
cargo run -p neuro-market
```

To run in headless mode (e.g., for CI):

```bash
cargo run -p neuro-market -- --headless
```
