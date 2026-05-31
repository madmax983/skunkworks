# market-resonance

A hybrid experiment crossing `market-sim` (Continuous Double Auction particle system) and `resonance-audio` (FDTD wave simulation).

## Concept: Acoustic Market Volatility
In standard `market-sim`, bid and ask particles interact to form trades. In this hybrid, every transaction acts as an acoustic impulse that strikes a continuous finite difference time domain (FDTD) wave grid provided by `resonance-audio`. The market acts as a reactor where high trading volume in specific price bands physically excites the acoustic space, creating standing waves and ripples of liquidity and resistance.

## Lineage
- **Parent A (market-sim)**: Provides the discrete bid/ask particle dynamics, collision logic, and price discovery.
- **Parent B (resonance-audio)**: Provides the continuous 2D FDTD simulation and audio processing pipeline.

## Novel Trait
Mapping discrete market trades directly to acoustic wave propagation.

## Setup & Running
```bash
cargo run -p market-resonance
cargo run -p market-resonance -- --headless
```
