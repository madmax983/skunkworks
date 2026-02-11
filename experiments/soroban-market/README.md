# Soroban Market ⚛️📉

A "High-Frequency Trading" simulation visualized through the ancient mechanics of the **Japanese Soroban (Abacus)**.

> "Markets are just beads moving on a string. Speed is an illusion." - Genesis

## Concept

This experiment combines **Abacus Algorithms** with **HFT Simulation**.
Instead of floating-point numbers and digital displays, the entire market state—Bids, Asks, Last Price, and Volume—is represented by Sorobans.
The "Matching Engine" calculates trade execution using bead arithmetic (complementary numbers, carries, borrows), visualizing the physical operations required to settle trades.

## Features

- **Ancient Arithmetic Engine**: A fully functional Soroban implementation that performs addition and subtraction by generating a sequence of physical bead movements (e.g., "Heaven Down", "Earth Up").
- **Visual Market**: Four real-time Abacuses displaying the market state.
- **HFT Simulation**: Random order generation and trade execution that drives the abacuses.
- **Animation**: Watch as the volume accumulates bead by bead, carrying over columns with satisfying mechanical logic.

## Controls

- Just watch the market flow.
- The **Volume** abacus (bottom right) shows the most activity as trades occur.

## arithmetic Logic

The `Soroban` struct implements traditional algorithms:
- **Addition**:
  - `val < 5`: Move Earth beads up. If not enough, move Heaven down (+5) and Earth down (-complement).
  - `val >= 5`: Move Heaven down (+5), then add remainder.
  - **Carry**: If sum >= 10, subtract complement from current column and add 1 to next.
- **Subtraction**:
  - **Borrow**: If `val > current`, borrow 1 from next column (-10), add complement to current.

## Running

```bash
cargo run -p soroban-market
```
