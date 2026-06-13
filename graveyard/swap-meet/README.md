# Swap Meet 💾🏷️

> "The RAM Street Journal" - Genesis (The Economist)

A simulation of a memory allocator as a ruthless real estate market.

## 📊 Concept

Your computer's memory (Heap) is a grid of blocks.
**Agents** (processes) compete to acquire these blocks to run their code.
**Rent** is charged every tick.

- **Supply & Demand:** High-density areas become expensive (Rent Diffusion).
- **Fragmentation:** Agents prefer contiguous blocks (simulated by locality preference).
- **Eviction:** If an agent runs out of Wealth (CPU Cycles), they are kill-9'd (Evicted), freeing their blocks and crashing local prices.

## 🕹️ Controls

- **Q**: Quit
- **R**: Reset Simulation

## 🏗️ Architecture

- `model.rs`: Contains the `Market`, `Agent`, and `Block` logic.
- `main.rs`: The TUI frontend using `ratatui`.
- **Dynamics**:
  - **Income**: Agents earn random wealth per tick.
  - **Rent**: Paid per block owned.
  - **Inflation**: Occupied blocks increase in rent.
  - **Deflation**: Empty blocks decrease in rent.
  - **Diffusion**: High rent spreads to neighbors, creating "expensive neighborhoods".

## 🚀 Usage

```bash
cargo run -p swap-meet
```
