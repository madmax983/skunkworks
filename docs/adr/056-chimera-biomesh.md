# 56. Chimera BioMesh Network

Date: 2025-05-15

## Status

Accepted

## Context

The ChimeraVM operates on a 16x16 2D grid ("Petri Dish"), which simulates biological tissue. Communication in this environment is primarily local, relying on neighbor-to-neighbor interactions (e.g., `GRead`, `GWrite`) or diffusive signals (e.g., `Signal`, `Receive`).

However, more advanced distributed experiments—such as simulating a nervous system, a distributed hash table, or a Hive Mind—require efficient, targeted, long-distance communication. Using mobile agents (Organelles) to carry messages is slow and stochastic. Using signal diffusion floods the grid and lacks addressability.

We needed a mechanism to allow specific cells to establish direct, routed communication channels with one another, forming an overlay network on top of the physical substrate.

## Decision

We implemented the **BioMesh** system, a graph-based overlay network integrated into the Nova feature set.

### 1. Graph Overlay
The BioMesh allows any grid cell to register itself as a **Node** via the `MeshNet` OpCode. These nodes exist in a separate state layer (`vm.biomesh`) but are anchored to their grid coordinates `(y, x)`.

### 2. Topology Construction
Nodes do not automatically connect. The organism must actively build connections using:
*   `MeshGrow`: Connects the current node to adjacent nodes (Von Neumann neighborhood).
*   `MeshPrune`: Severs all connections from the current node.

This allows the organism to construct specific topologies (Line, Ring, Star, Mesh) programmatically.

### 3. Packet Switching
Communication is packet-based and routed:
*   `MeshSend(target_id, value)`: Uses Breadth-First Search (BFS) to find the shortest path through the connected graph to a node with `target_id`.
*   `MeshRecv()`: Retrieves messages from the node's local buffer.

### 4. Zero-Latency Routing
Packet delivery occurs instantly within the same tick. This simplifies synchronization logic for the organism, treating the mesh as a "fast path" nervous system compared to the "slow path" chemical diffusion.

## Consequences

### Positive
*   **Structured Communication:** Enables the creation of complex distributed systems within the VM without relying on chaotic diffusion.
*   **Addressability:** Messages are sent to specific IDs, allowing for directed control.
*   **Speed:** Instant routing simulates electrical impulses (nerves) vs chemical signals (hormones).

### Negative
*   **State Complexity:** The graph state must be maintained. If a cell containing a Node is overwritten by `GWrite`, the Node persists in the overlay unless explicitly pruned, potentially leading to "phantom" nodes if the visual representation diverges from the logical overlay.
*   **DoS Potential:** BFS routing on a highly connected graph can be computationally expensive. We mitigate this by charging Energy for network operations and limiting the recursion/queue depth.
*   **Ephemeral Volatility:** Since nodes are tied to coordinates, moving code (e.g. via `Transposon`) does not move the Node. The "hardware" (Mesh) is distinct from the "software" (DNA).
