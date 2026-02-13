# 29. Chimera Metazoa (Organelles & Tissues)

Date: 2025-05-20 (Simulated)

## Status

Accepted

## Context

Originally, the Chimera VM simulated a single organism (a "cell") executing a single genome on a shared grid. While this allowed for complex internal logic, it limited the simulation of cooperative behaviors, specialization, and swarm dynamics.

To model more complex biological phenomena—such as multicellularity, symbiosis, and division of labor—we needed a way to support multiple, semi-autonomous agents operating within the same environment.

Key requirements were:
1.  **Multiple Agents**: Ability to spawn independent execution contexts.
2.  **Specialization**: Agents should have distinct roles (e.g., energy harvesting, waste processing).
3.  **Coordination**: Agents must be able to group together into coherent structures ("Tissues").
4.  **Communication**: Agents within a structure must be able to signal each other efficiently.

## Decision

We implemented the **Chimera Metazoa** system, introducing two key concepts: `Organelle` and `Tissue`.

### 1. Organelles

An `Organelle` is a lightweight, independent execution context that resides on the Grid. It has its own:
*   **Instruction Pointer (IP)**: To execute code independently.
*   **Stack**: Local memory for computation.
*   **Location**: Coordinates on the Grid.
*   **Type**: Functional specialization.

Organelles are spawned via the `Spawn(Type)` OpCode. The VM's `step()` function now iterates through all active organelles, allowing them to act based on their type and/or execute their own DNA.

**Organelle Types:**
*   **Worker**: Standard execution unit. Executes code normally.
*   **Chloroplast**: Passive energy harvester. Gains energy from `light_grid`.
*   **Mitochondria**: Active energy generator. Reduces metabolic cost for the host.
*   **Lysosome**: Waste processor. Consumes `waste_grid` to produce energy.
*   **Ribosome**: Protein synthesizer. Specialized for grid manipulation (Read/Write/Exec).
*   **Alchemist**: Transmutes grid values based on neighbors (e.g., Fire + Water = Steam).
*   **Void**: Consumes matter (Grid values) and emits entropy.
*   **Seed**: Dormant form that can grow into a new plant structure.
*   **Wisp**: Ephemeral entity composed of flux.
*   **Choir**: Sonic agent that emits specific notes to trigger global effects.
*   **MadScientist**: Chaos agent that triggers random mutations and events.

### 2. Tissues

A `Tissue` is a logical grouping of Organelles that have bonded together.

*   **Bonding**: An organelle can execute `Bond(Direction)` to attach itself to a neighbor. This merges their Tissue IDs.
*   **Communication**: An organelle can execute `Signify(Value)` to broadcast a message to *all* other members of its Tissue, regardless of distance.
*   **Unbonding**: An organelle can leave a tissue via `Unbond()`.

This allows for the creation of "multicellular organisms" where different organelles perform specialized tasks (e.g., Chloroplasts feed the tissue, Workers build structures, Lysosomes clean up) while sharing a common identity and communication channel.

## Consequences

### Positive
*   **Emergent Complexity**: We can now simulate colonies, bacterial mats, and simple multicellular organisms.
*   **Division of Labor**: Programs can be simplified by offloading tasks to specialized organelles (e.g., "Spawn a Chloroplast" instead of "Run complex photosynthesis loop").
*   **Scalability**: The `Tissue` system allows for coordinated behavior without a central controller (brain), mimicking biological signaling.

### Negative
*   **Performance Cost**: The VM loop now iterates over `N` organelles per tick, increasing computational overhead linearly.
*   **Scheduling Complexity**: Determining the order of execution for organelles (sequential vs parallel) introduces nondeterminism. We chose sequential for simplicity.
*   **Resource Contention**: Organelles compete for the same Grid space and global Energy pool (currently shared by the host VM, though this might need to change to local energy).

## Compliance

This system integrates with the **Nova** feature set, specifically:
*   **Chemistry**: Alchemists interact with the chemical simulation.
*   **Symbiosis**: Organelles can be absorbed by the host via `Symbiosis` OpCode.
*   **Genetics**: Organelles inherit the host's DNA but execute it independently.
