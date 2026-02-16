# 36. Use Abelian Sandpile Model for Distributed Task Scheduling Visualization

Date: 2024-05-22

## Status

Accepted

## Context

We need a way to visualize distributed system load balancing, failure cascades (DDoS), and critical thresholds in a manner that is both visually engaging and theoretically grounded. Traditional resource monitoring (CPU/Memory graphs) often fails to capture the "tipping point" dynamics of distributed systems under heavy load.

The challenge is to create a simulation that models:
1.  **Task accumulation**: Requests arriving at a node.
2.  **Load shedding**: Nodes passing excess load to neighbors when overwhelmed.
3.  **Cascading failure**: A chain reaction of shedding load causing system-wide instability.

## Decision

We will implement the **Abelian Sandpile Model** (Bak, Tang, Wiesenfeld) on a 2D grid to simulate distributed task scheduling.

*   **Mapping**:
    *   `Grid Cell` = Server Node.
    *   `Sand Grain` = Task / Request.
    *   `Topple Threshold (4)` = Max Capacity before shedding load.
    *   `Avalanche` = Load Balancing Event / Cascade.

*   **Implementation Details**:
    *   The grid will be **double-buffered** (`cells` and `next_cells`) to allow for simultaneous updates without race conditions.
    *   Updates will be parallelized using **`rayon`** (`par_iter_mut`) to handle large grid dimensions (e.g., 512x512) at 60 FPS.
    *   Processing of tasks (grain removal) will be stochastic, while toppling (load distribution) will be deterministic.

## Consequences

### Positive
*   **Visual Intuition**: The model naturally visualizes "criticality," where the system self-organizes to a state where avalanches of all sizes can occur. This mirrors real-world distributed system behavior near capacity.
*   **Performance**: The double-buffered, parallel approach allows for high-resolution simulations.
*   **Simplicity**: The rules are local (if load >= 4, distribute to neighbors), yet global complexity emerges.

### Negative
*   **Memory Overhead**: Double buffering requires 2x the memory for the grid state.
*   **Topology Limitation**: A 2D grid is a simplified network topology compared to real-world graphs (e.g., small-world networks), though it suffices for visual abstraction.
*   **Determinism vs. Reality**: Real load balancers use complex algorithms (Round Robin, Least Connections), whereas this model uses pure overflow physics.
