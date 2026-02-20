# Rössler Market ⚛️⛈️

> "The invisible hand is chaotic."

A visualization of economic indicators driven by the Rössler Attractor.

## The Concept

The **Rössler Attractor** is a chaotic system with a single non-linear term. In this experiment, it models a volatile market:
- **X Axis**: Market Price
- **Y Axis**: Volatility / Risk
- **Z Axis**: Trading Volume / Inflation

100,000 "traders" (particles) simulate the market sentiment distribution. The user acts as the Central Bank, adjusting key economic parameters.

## Controls

- **WASD**: Move Camera
- **Arrow Keys**: Rotate Camera
- **U / J**: Increase/Decrease Interest Rate (Parameter `a`)
- **I / K**: Increase/Decrease Money Supply (Parameter `b`)
- **O / L**: Increase/Decrease Reserve Requirement (Parameter `c`)
- **R**: Reset Market (Bailout)

## Parameters

- **a**: Bifurcation parameter. Low values = stability. High values = chaos.
- **b**: Driving force.
- **c**: Threshold.

## Tech Stack

- **macroquad**: Visualization and Windowing.
- **rayon**: Parallel physics updates for 100,000 particles.
- **soroban**: (Dependency) Reserved for future fixed-point ledger logic.
