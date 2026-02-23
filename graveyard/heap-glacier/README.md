# Heap Glacier ⚛️🪨

> "Memory is not just storage. It is a landscape, and allocations are the weather." — Genesis (The Geologist)

**Heap Glacier** is a geological simulation of memory allocation. It visualizes the heap as a mountain range, where allocations deposit snow that compacts into ice, and deallocations melt the ice into water, causing hydraulic erosion on the underlying bedrock.

## 🧬 Concept

*   **Bedrock**: The memory address space.
*   **Snow/Ice**: Allocated memory. Accumulates over time.
*   **Water**: Deallocated memory (meltwater).
*   **Erosion**: Rapid alloc/dealloc cycles (churn) create meltwater torrents that carve deep canyons into the bedrock.
*   **Glaciers**: Memory leaks (alloc without dealloc) form massive ice sheets that slowly flow and reshape the landscape.

## 🎮 Controls

*   **WASD + Arrows**: Move Camera & Look around.
*   **Space / Shift**: Fly Up / Down.
*   **1**: **Web Server Mode** (High Churn) - Random small allocs/deallocs. Watch the canyons form!
*   **2**: **Memory Leak Mode** (Ice Age) - Steady accumulation. Watch the glaciers grow!
*   **3**: **GC Mode** (Global Warming) - Massive deallocation events.
*   **0**: Stop auto-allocation.

## 🧪 The Science

The simulation uses a simplified hydraulic erosion model:
1.  **Precipitation**: `allocate()` adds height to the ice layer.
2.  **Melting**: `deallocate()` converts ice to water.
3.  **Flow**: Water flows downhill, carrying sediment (eroding bedrock).
4.  **Evaporation**: Water slowly disappears, leaving behind the scarred terrain.

## 📦 Usage

```bash
cargo run -p heap-glacier
```
