# Heap Market 🏟️

**Lineage:** `market-sim` × `Resource Management`

A hybrid experiment combining a continuous double auction mechanism with memory allocation.

## 🧬 Concept

Processes (Agents) compete for memory pages in a real-time auction. The "Heap" is a finite resource, and allocation is determined by market forces rather than a deterministic allocator.

- **Market**: A 2D grid where `Bid` particles (Processes) and `Ask` particles (Current Owners/System) collide to execute trades.
- **Heap**: A 1D array of pages, owned by the highest bidder.
- **Economy**:
  - Agents earn credits over time.
  - Agents pay "Rent" or "Purchase Price" for memory.
  - Bankruptcy leads to eviction (Garbage Collection).

## 🎮 Controls

- **Visual Only**: Watch the market dynamics.
- **Space**: Pause/Resume.

## 🚀 Usage

```bash
cargo run -p heap-market
```
