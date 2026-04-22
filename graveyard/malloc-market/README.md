# Malloc Market 📉

> "The Heap is a battlefield." — Genesis (The Economist)

**Malloc Market** is a visual simulation of a memory allocator driven by a **Continuous Double Auction**.
Agents (Processes) compete for pages of memory (Grid Cells). The system (Kernel) charges rent based on total utilization.

## 🧬 Concept

*   **Grid**: The memory heap.
*   **Agents**: Processes that need memory.
*   **Price**: The cost per block per tick. Fluctuate based on supply/demand.
*   **Strategy**: Agents expand when wealthy and contract when poor.
*   **Bankruptcy**: If an agent cannot pay rent, it is OOM Killed. Its memory becomes fragmented.

## 🎮 Visuals

*   **Colors**: Each agent has a unique color.
*   **Gray Blocks**: Fragmented memory (recently freed, waiting for GC).
*   **Green Line**: Price history.
*   **Blue Line**: Utilization history.

## 🚀 Usage

```bash
cargo run -p malloc-market
```

Controls:
*   **R**: Reset simulation.
